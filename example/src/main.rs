use std::sync::Arc;

mod app;

fn main() {
    println!("Hello, world!");

    let app_state = AppState{};

    nestrs::NestFactory::create(app::AppModule, app_state).listen::<AppError>(3000);
}



#[derive(Clone, Debug, Default)]
pub struct AppState{}

pub enum AppError {
    
}


impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        // Convert std::io::Error to AppError here
        // Example: AppError::new(error.to_string())
        unimplemented!()
    }
}