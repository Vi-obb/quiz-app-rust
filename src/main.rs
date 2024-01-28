mod quiz;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Queeez", options, Box::new(|ctx| Box::new(App::new())))
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let current_index = self.quiz.current_index + 1;
            let count = self.quiz.questions.len();

            ui.label(format!("Let's take a quiz!"));

            ui.label(format!("{current_index}/{count}"));

            let current_question = self.quiz.current_question();
            ui.label(&current_question.title);

            ui.horizontal(|ui| {
                if ui.button("True").clicked() {
                    self.quiz.answer(true);
                }
                if ui.button("False").clicked() {
                    self.quiz.answer(false);
                }
            });

            

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
