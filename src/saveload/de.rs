use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;

use serde::de::{self, Deserialize, DeserializeOwned, DeserializeSeed, Deserializer, SeqAccess,
                Visitor};

use saveload::EntityData;
use saveload::marker::{Marker, MarkerAllocator};
<<<<<<< HEAD
use shred::Write;
=======
>>>>>>> f83d15e... Saveload overhaul
use storage::WriteStorage;
use world::{Component, EntitiesRes, Entity};

/// A trait which allows to deserialize entities and their components.
pub trait DeserializeComponents<E, M>
where
    Self: Sized,
    E: Display,
    M: Marker,
{
    /// The data representation that a component group gets deserialized to.
    type Data: DeserializeOwned;

    /// The error type.
    type Error: Display;

    /// Loads `Component`s to entity from `Data` deserializable representation
    fn deserialize_entity<'a, F>(
        &mut self,
        entity: Entity,
        components: Self::Data,
        ids: F,
    ) -> Result<(), E>
    where
        F: FnMut(M) -> Option<Entity>;

    /// Deserialize entities according to markers.
    fn deserialize<'b, 'de, D>(
        &'b mut self,
        entities: &'b EntitiesRes,
        markers: &'b mut WriteStorage<'b, M>,
        allocator: &'b mut M::Allocator,
        deserializer: D,
    ) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(VisitEntities::<E, M, Self> {
            allocator,
            entities,
            markers,
            storages: self,
            pd: PhantomData,
        })
    }
}

/// Wrapper for `Entity` and tuple of `WriteStorage`s that implements `serde::Deserialize`.
<<<<<<< HEAD
struct DeserializeEntity<'a, 'b: 'a, M: Marker, E, T: Components<M::Identifier, E>> {
    entities: &'a Entities<'b>,
    storages: &'a mut <T as Storages<'b>>::WriteStorages,
    markers: &'a mut WriteStorage<'b, M>,
    allocator: &'a mut Write<'b, M::Allocator>,
    pd: PhantomData<(E, T)>,
=======
struct DeserializeEntity<'a: 'b, 'b, 's, E, M: Marker, S: 's> {
    allocator: &'b mut M::Allocator,
    entities: &'b EntitiesRes,
    storages: &'s mut S,
    markers: &'b mut WriteStorage<'a, M>,
    pd: PhantomData<E>,
>>>>>>> f83d15e... Saveload overhaul
}

impl<'de, 'a: 'b, 'b, 's, E, M, S> DeserializeSeed<'de> for DeserializeEntity<'a, 'b, 's, E, M, S>
where
    E: Display,
    M: Marker,
    S: DeserializeComponents<E, M> + 's,
{
    type Value = ();
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        let DeserializeEntity {
            entities,
            storages,
            markers,
            allocator,
            ..
        } = self;
        let data = EntityData::<M, S::Data>::deserialize(deserializer)?;
        let entity = allocator.get_entity(data.marker.id(), entities, markers);
        // TODO: previously, update was called here
        // TODO: should we still do that?
        let ids = |marker: M| Some(allocator.get_entity(marker.id(), entities, markers));

        storages
            .deserialize_entity(entity, data.components, ids)
            .map_err(de::Error::custom)
    }
}

<<<<<<< HEAD
/// Wrapper for `Entities` and tuple of `WriteStorage`s that implements `serde::de::Visitor`
struct VisitEntities<'a, 'b: 'a, M: Marker, E, T: Components<M::Identifier, E>> {
    entities: &'a Entities<'b>,
    storages: &'a mut <T as Storages<'b>>::WriteStorages,
    markers: &'a mut WriteStorage<'b, M>,
    allocator: &'a mut Write<'b, M::Allocator>,
    pd: PhantomData<(E, T)>,
}
=======
pub trait IntoDeserialize<M>: Component {
    /// Serializable data representation for component
    type Data: DeserializeOwned;
>>>>>>> f83d15e... Saveload overhaul

    /// Error may occur during serialization or deserialization of component
    type Error;

    /// Convert this component from a deserializable form (`Data`) using
    /// entity to marker mapping function
    fn into<F>(&self, data: Self::Data, ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>;
}

<<<<<<< HEAD
/// Deserialize entities according to markers.
pub fn deserialize<'a, 'de, D, M, E, T>(
    entities: &Entities<'a>,
    storages: &mut <T as Storages<'a>>::WriteStorages,
    markers: &mut WriteStorage<'a, M>,
    allocator: &mut Write<'a, M::Allocator>,
    deserializer: D,
) -> Result<(), D::Error>
where
    M: Marker,
    E: Display,
    T: Components<M::Identifier, E>,
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(VisitEntities::<M, E, T> {
        entities,
        storages,
        markers,
        allocator,
        pd: PhantomData,
    })
}

/// Struct which implements `DeserializeSeed` to allow serializing
/// components from `World`.
#[derive(SystemData)]
pub struct WorldDeserialize<'a, M: Marker, E, T: Components<M::Identifier, E>> {
    entities: Entities<'a>,
    storages: <T as Storages<'a>>::WriteStorages,
    markers: WriteStorage<'a, M>,
    allocator: Write<'a, M::Allocator>,
=======
/// Wrapper for `Entities` and tuple of `WriteStorage`s that implements `serde::de::Visitor`
struct VisitEntities<'a: 'b, 'b, E, M: Marker, S: 'b> {
    allocator: &'b mut M::Allocator,
    entities: &'b EntitiesRes,
    markers: &'b mut WriteStorage<'a, M>,
    storages: &'b mut S,
>>>>>>> f83d15e... Saveload overhaul
    pd: PhantomData<E>,
}

impl<'de, 'a, 'b: 'a, E, M, S> Visitor<'de> for VisitEntities<'a, 'b, E, M, S>
where
    E: Display,
    M: Marker,
    S: DeserializeComponents<E, M>,
{
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Sequence of serialized entities")
    }

    fn visit_seq<SEQ>(self, mut seq: SEQ) -> Result<(), SEQ::Error>
    where
        SEQ: SeqAccess<'de>,
    {
        loop {
            let ret = seq.next_element_seed(DeserializeEntity {
                entities: self.entities,
                storages: self.storages,
                markers: self.markers,
                allocator: self.allocator,
                pd: self.pd,
            })?;

            if ret.is_none() {
                break Ok(());
            }
        }
    }
}
