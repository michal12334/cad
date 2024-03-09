pub fn build_ui() -> impl FnMut(&egui::Context) {
    |egui_ctx| {
        egui::SidePanel::left("side_panel").exact_width(183.0).show(egui_ctx, |ui| {
            ui.heading("Hello World!");
            if ui.button("Quit").clicked() {
            }
            // if ui.text_edit_singleline().changed() {
            //     torus.major_radius = str.parse().unwrap();
            // }
        });
    }
}
