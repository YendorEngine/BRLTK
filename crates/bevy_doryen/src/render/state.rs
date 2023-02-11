use bevy::{ecs::world::WorldCell, prelude::Resource};

/// RenderState is a resource that gets added to Bevy to facilitate certain
/// features of Bevy's [`State`]s.
///
/// By default, only system sets in the same
/// stage as the one a `State` was changed in can make use of the
/// [`on_inactive_update`](State::on_inactive_update) and
/// [`on_in_stack_update`](State::on_in_stack_update) run criteria. Since
/// bevy_doryen runs render systems in an entirely different [`Schedule`], we're
/// obviously way outside "the same stage" as where you typically run your
/// update code.
///
/// By calling [`RenderState::state_updated`] when you change a [`State`],
/// you enable the use of the two run criteria mentioned above in the render
/// schedule as well.
#[derive(Resource)]
pub struct RenderState(pub(crate) bool, pub(crate) Vec<fn(&WorldCell<'_>)>);
impl RenderState {
    /// Call this method whenever you change a [`State`], i.e. when you call
    /// [`State::push`] and friends to tell bevy_doryen to run some extra code
    /// in the [`State`] that lets them work in the render [`Schedule`].
    pub fn state_updated(&mut self) {
        self.0 = true;
    }
}

impl Default for RenderState {
    fn default() -> Self {
        Self(true, Vec::new())
    }
}

impl std::fmt::Debug for RenderState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RenderState")
            .field(&self.0)
            .field(&format!("fn(&WorldCell<'_>) count = {}", self.1.len()))
            .finish()
    }
}
