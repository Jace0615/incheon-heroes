use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::services::user_service::UserService;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub user_service: UserService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            user_service: use_context(),
        };

        Ok(ctrl)
    }
}
