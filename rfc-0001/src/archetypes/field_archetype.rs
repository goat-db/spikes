use std::hash::{Hash, Hasher};
use std::ops::Deref;

use fxhash::FxHasher64;

use crate::r#type::Type;

#[derive(Clone, Copy, Debug, Default, Eq, PartialOrd, Ord)]
pub struct FieldArchetypeId(u64);

impl FieldArchetypeId {
  pub fn new(name: &str, ty: Type) -> Self {
    let mut hasher = FxHasher64::default();

    name.hash(&mut hasher);
    ty.hash(&mut hasher);

    Self(hasher.finish())
  }
}

impl Deref for FieldArchetypeId {
  type Target = u64;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Hash for FieldArchetypeId {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u64(self.0)
  }
}

impl PartialEq for FieldArchetypeId {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

#[derive(Debug, Hash)]
pub struct FieldArchetype {
  pub name: String,
  pub ty: Type,
}

impl FieldArchetype {
  pub fn new(name: String, ty: Type) -> Self {
    Self { name, ty }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn id_equals_archetype_hash() {
    let arch_id = FieldArchetypeId::new("foo", Type::Int32);
    let arch = FieldArchetype::new("foo".into(), Type::Int32);

    let mut hasher = FxHasher64::default();
    arch.hash(&mut hasher);
    let arch_hash = hasher.finish();

    assert_eq!(*arch_id, arch_hash);
  }
}
