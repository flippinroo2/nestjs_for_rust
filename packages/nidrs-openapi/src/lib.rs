use std::collections::HashMap;

use nidrs_extern::{
    router::{MetaRouter, StateCtx},
    shared::convert_path_to_openapi,
};
pub use utoipa;
use utoipa::openapi::{
    extensions::ExtensionsBuilder,
    path::{OperationBuilder, PathItemBuilder},
    request_body::RequestBodyBuilder,
    security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    tag::TagBuilder,
    ComponentsBuilder, Info, OpenApiBuilder, PathsBuilder, SecurityRequirement,
};
pub use utoipa_rapidoc;
use utoipa_rapidoc::RapiDoc;
pub use utoipa_redoc;
use utoipa_redoc::{Redoc, Servable};
pub use utoipa_scalar;
pub use utoipa_swagger_ui;
use utoipa_swagger_ui::SwaggerUi;

mod datasets;
pub use datasets::*;

pub use nidrs_openapi_macro::*;

pub mod union_type;

pub fn register(routers: &Vec<MetaRouter>) -> axum::Router<StateCtx> {
    // OPENAPI IMPLEMENTATION
    let mut paths = PathsBuilder::new().build();
    let mut components = ComponentsBuilder::new();
    let mut tags = HashMap::new();

    for router in routers.iter() {
        let path = router.meta.get_data::<nidrs_extern::datasets::RouterFullPath>().unwrap().value();
        let method = router.meta.get_data::<nidrs_extern::datasets::RouterMethod>().unwrap().value();
        let router_name = router.meta.get_data::<nidrs_extern::datasets::RouterName>().unwrap().value();
        let controller_name = router.meta.get_data::<nidrs_extern::datasets::ServiceName>().unwrap().value();
        // println!("path: {}, method: {}, body: {:?}", path, method, router.meta.get_data::<datasets::RouterIn>());
        let tag_name = controller_name.to_string();
        tags.insert(tag_name.clone(), 1);
        let path_type = match method.as_str() {
            "post" => utoipa::openapi::HttpMethod::Post,
            "put" => utoipa::openapi::HttpMethod::Put,
            "delete" => utoipa::openapi::HttpMethod::Delete,
            "patch" => utoipa::openapi::HttpMethod::Patch,
            "options" => utoipa::openapi::HttpMethod::Options,
            "head" => utoipa::openapi::HttpMethod::Head,
            "trace" => utoipa::openapi::HttpMethod::Trace,
            _ => utoipa::openapi::HttpMethod::Get,
        };

        let opath = convert_path_to_openapi(path);
        if paths.paths.get(&opath).is_none() {
            let path_item = PathItemBuilder::new().build();
            paths.paths.insert(opath.clone(), path_item);
        }

        if let Some(path_item) = paths.paths.get_mut(&opath) {
            let mut operation = OperationBuilder::new();
            let router_in = router.meta.get_data::<datasets::RouterIn>();
            let router_out = router.meta.get_data::<datasets::RouterOut>();
            let router_security = router.meta.get_data::<datasets::RouterSecurity>();
            // println!("router_in: {:?}, router_out: {:?}", router_in, router_out);
            if let Some(RouterSecurity(router_security)) = router_security {
                for security in router_security.to_owned() {
                    match security.as_str() {
                        "$bearer" => {
                            components = components.security_scheme(
                                "$bearer",
                                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).bearer_format("JWT").build()),
                            );
                        }
                        security_str => {
                            components =
                                components.security_scheme(security_str, SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(security_str))));
                        }
                    }
                    operation = operation.security(SecurityRequirement::new(security, Vec::<String>::new()));
                }
            }
            if let Some(router_in) = router_in {
                for param in router_in.value().value() {
                    match param {
                        datasets::ParamType::Param(p) => {
                            operation = operation.parameter(p.to_owned());
                        }
                        datasets::ParamType::Body(body) => {
                            if let Some(schema) = &body.schema {
                                // components.schemas.insert(schema.0.to_string(), schema.1.to_owned());
                                operation = operation.request_body(Some(
                                    RequestBodyBuilder::new()
                                        .content(body.content_type, utoipa::openapi::ContentBuilder::new().schema(Some(schema.to_owned().0)).build())
                                        .build(),
                                ));
                                for schema in schema.1.to_owned() {
                                    components = components.schema(schema.0.to_string(), schema.1.to_owned());
                                }
                            }
                        }
                    }
                }
            }
            if let Some(router_out) = router_out {
                for param in router_out.value().value() {
                    if let datasets::ParamType::Body(body) = param {
                        let mut content = utoipa::openapi::ContentBuilder::new();
                        if let Some(schema) = &body.schema {
                            content = content.schema(Some(schema.0.to_owned()));
                            for schema in schema.1.to_owned() {
                                components = components.schema(schema.0.to_string(), schema.1.to_owned());
                            }
                        } else {
                            content = content.example(Some(serde_json::Value::String("String".to_string())));
                        }
                        let response = utoipa::openapi::ResponseBuilder::new().content(body.content_type, content.build()).build();
                        operation = operation.response("200", response);
                    }
                }
            }

            path_item.merge_operations(
                PathItemBuilder::new()
                    .operation(
                        path_type,
                        operation
                            .tag(tag_name)
                            .extensions(Some(
                                ExtensionsBuilder::new()
                                    .add("x-controller", serde_json::Value::String(controller_name.clone()))
                                    .add("x-router", serde_json::Value::String(router_name.clone()))
                                    .build(),
                            ))
                            .description(Some(format!("{}::{}", controller_name, router_name)))
                            .build(),
                    )
                    .build(),
            );
        }
    }

    let api = OpenApiBuilder::new()
        .info(Info::new("Nidrs OpenAPI", "v1.0"))
        .paths(paths)
        .components(Some(components.build()))
        .tags(Some(tags.keys().map(|name| TagBuilder::new().name(name).description(Some(format!("Tag for {}", name))).build()).collect::<Vec<_>>()))
        // .security(
        //     Some([
        //         SecurityRequirement::new("bearer", Vec::<String>::new())
        //     ])
        // )
        .build();

    axum::Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
}
