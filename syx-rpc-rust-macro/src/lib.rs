extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, ImplItem, ReturnType};

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
        if let ImplItem::Method(method) = item {
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
        if let ImplItem::Method(method) = item {
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