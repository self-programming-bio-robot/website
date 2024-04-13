use serde::{Deserialize, Serialize};
use crate::dto::actions::redirect::RedirectAction;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Redirect(RedirectAction),
}