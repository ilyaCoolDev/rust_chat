use eframe::egui::{self, Color32, TextEdit};

#[derive(Default)]
pub struct RegisterScreen{
    pub username: String,
    pub password: String,
    pub error: Option<String>,
}

impl RegisterScreen{
    pub fn update(&mut self, ctx: &egui::Context) -> (bool, bool){
        let mut update = false;
        let mut login = false;
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Chat");
            ui.add_space(10.);
            ui.group(|ui|{
                ui.set_max_width(300.);
                if let Some(ref error) = self.error{
                    ui.colored_label(Color32::from_rgb(255, 0, 0), error);
                }
                ui.vertical(|ui|{
                    ui.heading("Register");
                    ui.add(TextEdit::singleline(&mut self.username).hint_text("Username"));
                    ui.add(TextEdit::singleline(&mut self.password).hint_text("Password").password(true));
                    if ui.button("Register").clicked(){
                        update = true;
                    }
                    if ui.button("I have an account").clicked(){
                        login = true;
                    }
                });
            })
        });
        (update, login)
    }
}