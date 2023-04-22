use core::marker::PhantomData;

pub use bevy_action_derive::*;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::Entity;

/// Implemented for any custom action type.
pub trait Action:
    Sized + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static + Default
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
