#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Statistics {
    #[serde(skip)]
    pub last_time_start: Option<web_time::Instant>,
    pub started_games: usize,
    pub won_games: Vec<(u8, Option<u32>)>, // number of guesses for each won games
    pub lost_games: Vec<Option<u32>>,      // lost games=>six guesses
}
impl Statistics {
    pub(crate) fn new_game(&mut self) {
        self.started_games += 1;
        self.last_time_start = Some(web_time::Instant::now());
    }

    pub(crate) fn game_completed(&mut self, guesses: Option<usize>) {
        let duration = if let Some(last_time_start) = self.last_time_start.take() {
            Some((web_time::Instant::now() - last_time_start).as_secs() as u32)
        } else {
            None
        };
        if let Some(guesses) = guesses {
            self.won_games.push((guesses as u8, duration));
        } else {
            self.lost_games.push(duration);
        }
    }
}
