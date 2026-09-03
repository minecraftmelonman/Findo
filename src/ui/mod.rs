pub mod views;

use eframe::egui;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Instant; // for the timer, not really good for anything else

// -------------------CONFIG:-------------------
const CRASH_ON_START: bool = false; // in case you dont want to use it
const DEV_MODE: bool = false; // extra print statements (if thats what you want)

pub struct FindoApp {
    pub search_query: String,
    pub matches: Vec<PathBuf>,
    pub extension_input: String,
    pub extension_query: String,
    pub target_folder: PathBuf,
    pub target_folder_input: String,
    pub search_duration: Option<f64>,
    pub search_performed: bool,
    pub is_searching: bool,
    pub rx: Option<Receiver<Vec<PathBuf>>>,
    pub start_time: Option<Instant>,
    pub total_scanned: Arc<AtomicUsize>, // atomic counter for fast background thread progress tracking
}

impl Default for FindoApp {
    fn default() -> Self {
        let default_folder = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

        Self {
            search_query: String::new(),
            extension_input: String::new(),
            extension_query: String::new(),
            matches: Vec::new(),
            target_folder: dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
            target_folder_input: default_folder.to_string_lossy().to_string(),
            search_duration: None,
            search_performed: false,
            is_searching: false,
            rx: None,
            start_time: None,
            total_scanned: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl FindoApp {
    pub fn trigger_search(&mut self) {
        // --------------------CODE:--------------------
        if CRASH_ON_START {
            let (error_code, error_msg) = (0, "Crash on start enabled");
            eprintln!("Error {}, {}", error_code, error_msg);
            return;
        }

        // clean the input
        let cleaned_search = self.search_query.trim().to_string();
        let extension_filter = if !self.extension_input.trim().is_empty() {
            self.extension_input.trim().to_string()
        } else {
            self.extension_query.trim().to_string()
        };

        if cleaned_search.is_empty() && extension_filter.is_empty() {
            return;
        }

        self.matches.clear();
        self.total_scanned.store(0, Ordering::Relaxed); // reset file counter
        self.is_searching = true;
        self.search_performed = false;
        let start = Instant::now(); // basically a stopwatch
        self.start_time = Some(start);

        let (tx, rx): (Sender<Vec<PathBuf>>, Receiver<Vec<PathBuf>>) = channel();
        self.rx = Some(rx);

        let target_folder = self.target_folder.clone();
        let counter = Arc::clone(&self.total_scanned);

        // spawn a background thread
        thread::spawn(move || {
            let matches = crate::search::search_files(&target_folder, &cleaned_search, &extension_filter, counter);
            let _ = tx.send(matches);
        });
    }

    pub fn check_search_results(&mut self, ctx: &egui::Context) {
        if self.is_searching {
            ctx.request_repaint(); // keep UI updating for smooth loading indicators

            if let Some(ref rx) = self.rx {
                if let Ok(found_matches) = rx.try_recv() {
                    self.matches = found_matches;

                    if DEV_MODE {
                        for path in &self.matches {
                            println!("Possible path: {:?}", path);
                        }
                    }

                    if let Some(start) = self.start_time {
                        let duration = start.elapsed(); // end of the stopwatch
                        self.search_duration = Some(duration.as_secs_f64());
                    }

                    self.is_searching = false;
                    self.search_performed = true;
                    self.rx = None;
                }
            }
        }
    }
}

impl eframe::App for FindoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_search_results(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            views::render_main_view(self, ui);
        });
    }
}