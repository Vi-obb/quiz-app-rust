mod quiz;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
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
            let current_index = self.quiz.current_index + 1;
            let count = self.quiz.questions.len();
            let score = self.quiz.calculate_score();

            ui.horizontal(|ui| {
                ui.label(format!("{}/{}", current_index, count));
                ui.label(format!("Score: {}", score));
            });

            // function to get the current question
            let current_question = self.quiz.current_question();
            ui.label(&current_question.title);

            // Buttons
            ui.horizontal(|ui| {
                if ui.button("True").clicked() {
                    self.quiz.answer(true);
                }
                if ui.button("False").clicked() {
                    self.quiz.answer(false);
                }

                let current_question = self.quiz.current_question();
                if let Some(_user_answer) = current_question.user_answer {
                    if current_question.is_correct() {
                        ui.label("Correct!");
                    } else {
                        ui.label("Incorrect!");
                    }
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Previous").clicked() {
                    self.quiz.prev_question();
                }
                if ui.button("Next").clicked() {
                    self.quiz.next_question();
                }
            });
            // Question count and score

            // egui::CentralPanel::default().show(ctx, |ui| {
            //     ui.label("1/10");
            //     ui.label("What is the capital of Nigeria?");

            //     ui.horizontal(|ui| {
            //         ui.button("True");
            //         ui.button("False");
            //     });

            //     ui.horizontal(|ui| {
            //         ui.button("Previous");
            //         ui.button("Next");
            //     });
            // });
        });
    }
}
