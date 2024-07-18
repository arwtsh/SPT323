#[derive(Clone, Copy)]
/// Each event that can be sent along with the parameters for that event type.
pub enum EventType {
    OnMoveScenesRequest(scene_util::scene_id::SceneId),
    OnGameStart,
    MoveLeft,
    MoveRight,
    OnApplicationShutdown,
    QuitApplication
}