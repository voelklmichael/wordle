#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Statistics {
    pub started_games: usize,
    pub won_games: Vec<u8>, // number of guesses for each won games
    pub lost_games: usize,  // lost games=>six guesses
}
impl Statistics {
    pub(crate) fn new_game(&mut self) {
        self.started_games += 1;
    }

    pub(crate) fn game_completed(&mut self, guesses: Option<usize>) {
        if let Some(guesses) = guesses {
            self.won_games.push(guesses as u8);
        } else {
            self.lost_games += 1;
        }
    }
}
