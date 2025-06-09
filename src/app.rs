use crate::wordlist::{self, CHAR_COUNT, TargetWord, WordWithLink};

pub const TRY_COUNT: usize = 6;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WordleApp {
    font_size_adjustment: f32,
    #[serde(skip)]
    wordlist: Vec<WordWithLink>,
    used_targets: std::collections::HashSet<TargetWord>,
    current_target: Option<WordWithLink>,
    current_guess: [Option<char>; CHAR_COUNT],
    previous_guesses: [Option<WordWithLink>; TRY_COUNT],
    current_selected: usize,
    game_is_won: Option<bool>,
    error_message: Option<String>,
    #[serde(default)]
    statistics: crate::statistics::Statistics,
}

impl WordleApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.style_mut(|style| {
            style.text_styles.iter_mut().for_each(|x| x.1.size = 30.);
        });
        if let Some(storage) = cc.storage {
            let app: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            cc.egui_ctx.style_mut(|style| {
                style.text_styles.iter_mut().for_each(|font| {
                    let size = &mut font.1.size;
                    let new_size = *size + app.font_size_adjustment;
                    *size = if new_size > 1. { new_size } else { 1. }
                });
            });

            return app;
        }

        Default::default()
    }

    fn new_game(&mut self) {
        let target = loop {
            let random = match getrandom::u32() {
                Ok(value) => value,
                Err(err) => {
                    log::error!("Failed to get random number: {err}");
                    return;
                }
            };
            let words = self.wordlist.len() as u32;
            let max = (u32::MAX / words) * words;
            if random >= max {
                continue;
            }
            let index = random % words;
            let target = self.wordlist[index as usize].clone();
            if self.used_targets.contains(&target.word) {
                continue;
            }
            break target;
        };
        assert!(self.used_targets.insert(target.word));

        self.current_target = Some(target);
        self.previous_guesses = Default::default();
        self.current_guess = Default::default();
        self.current_selected = 0;
        self.game_is_won = None;
        self.error_message = None;
        self.statistics.new_game();
    }

    fn draw_letter_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("letter_grid")
            .num_columns(CHAR_COUNT + 1)
            .show(ui, |ui| {
                // previous guesses
                for previous in &self.previous_guesses {
                    let Some(previous) = previous else {
                        break;
                    };
                    let Some(target) = &self.current_target else {
                        self.error_message = Some("Target not set - this is a bug".into());
                        return;
                    };
                    let target = target
                        .word
                        .map(|x| x.to_uppercase().into_iter().next().unwrap());
                    for (&p, t) in previous.word.iter().zip(target) {
                        let text = egui::RichText::new(p).family(egui::FontFamily::Monospace);
                        let text = if p == t {
                            text.background_color(egui::Color32::GREEN)
                                .color(egui::Color32::BLACK)
                        } else if target.contains(&p) {
                            text.background_color(egui::Color32::YELLOW)
                                .color(egui::Color32::BLACK)
                        } else {
                            text
                        };
                        ui.group(|ui| {
                            ui.add_enabled(false, egui::SelectableLabel::new(false, text));
                        });
                    }
                    ui.add(
                        egui::Hyperlink::from_label_and_url(
                            "🏷",
                            format!("https://de.wiktionary.org/wiki/{}", previous.link),
                        )
                        .open_in_new_tab(true),
                    );
                    ui.end_row();
                }

                // current guess
                let skip_current = if self.game_is_won.is_none() {
                    for (i, x) in self.current_guess.iter_mut().enumerate() {
                        ui.group(|ui| {
                            if ui
                                .selectable_label(
                                    i == self.current_selected,
                                    egui::RichText::new(x.unwrap_or(' '))
                                        .family(egui::FontFamily::Monospace),
                                )
                                .clicked()
                            {
                                self.current_selected = i;
                            }
                        });
                    }
                    ui.end_row();
                    1
                } else {
                    0
                };

                // remaining rows
                for _ in self
                    .previous_guesses
                    .iter()
                    .skip_while(|x| x.is_some())
                    .skip(skip_current)
                {
                    for _ in 0..CHAR_COUNT {
                        ui.group(|ui| {
                            ui.add_enabled(
                                false,
                                egui::SelectableLabel::new(
                                    false,
                                    egui::RichText::new(" ").family(egui::FontFamily::Monospace),
                                ),
                            );
                        });
                    }
                    ui.end_row();
                }
            });
    }

    fn draw_letter_selection(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::left("letter_remove_current3")
            .max_width(3.)
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |_| {});
        egui::SidePanel::left("letter_remove_current")
            .max_width(24.)
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    if ui.button("✖").clicked() {
                        self.letter_remove_current();
                    }
                });
            });
        egui::SidePanel::left("letter_remove_current2")
            .max_width(3.)
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |_| {});

        egui::SidePanel::right("letter_enter_current_word3")
            .max_width(3.)
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |_| {});
        egui::SidePanel::right("letter_enter_current_word")
            .max_width(24.)
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    if ui.button("⮨").clicked() {
                        self.letter_enter_current_word();
                    }
                });
            });
        egui::SidePanel::right("letter_enter_current_word2")
            .max_width(3.)
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |_| {});

        egui::CentralPanel::default()
            .frame(egui::Frame::new().stroke(egui::Stroke::new(0., egui::Color32::TRANSPARENT)))
            .show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let alphabet = "abcdefghijklmnopqrstuvwxyz";
                    for letter in alphabet.chars() {
                        let letter = letter.to_uppercase();
                        let letter = letter.to_string().chars().next().unwrap();
                        let target_contains_letter = self
                            .current_target
                            .as_ref()
                            .map(|x| x.word.contains(&letter))
                            .unwrap_or_default();
                        let previous_contains_letter = self
                            .previous_guesses
                            .iter()
                            .filter_map(|x| x.as_ref().map(|x| x.word.contains(&letter)))
                            .any(|x| x);
                        let button = if previous_contains_letter {
                            egui::Button::new(
                                egui::RichText::new(letter.to_string())
                                    .color(egui::Color32::BLACK)
                                    .family(egui::FontFamily::Monospace),
                            )
                            .fill(if target_contains_letter {
                                egui::Color32::GREEN
                            } else {
                                egui::Color32::RED
                            })
                        } else {
                            egui::Button::new(
                                egui::RichText::new(letter.to_string())
                                    .family(egui::FontFamily::Monospace),
                            )
                        };
                        if egui::Widget::ui(button, ui).clicked() {
                            self.current_guess[self.current_selected] = Some(letter);
                            let next = self.current_selected + 1;
                            if next < CHAR_COUNT {
                                self.current_selected = next;
                            }
                            self.error_message = None;
                        }
                    }
                });
            });
    }

    fn letter_remove_current(&mut self) {
        self.current_guess[self.current_selected] = None;
        if self.current_selected > 0 {
            self.current_selected -= 1;
        }
        self.error_message = None;
    }

    fn letter_enter_current_word(&mut self) {
        self.error_message = None;
        if self.current_guess.iter().any(|x| x.is_none()) {
            dbg!("This should never be allowed");
            return;
        }
        let guess = self.current_guess.map(|x| x.unwrap());
        let guess_lowercase =
            guess.map(|x| x.to_ascii_uppercase().to_string().chars().next().unwrap());
        if let Some(guess_in_wordlist) = self.wordlist.iter().find(|x| x.word == guess_lowercase) {
            if let Some(entry) = self.previous_guesses.iter_mut().find(|x| x.is_none()) {
                *entry = Some(guess_in_wordlist.clone());
            } else {
                panic!("This should never happen!")
            }
            if guess_lowercase == self.current_target.as_ref().unwrap().word {
                self.game_is_won = Some(true);
                self.statistics.game_completed(Some(
                    self.previous_guesses
                        .iter()
                        .take_while(|x| x.is_some())
                        .count(),
                ));
            } else if self.previous_guesses.iter().all(|x| x.is_some()) {
                self.game_is_won = Some(false);
                self.statistics.game_completed(None);
            }
            self.current_selected = 0;
            self.current_guess = Default::default();
        } else {
            self.error_message = Some("Word is not contained in wordlist!".into());
        }
    }
}

