use std::thread;

use eframe::egui;
use egui::Ui;
use tokio::runtime::Runtime;

use crate::controllers::{get_data::query_all, manage_auth_key::url_to_key, read_write::write_or_update_all};

pub struct P5XUI {
    url: String,
    submitted: bool
}

impl P5XUI {
    pub async fn new() -> Self {
        Self {
            url: String::new(),
            submitted: false
        }
    }
}

impl eframe::App for P5XUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            show_ui(ui, &mut self.url, &mut self.submitted);
        });
    }
}

fn show_ui(
    ui: &mut Ui,
    url: &mut String,
    submitted: &mut bool
) {
    ui.vertical_centered(|ui| {
        ui.label("Enter URL Here:");
        ui.text_edit_singleline(url);
        let button_text = if *submitted {"DONE!"} else {"Submit"};
        if ui.button(button_text).clicked() {
            let clone = url.clone();
            // temp thread for async function
            let result = thread::spawn(
                move || {
                    Runtime::new().unwrap().block_on(async {
                        if clone.is_empty() {
                            query_all(None).await
                        } else {
                            query_all(url_to_key(clone)).await
                        }
                    })
                }
            );
            url.clear();
            // handle results
            match result.join() {
                Ok(outcome) => {
                    write_or_update_all(outcome);
                },
                Err(_) => {},
            }
            *submitted = true;
        }
    });
}