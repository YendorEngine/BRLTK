//! Custom render scheduler for `doryen-rs`.

use bevy::prelude::*;

mod ext;
pub use ext::*;
mod state;
pub use state::*;
mod image;
pub use self::image::*;

/// Render plugin for Bevy Doryen.
pub struct DoryenRenderPlugin;
impl Plugin for DoryenRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderState>()
            .init_resource::<DoryenRenderSystems>();
    }
}

/// The names of the Doryen plugin render stages.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, StageLabel)]
pub enum DoryenRenderStage {
    /// This stage runs before all the other stages.
    First,
    /// This stage runs right before the render stage.
    PreRender,
    /// This stage is where rendering should be done.
    Render,
    /// This stage runs right after the render stage.
    PostRender,
    /// This stage runs after all the other stages.
    Last,
}

#[derive(Resource)]
pub(crate) struct DoryenRenderSystems(pub(crate) Option<Schedule>);
impl Default for DoryenRenderSystems {
    fn default() -> Self {
        let mut scheduler = Schedule::default();

        scheduler
            .add_stage(DoryenRenderStage::First, SystemStage::single_threaded())
            .add_stage_after(
                DoryenRenderStage::First,
                DoryenRenderStage::PreRender,
                SystemStage::single_threaded(),
            )
            .add_stage_after(
                DoryenRenderStage::PreRender,
                DoryenRenderStage::Render,
                SystemStage::single_threaded(),
            )
            .add_stage_after(
                DoryenRenderStage::Render,
                DoryenRenderStage::PostRender,
                SystemStage::single_threaded(),
            )
            .add_stage_after(
                DoryenRenderStage::PostRender,
                DoryenRenderStage::Last,
                SystemStage::single_threaded(),
            );

        Self(Some(scheduler))
    }
}
