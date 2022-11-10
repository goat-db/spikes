use scc::ebr::Arc;
use scc::TreeIndex;

use crate::value::Value;
use crate::Object;

use super::{
  FieldArchetype, FieldArchetypeId, ObjectArchetype, ObjectArchetypeId
};

#[derive(Debug, Default)]
pub struct Registry {
  fields: TreeIndex<FieldArchetypeId, Arc<FieldArchetype>>,
  objects: TreeIndex<ObjectArchetypeId, Arc<ObjectArchetype>>,
}

impl Registry {
  #[must_use]
  pub fn assign_field(
    &mut self,
    name: &str,
    value: &Value,
  ) -> FieldArchetypeId {
    let ty = value.ty();
    let id = FieldArchetypeId::new(name, ty);

    if self.fields.read(&id, |k, _| *k).is_none() {
      self
        .fields
        .insert(id, Arc::new(FieldArchetype::new(name.to_owned(), ty)))
        .unwrap();
    }

    id
  }

  #[must_use]
  pub fn assign_object(&mut self, object: &Object) -> ObjectArchetypeId {
    let field_ids: Vec<_> = object
      .iter()
      .map(|(n, v)| self.assign_field(n, v))
      .collect();

    let object_id = ObjectArchetypeId::new(&field_ids);

    if self.objects.read(&object_id, |k, _| *k).is_none() {
      self
        .objects
        .insert(object_id, Arc::new(ObjectArchetype::new(field_ids)))
        .unwrap();
    }

    object_id
  }

  #[must_use]
  pub fn get_field_archetype(
    &self,
    id: FieldArchetypeId,
  ) -> Option<Arc<FieldArchetype>> {
    self.fields.read(&id, |_, v| v.clone())
  }

  #[must_use]
  pub fn get_object_archetype(
    &self,
    id: ObjectArchetypeId,
  ) -> Option<Arc<ObjectArchetype>> {
    self.objects.read(&id, |_, v| v.clone())
  }
}

#[cfg(test)]
mod tests {
  use uuid::Uuid;

  use super::*;

  #[test]
  fn assign_the_same_archetype_to_similar_fields() {
    let mut registry = Registry::default();

    let arch_id1 = registry.assign_field("foo", &Value::Bool(true));
    let arch_id2 = registry.assign_field("foo", &Value::Bool(false));

    assert_eq!(arch_id1, arch_id2);
  }

  #[test]
  fn assign_different_archetypes_for_fields_with_different_names() {
    let mut registry = Registry::default();

    let arch_id1 = registry.assign_field("foo", &Value::Bool(true));
    let arch_id2 = registry.assign_field("bar", &Value::Bool(false));

    assert_ne!(arch_id1, arch_id2);
  }

  #[test]
  fn assign_different_archetypes_for_fields_with_different_value_types() {
    let mut registry = Registry::default();

    let arch_id1 = registry.assign_field("foo", &Value::Bool(true));
    let arch_id2 = registry.assign_field("foo", &Value::Int8(42));

    assert_ne!(arch_id1, arch_id2);
  }

  #[test]
  fn assign_the_same_archetype_to_similar_shaped_objects() {
    let mut registry = Registry::default();

    let o1 = Object::from([
      ("id".into(), Value::Uuid(Uuid::new_v4())),
      ("name".into(), Value::String("Nicolas".into())),
      ("age".into(), Value::Uint8(38)),
    ]);
    let arch_id1 = registry.assign_object(&o1);

    let o2 = Object::from([
      ("name".into(), Value::String("Nicolas".into())),
      ("age".into(), Value::Uint8(38)),
      ("id".into(), Value::Uuid(Uuid::new_v4())),
    ]);
    let arch_id2 = registry.assign_object(&o2);

    assert_eq!(arch_id1, arch_id2);
  }

  #[test]
  fn assign_different_archetypes_for_objects_with_different_sets_of_fields() {
    let mut registry = Registry::default();

    let o1 = Object::from([
      ("id".into(), Value::Uuid(Uuid::new_v4())),
      ("name".into(), Value::String("Nicolas".into())),
      ("age".into(), Value::Uint8(38)),
    ]);
    let arch_id1 = registry.assign_object(&o1);

    let o2 = Object::from([
      ("name".into(), Value::String("Nicolas".into())),
      ("id".into(), Value::Uuid(Uuid::new_v4())),
    ]);
    let arch_id2 = registry.assign_object(&o2);

    assert_ne!(arch_id1, arch_id2);
  }

  #[test]
  fn assign_different_archetypes_for_objects_with_fields_with_different_types()
  {
    let mut registry = Registry::default();

    let o1 = Object::from([
      ("id".into(), Value::Uuid(Uuid::new_v4())),
      ("name".into(), Value::String("Nicolas".into())),
      ("age".into(), Value::Uint8(38)),
    ]);
    let arch_id1 = registry.assign_object(&o1);

    let o2 = Object::from([
      ("id".into(), Value::Uint64(1337)),
      ("name".into(), Value::String("Nicolas".into())),
      ("age".into(), Value::Uint8(38)),
    ]);
    let arch_id2 = registry.assign_object(&o2);

    assert_ne!(arch_id1, arch_id2);
  }

  #[test]
  fn get_an_existing_field_archetype() {
    let mut registry = Registry::default();

    let arch_id = registry.assign_field("foo", &Value::Bool(true));
    let arch = registry.get_field_archetype(arch_id);

    assert!(arch.is_some());
  }

  #[test]
  fn get_an_existing_object_archetype() {
    let mut registry = Registry::default();

    let o = Object::from([("id".into(), Value::Uint64(1337))]);
    let arch_id = registry.assign_object(&o);
    let arch = registry.get_object_archetype(arch_id);

    assert!(arch.is_some());
  }
}
