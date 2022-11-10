use std::time::Duration;

use chrono::{DateTime, Utc};
use scc::HashMap;
use uuid::Uuid;

use crate::r#type::Type;
use crate::Object;

#[derive(Debug)]
pub enum Value {
  Array(Vec<Value>),
  Bool(bool),
  DateTime(DateTime<Utc>),
  Duration(Duration),
  Int8(i8),
  Int16(i16),
  Int32(i32),
  Int64(i64),
  Float32(f32),
  Float64(f64),
  Map(HashMap<String, Value>),
  Object(Object),
  String(String),
  Uint8(u8),
  Uint16(u16),
  Uint32(u32),
  Uint64(u64),
  Uint128(u128),
  Uuid(Uuid),
}

impl Value {
  pub fn ty(&self) -> Type {
    use Value::*;

    match self {
      Array(_) => Type::Array,
      Bool(_) => Type::Bool,
      DateTime(_) => Type::DateTime,
      Duration(_) => Type::Duration,
      Int8(_) => Type::Int8,
      Int16(_) => Type::Int16,
      Int32(_) => Type::Int32,
      Int64(_) => Type::Int64,
      Float32(_) => Type::Float32,
      Float64(_) => Type::Float64,
      Map(_) => Type::Map,
      Object(_) => Type::Object,
      String(_) => Type::String,
      Uint8(_) => Type::Uint8,
      Uint16(_) => Type::Uint16,
      Uint32(_) => Type::Uint32,
      Uint64(_) => Type::Uint64,
      Uint128(_) => Type::Uint128,
      Uuid(_) => Type::Uuid,
    }
  }
}
