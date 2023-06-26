use axum::Router;

use crate::GlobalState;

use self::{create::create_tag, delete::delete_tag, get::get_tags, set::set, update::update_tag};

pub mod create;
pub mod delete;
pub mod get;
pub mod set;
pub mod update;

pub fn tag_route() -> Router<GlobalState> {
    Router::new().nest(
        "/tag",
        Router::new()
            .merge(get_tags())
            .merge(create_tag())
            .merge(update_tag())
            .merge(delete_tag())
            .merge(set()),
    )
}
