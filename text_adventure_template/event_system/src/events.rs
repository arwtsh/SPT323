#[derive(Clone, Copy)]
/// Each event that can be sent along with the parameters for that event type.
pub enum EventType {
    StartGame,
    EndGame,
    OnGameStart,
    OnGameEnd,
    MoveScenes(bool)
}