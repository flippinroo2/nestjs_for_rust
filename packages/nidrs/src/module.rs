use nidrs_extern::axum;
use nidrs_extern::tokio;
use std::{any::Any, collections::HashMap};

use crate::{provider, AppResult, Service};

pub trait Module {
  fn init(self, ctx: ModuleCtx)->ModuleCtx;
}

pub struct DynamicModule {
  pub services: HashMap<&'static str, Box<dyn Any>>,
}

impl DynamicModule {
  pub fn new() -> Self {
    DynamicModule {
        services: HashMap::new(),
    }
  }

  pub fn service(mut self, service: (&'static str, Box<dyn Any>)) -> Self {
    self.services.insert(service.0, service.1);
    self
  }

  pub fn provider<T:Service + 'static>(mut self, service:T) -> Self {
    let (name, service) = provider(service);
    self.services.insert(name, service);
    self
  }
}

pub struct NidrsFactory {
  pub router: axum::Router<StateCtx>,
  pub default_prefix: &'static str,
}

impl NidrsFactory {
  pub fn create<T: Module>(
      module: T,
  ) -> Self {
      let router = axum::Router::new().route("/", axum::routing::get(|| async move {
          "Hello, Nidrs!"
      }));
      let module_ctx = ModuleCtx::new();
      let module_ctx = module.init(module_ctx);
      let routers = module_ctx.routers;
      let mut sub_router = axum::Router::new();
      for router in routers.iter() {
          sub_router = sub_router.merge(router.clone());
      }
      NidrsFactory {
          router: router.merge(sub_router),
          default_prefix: "",
      }
  }

  pub fn default_prefix(mut self, prefix: &'static str) -> Self {
    self.default_prefix = prefix;
    self
  }

  pub fn listen(self, port: u32) {
    let server = || async {
      let tcp = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
      let addr = tcp.local_addr()?;
      nidrs_macro::log!("Listening on {}", addr);
      
      axum::serve(tcp, self.router.with_state(StateCtx{})).await?;

      AppResult::Ok(())
    };
    
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(server());
  }
}

#[derive(Debug, Clone)]
pub struct StateCtx{
}

pub struct ModuleCtx{
  pub modules:HashMap<String, Box<dyn Any>>,
  pub services: HashMap<String, Box<dyn Any>>,
  pub controllers: HashMap<String, Box<dyn Any>>,
  pub routers: Vec<axum::Router<StateCtx>>,
  pub interceptors: HashMap<String, Box<dyn Any>>,
}

impl ModuleCtx {
  pub fn new() -> Self {
      ModuleCtx {
          modules: HashMap::new(),
          services: HashMap::new(),
          controllers: HashMap::new(),
          routers: Vec::new(),
          interceptors: HashMap::new(),
      }
  }
}
