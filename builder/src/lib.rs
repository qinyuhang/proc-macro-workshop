use proc_macro::TokenStream;
use syn;
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // let _ = input;
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &st.ident;
    let data = match st.data {
        syn::Data::Struct(d) => {
            match &d.fields {
                syn::Fields::Named(f_name) => {
                    f_name.named.iter().for_each(|val| {
                        println!("fileds: {:?}", val.ident.as_ref().unwrap().span());
                    });
                },
                _ => {}
            };
            // println!("fields: {:?}", d.fields);
            Some(d)
        },
        _ => None,
    };
    // let syn::Data::Struct(data) = &st.data;
    // println!("name: {}, data: {:?}", &name, &data);
    TokenStream::from( quote! {
        impl #name {
            pub fn builder() -> MyBuilder {
                MyBuilder {
                    executable: "".into(),
                    args: vec![],
                    env: vec![],
                    current_dir: "".into(),
                }
            }
        }
        pub struct MyBuilder {
            executable: String,
            args: Vec<String>,
            env: Vec<String>,
            current_dir: String,
        }
        impl MyBuilder {
            pub fn executable(&mut self, exe: String) -> &mut Self {
                self.executable = exe;
                self
            }
            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = env;
                self
            }
            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = args;
                self
            }
            pub fn current_dir(&mut self, dir: String) -> &mut Self {
                self.current_dir = dir;
                self
            }
            pub fn build(&self) -> Result<#name, String> {
                Ok(#name {
                    executable: self.executable.clone(),
                    args: self.args.clone(),
                    env: self.env.clone(),
                    current_dir: self.current_dir.clone(),
                })
            }
        }
    } )

}
