use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
pub fn process_enum_from(input: DeriveInput) -> TokenStream {
    // println!("{:#?}", input);
    // 获取要处理的枚举类型的具体名称 enum Direction 这个Direction
    let ident = input.ident;
    // 获取枚举类型的所有变体，[Up(DirectionUp), Down, Left(u32), Right { a: u32 }]
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    // 泛型参数
    let generics = input.generics;
    // println!("{:#?}", variants);
    // 匹配枚举类型的所有变体，为不同的变体生成不同的实现
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            // 为Unnamed未命名字段的类型，如 Up(DirectionUp)，Left(u32), 生成实现代码
            syn::Fields::Unnamed(fields) => {
                // only support one field，只处理一个未命名字段的变体
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    // 获取未命名字段
                    let field = fields.unnamed.first().expect("should have 1 field");
                    // 获取未命名字段的类型
                    let ty = &field.ty;
                    // 生成代码
                    quote! {
                        // 添加泛型支持
                        impl #generics From<#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            // 为Unit无字段 类型生成实现，如 Direction::Down
            syn::Fields::Unit => quote! {},
            // 为Named命名字段 枚举类型生成实现，如Right { a: u32 }
            syn::Fields::Named(_fields) => quote! {},
        }
    });

    // quote return proce-macro2 TokenTtream so we need to convert it to TokenStream
    // quote!宏重新生成代码，将之前生成的所有 From trait 实现代码组合在一起。
    // #(#from_impls)* 表示展开 from_impls 中的所有代码片段。
    // into()将生成的代码片段转换为 TokenStream 类型，以便作为过程宏的输出返回。
    quote! {
        #(#from_impls)*
    }
}