impl eframe::App for WordleApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.wordlist.is_empty() {
            self.wordlist = wordlist::wordlist_german();
        }
        if self.current_target.is_none() {
            self.new_game();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                if ui.button("New Game").clicked() {
                    self.new_game()
                }
                if ui.button("+").clicked() {
                    self.font_size_adjustment += 1.;
                    ctx.style_mut(|style| {
                        style
                            .text_styles
                            .iter_mut()
                            .for_each(|font| font.1.size += 1.);
                    });
                }
                if ui.button("-").clicked() {
                    self.font_size_adjustment -= 1.;
                    ctx.style_mut(|style| {
                        style.text_styles.iter_mut().for_each(|font| {
                            let size = &mut font.1.size;
                            if *size >= 2. {
                                *size -= 1.;
                            }
                        });
                    });
                }
            });
            //ui.heading("Wordle");
            //ui.label(format!("{:?}", self.current_target));

            self.draw_letter_grid(ui);
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                if let Some(error) = &self.error_message {
                    ui.label(error);
                }

                if let Some(won) = self.game_is_won {
                    let target: String = self
                        .current_target
                        .as_ref()
                        .map(|x| x.word)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|x| x.to_ascii_uppercase().to_string())
                        .collect();
                    ui.add(
                        egui::Hyperlink::from_label_and_url(
                            target,
                            format!(
                                "https://de.wiktionary.org/wiki/{}",
                                self.current_target.as_ref().unwrap().link
                            ),
                        )
                        .open_in_new_tab(true),
                    );
                    ui.label("The hidden word was:");
                    let msg = if won { "You won!" } else { "You lost!" };
                    if ui.button("New game").clicked() {
                        self.new_game();
                    }
                    ui.label(msg);
                    ui.label("The game is over!");
                } else {
                    self.draw_letter_selection(ui);
                }
            });
        });
    }
}
