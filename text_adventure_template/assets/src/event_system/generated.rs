use crate::event_system::events::EventType;
use crate::scene_system::scene_id::SceneId;

pub enum EventDelegate {
    OnMoveScenesRequest(fn(SceneId)),
    OnGameStart(fn()),
    MoveLeft(fn()),
    MoveRight(fn()),
    OnApplicationShutdown(fn()),
    QuitApplication(fn()),
    WinGame(fn()),
    LoseGame(fn())
}

pub struct AllEvents {
    on_move_scenes_request: Vec<fn(SceneId)>,
    on_game_start: Vec<fn()>,
    move_left: Vec<fn()>,
    move_right: Vec<fn()>,
    on_application_shutdown: Vec<fn()>,
    quit_application: Vec<fn()>,
    win_game: Vec<fn()>,
    lose_game: Vec<fn()>
}

impl AllEvents {
    pub fn new() -> Self {
        Self {
            on_move_scenes_request: Vec::new(),
            on_game_start: Vec::new(),
            move_left: Vec::new(),
            move_right: Vec::new(),
            on_application_shutdown: Vec::new(),
            quit_application: Vec::new(),
            win_game: Vec::new(),
            lose_game: Vec::new()
        }
    }

    pub fn broadcast(&self, event: EventType)
    {
        match event {
            EventType::OnMoveScenesRequest(param) => {
                for func in self.on_move_scenes_request.iter() {
                    func(param);
                }
            },
            EventType::OnGameStart => {
                for func in self.on_game_start.iter() {
                    func();
                }
            },
            EventType::MoveLeft => {
                for func in self.move_left.iter() {
                    func();
                }
            },
            EventType::MoveRight => {
                for func in self.move_right.iter() {
                    func();
                }
            },
            EventType::OnApplicationShutdown => {
                for func in self.on_application_shutdown.iter() {
                    func();
                }
            },
            EventType::QuitApplication => {
                for func in self.quit_application.iter() {
                    func();
                }
            },
            EventType::WinGame => {
                for func in self.win_game.iter() {
                    func();
                }
            },
            EventType::LoseGame => {
                for func in self.lose_game.iter() {
                    func();
                }
            }
        }
    }

    pub fn bind(&mut self, event: EventDelegate) {
        match event {
            EventDelegate::OnMoveScenesRequest(func) => self.on_move_scenes_request.push(func),
            EventDelegate::OnGameStart(func) => self.on_game_start.push(func),
            EventDelegate::MoveLeft(func) => self.move_left.push(func),
            EventDelegate::MoveRight(func) => self.move_right.push(func),
            EventDelegate::OnApplicationShutdown(func) => self.on_application_shutdown.push(func),
            EventDelegate::QuitApplication(func) => self.quit_application.push(func),
            EventDelegate::WinGame(func) => self.win_game.push(func),
            EventDelegate::LoseGame(func) => self.lose_game.push(func)
        }
    }
}