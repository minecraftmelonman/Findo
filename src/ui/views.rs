use super::FindoApp;
use eframe::egui;
use std::path::PathBuf;

pub fn render_main_view(app: &mut FindoApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.heading("Findo File Search");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("</>").clicked() {
                let _ = open::that("https://github.com/minecraftmelonman/Findo");
            }
        });
    });

    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("Target Directory:");
        if ui
            .text_edit_singleline(&mut app.target_folder_input)
            .changed()
        {
            app.target_folder = PathBuf::from(&app.target_folder_input);
        }
        if ui.button("Browse..").clicked() {
            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                app.target_folder = folder;
            }
        }
    });

    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("Search:");
        let response = ui.text_edit_singleline(&mut app.search_query);

        if ui.button("Search!").clicked() {
            app.trigger_search();
        }

        if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::F)) {
            response.request_focus();
        }
    });

    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("File Extension (optional):");
        ui.text_edit_singleline(&mut app.extension_query);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if let Some(duration) = app.search_duration {
                ui.weak(format!("{:.2}s", duration));
            }
        });
    });

    ui.add_space(10.0);

    if app.is_searching {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label("Scanning directory...".to_string());
        });
    }

    ui.separator();

    let row_height = ui.text_style_height(&egui::TextStyle::Body) + 4.0;
    let total_rows = app.matches.len();

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show_rows(ui, row_height, total_rows, |ui, row_range| {
            if app.matches.is_empty() && app.search_performed && !app.is_searching {
                ui.label("No files found matching your search.");
            }

            for i in row_range {
                let path = &app.matches[i];
                let file_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                let extension_tag = path
                    .extension()
                    .map(|ext| format!(" [{}]", ext.to_string_lossy().to_uppercase()))
                    .unwrap_or_default();

                let full_label = format!("{}{}", file_name, extension_tag);

                ui.horizontal(|ui| {
                    let response = render_highlighted_label(ui, &full_label, &app.search_query);

                    response.context_menu(|ui| {
                        if ui.button("Open File").clicked() {
                            let _ = open::that(path);
                            ui.close_menu();
                        }
                        if ui.button("Open Parent Folder").clicked() {
                            if let Some(parent) = path.parent() {
                                let _ = open::that(parent);
                            }
                            ui.close_menu();
                        }
                        if ui.button("Copy Dir Path").clicked() {
                            ui.output_mut(|o| o.copied_text = path.to_string_lossy().to_string());
                            ui.close_menu();
                        }
                    });

                    ui.weak(path.parent().unwrap_or(path).to_string_lossy());
                });
            }
        });
}

fn render_highlighted_label(ui: &mut egui::Ui, text: &str, query: &str) -> egui::Response {
    let mut job = egui::text::LayoutJob::default();
    let query_trim = query.trim();

    if query_trim.is_empty() {
        job.append(
            text,
            0.0,
            egui::TextFormat {
                color: ui.visuals().text_color(),
                ..Default::default()
            },
        );
    } else {
        let text_lower = text.to_lowercase();
        let query_lower = query_trim.to_lowercase();
        let mut last_idx = 0;

        for (start_idx, _) in text_lower.match_indices(&query_lower) {
            if start_idx > last_idx {
                job.append(
                    &text[last_idx..start_idx],
                    0.0,
                    egui::TextFormat {
                        color: ui.visuals().text_color(),
                        ..Default::default()
                    },
                );
            }

            let end_idx = start_idx + query_trim.len();
            job.append(
                &text[start_idx..end_idx],
                0.0,
                egui::TextFormat {
                    color: egui::Color32::from_rgb(255, 140, 0),
                    background: egui::Color32::from_black_alpha(40),
                    ..Default::default()
                },
            );
            last_idx = end_idx;
        }

        if last_idx < text.len() {
            job.append(
                &text[last_idx..],
                0.0,
                egui::TextFormat {
                    color: ui.visuals().text_color(),
                    ..Default::default()
                },
            );
        }
    }

    ui.add(egui::Label::new(job))
}