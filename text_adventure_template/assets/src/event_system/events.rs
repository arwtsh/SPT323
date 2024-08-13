use crate::scene_system::scene_id::SceneId;

#[derive(Clone, Copy)]
/// Each event that can be sent along with the parameters for that event type.
pub enum EventType {
    OnMoveScenesRequest(SceneId),
    OnGameStart,
    MoveLeft,
    MoveRight,
    OnApplicationShutdown,
    QuitApplication,
    LoseGame,
    WinGame
}