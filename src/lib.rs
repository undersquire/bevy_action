use core::marker::PhantomData;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::Entity;
use bevy_reflect::TypeUuid;

/// Implemented for any custom action type.
///
/// NOTE: Currently requires implementing [`TypeUuid`]. This may be removed in the future.
pub trait Action:
    Sized
    + Clone
    + std::fmt::Debug
    + PartialEq
    + Eq
    + std::hash::Hash
    + Send
    + Sync
    + 'static
    + Default
    + TypeUuid
{
}

/// An ordinary Bevy event that contains the action and the entity that fired it.
#[derive(Clone, Debug)]
pub struct ActionEvent<T: Action> {
    pub action: T,
    pub entity: Entity,
}

/// NOTE: Must be registered for each custom action type.
#[derive(Default)]
pub struct ActionPlugin<T: Action>(PhantomData<T>);

impl<T: Action> Plugin for ActionPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent<T>>();
    }
}
