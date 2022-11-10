use std::hash::{Hash, Hasher};
use std::ops::Deref;

use fxhash::FxHasher64;

use super::FieldArchetypeId;

#[derive(Clone, Copy, Debug, Default, Eq, PartialOrd, Ord)]
pub struct ObjectArchetypeId(u64);

impl ObjectArchetypeId {
  pub fn new(field_ids: &[FieldArchetypeId]) -> Self {
    let mut hasher = FxHasher64::default();
    field_ids.hash(&mut hasher);
    Self(hasher.finish())
  }
}

impl Deref for ObjectArchetypeId {
  type Target = u64;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Hash for ObjectArchetypeId {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u64(self.0)
  }
}

impl PartialEq for ObjectArchetypeId {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

#[derive(Debug, Hash)]
pub struct ObjectArchetype {
  pub field_ids: Vec<FieldArchetypeId>,
}

impl ObjectArchetype {
  pub fn new(field_ids: Vec<FieldArchetypeId>) -> Self {
    Self { field_ids }
  }
}

#[cfg(test)]
mod tests {
  use crate::r#type::Type;

  use super::*;

  #[test]
  fn id_equals_archetype_hash() {
    let field_ids = vec![FieldArchetypeId::new("foo", Type::Int32)];
    let arch_id = ObjectArchetypeId::new(&field_ids);
    let arch = ObjectArchetype::new(field_ids);

    let mut hasher = FxHasher64::default();
    arch.hash(&mut hasher);
    let arch_hash = hasher.finish();

    assert_eq!(*arch_id, arch_hash);
  }
}
