use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{ItemFn, ItemStruct};

#[proc_macro_attribute]
pub fn api(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);

    let mut router_in: Vec<TokenStream2> = vec![];

    input.sig.inputs.iter().for_each(|arg| {
        if let syn::FnArg::Typed(pat) = arg {
            let tokens = pat.ty.to_token_stream();
            let name = if let syn::Pat::Ident(pat_ident) = &*pat.pat { pat_ident.ident.to_string() } else { String::new() };
            router_in.push(quote! {
                .comb::<#tokens>(#name)
            })
        }
    });

    let mut router_out: Vec<TokenStream2> = vec![];

    if let syn::ReturnType::Type(_, ty) = &input.sig.output {
        router_out.push(quote! {
            .comb::<#ty>("")
        });
    }

    quote! {
        // #[meta(disable_auto_json = true)]
        #[meta(
            nidrs::openapi::RouterIn(
                nidrs::openapi::RouterParams::default()
                #(#router_in)*,
            ),
            nidrs::openapi::RouterOut(
                nidrs::openapi::RouterParams::default()
                #(#router_out)*,
            )
        )]
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn schema(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemStruct);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident = &input.ident;

    quote! {
        #[derive(nidrs::openapi::utoipa::IntoParams, nidrs::openapi::utoipa::ToSchema)]
        #input

        impl #impl_generics nidrs::openapi::ToParamDto for #ident #ty_generics #where_clause {
            fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                use nidrs::openapi::utoipa::IntoParams;
                use nidrs::openapi::utoipa::ToSchema;
                use nidrs::openapi::utoipa::openapi::Schema;
                use nidrs::openapi::utoipa::openapi::RefOr;
                use nidrs::openapi::utoipa;

                let ref_schema: RefOr<Schema> = utoipa::schema!(Self).into();
                let mut schemas: Vec<(String, RefOr<Schema>)> = vec![
                    (
                        <Self as utoipa::ToSchema>::name().to_string(),
                        utoipa::schema!(#[inline] Self).into(),
                    )
                ];

                <Self as utoipa::ToSchema>::schemas(&mut schemas);

                match dto_type {
                    nidrs::openapi::ParamDtoIn::Param(p) => nidrs::openapi::ParamDto::ParamList(Self::into_params(|| Some(p.clone()))),
                    nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema((
                        ref_schema,
                        schemas,
                    )),
                    _ => nidrs::openapi::ParamDto::None,
                }
            }
        }
    }
    .into()
}
//                     utoipa::schema!()
