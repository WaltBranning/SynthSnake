use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: i32,
}

impl Score {
    pub fn increment(&mut self) {
        self.value += 1;
    }
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct EndGame {
    pub is_game_over: bool,
}

impl Default for EndGame {
    fn default() -> Self {
        EndGame { is_game_over: false }
    }
}
