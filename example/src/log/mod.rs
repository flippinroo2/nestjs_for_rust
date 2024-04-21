use nidrs_macro::module;

pub mod interceptor;
pub mod service;

use interceptor::LogInterceptor;
use service::LogService;

#[module({
  services = [LogService];
  exports = [LogService];
})]
#[derive(Clone, Debug, Default)]
pub struct LogModule;
