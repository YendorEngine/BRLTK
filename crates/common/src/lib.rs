pub trait Backend: Send + Sync {
    fn build(&self, app: &mut bevy::app::App);
}
