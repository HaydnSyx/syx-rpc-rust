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
                fn #method_name(#params) -> #return_type
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
            Some(quote! {
                if method_sign == #method_name {
                    return self.#ident(args);
                }
            })
        } else {
            None
        }
    });

    let gen = quote! {
        #original_impl

        impl RpcService for #struct_name {
            fn invoke(&self, method_sign: &str, args: &str) -> String {
                #(#methods)*
                panic!("not find method")
            }
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn rpc_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
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
    println!("item_trait: {}", item_trait.to_string());
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
        let ident = item.ident;
        let inputs = item.inputs;
        /*let req = inputs.iter().map(|e| {
            if let FnArg::Typed(req) = e {
                req.pat.clone()
            } else {
                panic!("rpc_trait only support typed fn arg")
            }
        });*/
        println!("generate method, ident: {}", ident.to_string());
        let proxy_fn = quote! {
                pub fn #ident (#inputs) -> String {
                    let request = syx_rpc_rust_core::RpcRequest {
                        service: #provider,
                        method_sign: stringify!(#ident).to_string(),
                        args: "syx",
                    };

                    let res :String = syx_rpc_rust_core::invoke_provider(&request).await;
                    return res;
                }
            };
        println!("proxy_fn: {}", proxy_fn.to_string());
        fn_quote.push(proxy_fn);
    }
    let rpc_client = syn::Ident::new(&format!("{}Rpc", trait_ident), trait_ident.span());
    let expanded = quote! {
        #item_trait

        pub struct #rpc_client {
        }
        impl #rpc_client {
        #(
            #fn_quote
        )*
       }

    };
    println!("expanded: {}", expanded.to_string());
    TokenStream::from(expanded)
}

fn get_item_trait(item: ItemTrait) -> proc_macro2::TokenStream {
    let trait_ident = &item.ident;
    let item_fn = item.items.iter().fold(vec![], |mut vec, e| {
        if let TraitItem::Fn(item_fn) = e {
            let asyncable = &item_fn.sig.asyncness;
            let ident = &item_fn.sig.ident;
            let inputs = &item_fn.sig.inputs;
            vec.push(quote!(
               #asyncable fn #ident (#inputs) -> String;
            ));
        }
        vec
    },
    );
    quote! {
        pub trait #trait_ident {
           #(
               #item_fn
            )*
        }
    }
}