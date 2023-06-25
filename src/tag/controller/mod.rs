use axum::Router;

use crate::GlobalState;

use self::{
    attach::attach, create::create_tag, delete::delete_tag, get::get_tags, update::update_tag,
};

pub mod attach;
pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn tag_route() -> Router<GlobalState> {
    Router::new().nest(
        "/tag",
        Router::new()
            .merge(get_tags())
            .merge(create_tag())
            .merge(update_tag())
            .merge(delete_tag())
            .merge(attach()),
    )
}
