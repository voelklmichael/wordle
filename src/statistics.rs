use crate::app::TRY_COUNT;

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

    pub(crate) fn show(&mut self, ui: &mut egui::Ui) {
        if self.started_games == 0 {
            ui.label("Not yet any games played");
            return;
        }
        egui::Grid::new("basic_statistics")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                let data = [
                    ("Total games", self.started_games.to_string()),
                    ("Won games", self.won_games.len().to_string()),
                    ("Lost games", self.lost_games.len().to_string()),
                    (
                        "Win ratio",
                        format!(
                            "{:.2}%",
                            self.won_games.len() as f32 / self.started_games as f32 * 100.0
                        ),
                    ),
                ];
                for (label, value) in data {
                    ui.label(label);
                    ui.label(value);
                    ui.end_row();
                }
            });

        ui.separator();
        if !self.won_games.is_empty() {
            ui.label("Guesses per win");
            {
                let bars = (0..=crate::app::TRY_COUNT)
                    .map(|i| {
                        let count = self
                            .won_games
                            .iter()
                            .filter(|(c, _)| *c as usize == i)
                            .count();
                        let ratio = count as f64 / self.won_games.len() as f64 * 100.0;
                        (i, ratio)
                    })
                    .map(|(guesses, ratio)| egui_plot::Bar::new(guesses as _, ratio))
                    .collect::<Vec<_>>();

                egui_plot::Plot::new("statistics_simple_plot")
                    .allow_boxed_zoom(false)
                    .allow_double_click_reset(false)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .default_x_bounds(1., TRY_COUNT as f64 + 0.2)
                    //.cursor_color(egui::Color32::TRANSPARENT)
                    .sense(egui::Sense::empty())
                    .label_formatter(|_, _| Default::default())
                    .x_axis_label("Guesses")
                    .y_axis_label("Ratio [%]")
                    .show(ui, |ui| {
                        ui.bar_chart(egui_plot::BarChart::new("Guesses", bars));
                    });
            }
        }
    }
}
