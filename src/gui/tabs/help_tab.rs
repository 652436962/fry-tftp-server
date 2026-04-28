use egui::Ui;

use crate::core::i18n::I18n;

pub struct HelpState {
    pub show_about: bool,
}

impl Default for HelpState {
    fn default() -> Self {
        Self::new()
    }
}

impl HelpState {
    pub fn new() -> Self {
        Self { show_about: false }
    }
}

pub fn draw(ui: &mut Ui, _help: &mut HelpState, i18n: &I18n) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading(i18n.t("help_title"));
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(i18n.t("help_subtitle"))
                .size(14.0)
                .italics(),
        );
        ui.label(format!(
            "{} {}",
            i18n.t("version"),
            env!("CARGO_PKG_VERSION")
        ));

        ui.add_space(12.0);
        ui.separator();
        ui.add_space(8.0);

        // ── Supported RFCs ──
        ui.heading(i18n.t("supported_rfcs"));
        ui.add_space(4.0);

        egui::Grid::new("rfc_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                ui.strong("RFC");
                ui.strong(i18n.t("rfc_header_title"));
                ui.strong(i18n.t("rfc_header_description"));
                ui.end_row();

                ui.label("RFC 1350");
                ui.label(i18n.t("rfc1350_title"));
                ui.label(i18n.t("rfc1350_desc"));
                ui.end_row();

                ui.label("RFC 2347");
                ui.label(i18n.t("rfc2347_title"));
                ui.label(i18n.t("rfc2347_desc"));
                ui.end_row();

                ui.label("RFC 2348");
                ui.label(i18n.t("rfc2348_title"));
                ui.label(i18n.t("rfc2348_desc"));
                ui.end_row();

                ui.label("RFC 2349");
                ui.label(i18n.t("rfc2349_title"));
                ui.label(i18n.t("rfc2349_desc"));
                ui.end_row();

                ui.label("RFC 7440");
                ui.label(i18n.t("rfc7440_title"));
                ui.label(i18n.t("rfc7440_desc"));
                ui.end_row();
            });

        ui.add_space(12.0);
        ui.separator();
        ui.add_space(8.0);

        // ── Features ──
        ui.heading(i18n.t("features"));
        ui.add_space(4.0);

        let features = [
            i18n.t("feature_gui"),
            i18n.t("feature_tui"),
            i18n.t("feature_headless"),
            i18n.t("feature_hot_reload"),
            i18n.t("feature_acl"),
            i18n.t("feature_rate_limit"),
            i18n.t("feature_mmap"),
            i18n.t("feature_sliding_window"),
            i18n.t("feature_transfer_modes"),
            i18n.t("feature_path_protection"),
            i18n.t("feature_log_rotation"),
            i18n.t("feature_tray"),
            i18n.t("feature_service_support"),
            i18n.t("feature_env_overrides"),
            i18n.t("feature_export"),
        ];
        for feat in &features {
            ui.horizontal(|ui| {
                ui.label("  -");
                ui.label(*feat);
            });
        }
    });
}
