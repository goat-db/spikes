use std::collections::BTreeMap;

pub use crate::archetypes::*;

use crate::value::Value;

mod archetypes;
mod r#type;
mod value;

pub type Object = BTreeMap<String, Value>;
