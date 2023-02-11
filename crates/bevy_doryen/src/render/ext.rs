use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use crate::*;

/// Adds methods to the [`AppBuilder`] for adding systems to the Doryen
/// [`render`](crate::doryen::Engine::render) schedule.
pub trait RenderSystemExt {
    /// Adds a system to the [`RenderStage::Render`] stage of the
    /// render schedule.
    fn add_doryen_render_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self;

    /// Adds a system to the given stage of the render schedule.
    fn add_doryen_render_system_to_stage<Params>(
        &mut self,
        stage_name: impl StageLabel,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self;

    /// Adds a system set to the [`RenderStage::Render`] stage of the
    /// render schedule.
    fn add_doryen_render_system_set(&mut self, system_set: SystemSet) -> &mut Self;

    /// Adds a system set to the given stage of the render schedule.
    fn add_doryen_render_system_set_to_stage(
        &mut self,
        stage_label: impl StageLabel,
        system_set: SystemSet,
    ) -> &mut Self;

    /// Adds a [`State`] to the render schedule. This method assumes you've
    /// already added the State to the main Bevy app through
    /// [`AppBuilder::add_state`] or similar means.
    ///
    /// If you want to make use of
    /// [`on_inactive_update`](State::on_inactive_update) and
    /// [`on_in_stack_update`](State::on_in_stack_update) run criteria, you must
    /// ask for [`ResMut<RenderState>`](RenderState) in the same systems that
    /// call one of the `State` transition methods, and call
    /// [`state_updated`](RenderState::state_updated) on it, otherwise they
    /// won't work. This is due to a limitation with how `State` works in
    /// general; even trying to use those from a different
    /// [`Stage`](bevy_ecs::schedule::Stage) in Bevy will have the same issue.
    ///
    /// Important note: this must be inserted **before** all other
    /// state-dependant sets to work properly!
    fn add_doryen_render_state<T>(&mut self) -> &mut Self
    where T: Component + Debug + Clone + Eq + Hash;
}

#[inline(always)]
fn do_to_doryen_render_systems<F: FnOnce(&mut DoryenRenderSystems)>(app_builder: &mut App, operation: F) {
    let mut doryen_render_systems = app_builder.world.get_resource_mut::<DoryenRenderSystems>().unwrap();
    operation(&mut doryen_render_systems)
}

#[inline(always)]
fn do_to_doryen_render_systems_schedule<F: FnOnce(&mut Schedule)>(app_builder: &mut App, operation: F) {
    do_to_doryen_render_systems(app_builder, |drs| operation(drs.0.as_mut().unwrap()));
}

impl RenderSystemExt for App {
    fn add_doryen_render_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        do_to_doryen_render_systems_schedule(self, move |drss| {
            drss.add_system_to_stage(DoryenRenderStage::Render, system);
        });
        self
    }

    fn add_doryen_render_system_to_stage<Params>(
        &mut self,
        stage_label: impl StageLabel,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        do_to_doryen_render_systems_schedule(self, move |drss| {
            drss.add_system_to_stage(stage_label, system);
        });
        self
    }

    fn add_doryen_render_system_set(&mut self, system_set: SystemSet) -> &mut Self {
        do_to_doryen_render_systems_schedule(self, move |drss| {
            drss.add_system_set_to_stage(DoryenRenderStage::Render, system_set);
        });
        self
    }

    fn add_doryen_render_system_set_to_stage(
        &mut self,
        stage_label: impl StageLabel,
        system_set: SystemSet,
    ) -> &mut Self {
        do_to_doryen_render_systems_schedule(self, move |drss| {
            drss.add_system_set_to_stage(stage_label, system_set);
        });
        self
    }

    fn add_doryen_render_state<T>(&mut self) -> &mut Self
    where T: Component + Debug + Clone + Eq + Hash {
        self.add_doryen_render_system_set_to_stage(DoryenRenderStage::Render, State::<T>::get_driver())
    }
}
