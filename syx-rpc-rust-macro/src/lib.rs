extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashMap;

use quote::quote;
use syn::{FnArg, ImplItem, ItemImpl, ItemTrait, parse_macro_input, ReturnType, TraitItem};

#[proc_macro_attribute]
pub fn syx_provider(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let struct_name = if let syn::Type::Path(type_path) = *input.self_ty {
        type_path.path.segments.last().unwrap().ident.clone()
    } else {
        panic!("syx_provider can only be applied to impl blocks for structs.");
    };

    // 获取trait名称
    let trait_name = if let Some((_, path, _)) = input.trait_ {
        path.segments.last().unwrap().ident.clone()
    } else {
        panic!("syx_provider must be applied to impl blocks for a trait.");
    };

    // 生成每个方法的实现，仅当方法有确切的返回类型时进行
    let method_impls = input.items.iter().filter_map(|item| {
        if let ImplItem::Fn(method) = item {
            let method_block = &method.block;
            let params = &method.sig.inputs;
            let method_name = &method.sig.ident;

            let return_type = if let ReturnType::Type(_, ty) = &method.sig.output {
                quote!(#ty)
            } else {
                quote!()
            };
            Some(quote! {
                async fn #method_name(#params) -> #return_type
                   #method_block
            })
        } else {
            None
        }
    });

    // 动态生成原始的trait实现
    let original_impl = quote! {
        impl #trait_name for #struct_name {
            #(#method_impls)*
        }
    };

    let methods = input.items.iter().filter_map(|item| {
        if let ImplItem::Fn(method) = item {
            let ident = &method.sig.ident;
            let method_name = ident.to_string();
            // 解析每一个入参（json反序列化）
            let mut req_pat = vec![];
            let req = method.sig.inputs.iter().fold(vec![], |mut vec, e| {
                if let FnArg::Typed(input) = e {
                    let req = &input.pat;
                    let req_type = &input.ty;
                    let token = quote! {
                    let #req : #req_type = serde_json::from_str(&args[idx].clone()).unwrap();
                    idx += 1;
                    };
                    req_pat.push(req);
                    vec.push(token);
                }
                vec
            },
            );
            // 构建方法跳转
            Some(quote! {
                if method_sign == #method_name {
                    let mut idx = 0;
                    #(
                        #req
                    )*

                    let re = self.#ident(
                        #(
                            #req_pat,
                        )*
                    ).await;
                    return serde_json::to_string(&re).unwrap();
                }
            })
        } else {
            None
        }
    });

    let gen = quote! {
        // 原始方法实现
        #original_impl

        // 调用原始方法入口
        #[async_trait::async_trait]
        impl RpcService for #struct_name {
            async fn invoke(&self, method_sign: &str, args: Vec<String>) -> String {
                #(#methods)*
                panic!("not find method")
            }
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn syx_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    // 解析参数
    let mut map = HashMap::new();
    let attr = attr.clone().to_string();
    let args: Vec<&str> = attr.split(",").collect();
    for arg in args {
        let arg = arg.replace(" ", "");
        let item: Vec<&str> = arg.split("=").collect();
        map.insert(
            item[0].to_string().clone(),
            item[1].replace("\"", "").to_string().clone(),
        );
    }
    let provider = map.get("provider").map_or("unknown", |e| e);

    let item_trait = get_item_trait(input.clone());
    let trait_ident = &input.ident;
    let items = &input.items;
    // 获取所有trait的方法说明
    let mut sig_item = vec![];
    for item in items {
        if let TraitItem::Fn(item) = item {
            sig_item.push(item.sig.clone());
        }
    }

    // 对每个方法生成rpc client
    let mut fn_quote = vec![];
    for item in sig_item {
        let asyncable = item.asyncness;
        let ident = item.ident;
        let inputs = item.inputs;
        let req = inputs.iter().fold(vec![], |mut vec, e| {
            if let FnArg::Typed(req) = e {
                vec.push(req.pat.clone());
            }
            vec
        });
        let return_type = if let ReturnType::Type(_, ty) = item.output {
            quote!(#ty)
        } else {
            quote!()
        };
        let proxy_fn = quote! {
                pub #asyncable fn #ident (#inputs) -> #return_type {
                    let mut req_vec : Vec<String> = vec![];
                    #(
                        let mut res_str = serde_json::to_string(&#req);
                        req_vec.push(res_str.unwrap());
                    )*

                    let request = syx_rpc_rust_core::RpcRequest {
                        service: #provider.to_string(),
                        method_sign: stringify!(#ident).to_string(),
                        args: req_vec,
                    };
                    // 远端结果
                    let res :String = syx_rpc_rust_core::invoke_provider(&request, self.config).await;
                    // 反序列化为response
                    let response: syx_rpc_rust_core::RpcResponse = serde_json::from_str(&res).unwrap();
                    // 反序列化为对应的data
                    let data :#return_type = serde_json::from_str(&response.data).unwrap();
                    return data;
                }
            };
        fn_quote.push(proxy_fn);
    }
    let rpc_client = syn::Ident::new(&format!("{}Rpc", trait_ident), trait_ident.span());
    let expanded = quote! {
        #item_trait

        pub struct #rpc_client {
            config: &'static syx_rpc_rust_core::RpcConsumerClientConfig,
        }

        impl #rpc_client {
             pub fn new(config : &'static syx_rpc_rust_core::RpcConsumerClientConfig) -> #rpc_client {
                #rpc_client {config}
            }

            #(
                #fn_quote
            )*
       }

    };
    TokenStream::from(expanded)
}

fn get_item_trait(item: ItemTrait) -> proc_macro2::TokenStream {
    let trait_ident = &item.ident;
    let item_fn = item.items.iter().fold(vec![], |mut vec, e| {
        if let TraitItem::Fn(item_fn) = e {
            let asyncable = &item_fn.sig.asyncness;
            let ident = &item_fn.sig.ident;
            let inputs = &item_fn.sig.inputs;

            let return_type = if let ReturnType::Type(_, ty) = &item_fn.sig.output {
                quote!(#ty)
            } else {
                quote!()
            };

            vec.push(quote!(
               #asyncable fn #ident (#inputs) -> #return_type;
            ));
        }
        vec
    },
    );
    quote! {
        #[allow(async_fn_in_trait)]
        pub trait #trait_ident {
           #(
               #item_fn
            )*
        }
    }
}