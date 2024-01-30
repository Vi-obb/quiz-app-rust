mod quiz;

fn main() -> Result<(), eframe::Error> {
    let viewport = egui::ViewportBuilder::default().with_inner_size(egui::Vec2::new(500.0, 500.0));

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native("QuizMe", options, Box::new(|_ctx| Box::new(App::new())))
}

struct App {
    quiz: quiz::Quiz,
}

impl App {
    fn new() -> Self {
        let quiz = quiz::Quiz::sample();
        Self { quiz }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();

            style.text_styles = [
                (egui::TextStyle::Body, egui::FontId::proportional(18.0)),
                (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
            ]
            .into();

            style.spacing.button_padding = egui::Vec2::new(10.0, 5.0);

            ui.set_style(style);

            let current_index = self.quiz.current_index + 1;
            let count = self.quiz.questions.len();
            let score = self.quiz.calculate_score();

            ui.horizontal(|ui| {
                ui.label(format!("{}/{}", current_index, count));
                ui.label(format!("Score: {}", score));
            });

            // function to get the current question
            let current_question = self.quiz.current_question();
            ui.label(
                egui::RichText::new(current_question.title.as_str())
                    .font(egui::FontId::proportional(32.0)),
            );

            // Buttons
            ui.horizontal(|ui| {
                let true_button = get_button(
                    "true",
                    self.quiz.current_question().user_answer == Some(true),
                );

                let false_button = get_button(
                    "false",
                    self.quiz.current_question().user_answer == Some(false),
                );

                if ui.add(true_button).clicked() {
                    self.quiz.answer(true);
                }

                if ui.add(false_button).clicked() {
                    self.quiz.answer(false);
                };
            });

            ui.columns(2, |columns| {
                columns[0].allocate_ui_with_layout(
                    egui::Vec2 { x: 120.0, y: 40.0 },
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        if ui.button("Previous").clicked() {
                            self.quiz.prev_question();
                        }
                    },
                );
                columns[1].allocate_ui_with_layout(
                    egui::Vec2 { x: 120.0, y: 40.0 },
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        if ui.button("Next").clicked() {
                            self.quiz.next_question();
                        }
                    },
                );
            });
        });
    }
}

fn get_button(label: &str, selected: bool) -> egui::Button<'static> {
    let mut label = egui::RichText::from(label);

    if selected {
        label = label.color(egui::Color32::WHITE);
    }

    let mut button = egui::Button::new(label).min_size(egui::Vec2::new(60.0, 30.0));

    if selected {
        button = button.fill(egui::Color32::BLUE);
    }

    button
}
