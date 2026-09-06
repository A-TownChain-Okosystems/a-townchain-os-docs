// ATC Linux Edition -- Sprint 0 Platzhalter
// Rust mit std, GUI via eframe/egui. Ziel: x86_64-unknown-linux-gnu.
// Code ist weitgehend identisch zu atc-windows-edition (egui ist cross-platform).

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("ATC Linux Edition", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ATC Linux Edition -- Sprint 0 Platzhalter");
            ui.label("Naechste View folgt: Wallet / Explorer / Dashboard (siehe README.md).");
        });
    })
}
