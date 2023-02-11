use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &st.ident;
    let builder_name = Ident::new(&format!("{name}Builder"), Span::call_site());

    let ident_and_ty = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = st.data
    {
        named
            .iter()
            .map(
                |syn::Field {
                     ref ident, ref ty, ..
                 }| {
                    // println!("{:?} {:?}", ident, ty);
                    (ident, ty)
                },
            )
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    let mut builder_fields_def = proc_macro2::TokenStream::new();
    let mut builder_fns = proc_macro2::TokenStream::new();
    let mut builder_default_fields = proc_macro2::TokenStream::new();
    let mut builder_main_struct = proc_macro2::TokenStream::new();
    

    ident_and_ty.iter().for_each(|&(ident, ty)| {
        if ident.is_none() {
            return;
        }
        let ty_wrapped_with_option = if let syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) = &ty
        {
            // println!("sg: {:?}", segments);
            // println!("{:?}", segments.last().as_ref().unwrap().ident);
            match segments.last().as_ref().unwrap().ident.to_string().as_str() {
                // FIXME: 从 Option 中取出类型
                "Option" => {
                    if let syn::PathSegment {
                        arguments:
                            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                                args: z,
                                ..
                            }),
                        ..
                    } = segments.last().as_ref().unwrap()
                    {
                        if let Some(syn::GenericArgument::Type(inner_type)) = z.last() {
                            // println!("\n\nwtf inner_type: {:?}\n\n", inner_type);
                            Some(inner_type)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => {
                    // println!("find other type: {:?}", &ty);
                    None
                }
            }
        } else {
            None
        };
        let ty_ = *ty_wrapped_with_option.as_ref().unwrap_or(&ty);
        if ty_wrapped_with_option.as_ref().is_some() {
            // struct 的字段是 Option 的
            builder_main_struct.extend(
                quote!(
                    #ident: self.#ident.clone(),
                )
            );
        } else {
            builder_main_struct.extend(
                quote!(
                    #ident: self.#ident.as_ref().unwrap().clone(),
                )
            );
        };
        builder_fields_def.extend(quote! {
            // FIXME: 如果 ty 就是 Option<T> 这里会出现 Option<Option<T>>
            #ident: std::option::Option<#ty_>,
        });
        builder_default_fields.extend(quote! {
            #ident: None,
        });
        builder_fns.extend(
            // FIXME: 如果 ty 就是 Option<T> 这里会出现 Option<Option<T>>
            quote! {
                pub fn #ident(&mut self, #ident: #ty_) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            },
        );
    });

    TokenStream::from(quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #builder_default_fields
                }
            }
        }

        pub struct #builder_name {
            #builder_fields_def
        }
        impl #builder_name {
            #builder_fns
            pub fn build(&self) -> std::result::Result<#name, String> {
                return Ok(
                    #name {
                        #builder_main_struct
                    }
                );
            }
        }
    })
}
