use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    // pub enum Data<V, F> { Enum(Vec<V>),Struct(Fields<F>), }
    data: Data<(), AutoDerefFieldsInfo>,
    #[darling(default)]
    mutable: bool,
    // 字段名，default可以缺省
    #[darling(default)]
    field: Option<syn::Ident>,
}

// 要提取结构体字段的哪些信息
#[derive(Debug, FromField)]
struct AutoDerefFieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
    // destruct结构input，获取属性和字段信息
    let AutoDerefInfo {
        ident,
        generics,
        // input的数据类型是结构体，且给出了要获取结构体中的哪些字段信息
        data: Data::Struct(fields),
        mutable,
        // 要deref出去的结构体字段名
        field,
    } = AutoDerefInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDeref only works on structs");
    };
    // 当#[deref(field = "inner")],AutoDeref的结构体里面有inner这个字段时...
    let (fd, ty) = if let Some(field) = field {
        // 如果fields中的元素f.ident==field，则返回这个字段的ident和ty
        match fields.iter().find(|f| f.ident.as_ref().unwrap() == &field) {
            Some(f) => (field, &f.ty),
            None => panic!("field {:?} not found in the data structure", field),
        }
    } else {
        // 当#[deref(field = "inner")],AutoDeref的结构体里面 没有 inner这个字段时...
        // if only 1 field, use that field
        if fields.len() == 1 {
            let f = fields.iter().next().unwrap();
            (f.ident.as_ref().unwrap().clone(), &f.ty)
        } else {
            panic!("AutoDeref only works on structs with 1 field or with field attribute");
        }
    };

    let mut code = vec![quote! {
        impl #generics std::ops::Deref for #ident #generics {
            type Target = #ty;

            fn deref(&self) -> &Self::Target {
                &self.#fd
            }
        }
    }];

    if mutable {
        code.push(quote! {
            impl #generics std::ops::DerefMut for #ident #generics {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#fd
                }
            }
        });
    }

    quote! {
        #(#code)*
    }
}
