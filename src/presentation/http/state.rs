use std::sync::{ Arc };

use crate::providers::{ AppState };

#[derive(Clone)]
pub struct HttpState {
    pub app: Arc<AppState>
}

impl HttpState {
    pub fn new(app: Arc<AppState>) -> Self {
        Self { app }
    }
}
