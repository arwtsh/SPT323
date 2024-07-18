use crate::events::EventType;

pub enum EventDelegate {
    StartGame(fn()),
    EndGame(fn()),
    OnGameStart(fn()),
    OnGameEnd(fn()),
    MoveScenes(fn(bool))
}

pub struct AllEvents {
    start_game: Vec<fn()>,
    end_game: Vec<fn()>,
    on_game_start: Vec<fn()>,
    on_game_end: Vec<fn()>,
    move_scenes: Vec<fn(bool)>
}

impl AllEvents {
    pub fn new() -> Self {
        Self {
            start_game: Vec::new(),
            end_game: Vec::new(),
            on_game_start: Vec::new(),
            on_game_end: Vec::new(),
            move_scenes: Vec::new()
        }
    }

    pub fn broadcast(&self, event: EventType)
    {
        match event {
            EventType::StartGame => {
                for func in self.start_game.iter() {
                    func();
                }
            },
            EventType::EndGame => {
                for func in self.end_game.iter() {
                    func();
                }
            },
            EventType::OnGameStart => {
                for func in self.on_game_start.iter() {
                    func();
                }
            },
            EventType::OnGameEnd => {
                for func in self.on_game_start.iter() {
                    func();
                }
            },
            EventType::MoveScenes(param) => {
                for func in self.move_scenes.iter() {
                    func(param);
                }
            }
        }
    }

    pub fn bind(&mut self, event: EventDelegate) {
        match event {
            EventDelegate::StartGame(func) => self.start_game.push(func),
            EventDelegate::EndGame(func) => self.end_game.push(func),
            EventDelegate::OnGameStart(func) => self.on_game_start.push(func),
            EventDelegate::OnGameEnd(func) => self.on_game_end.push(func),
            EventDelegate::MoveScenes(func) => self.move_scenes.push(func),
        }
    }
}