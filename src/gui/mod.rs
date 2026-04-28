#[cfg(feature = "gui")]
pub mod app;
#[cfg(feature = "gui")]
pub mod log_layer;
#[cfg(feature = "gui")]
pub mod tabs;
#[cfg(feature = "gui")]
pub mod theme;
#[cfg(feature = "gui")]
pub mod tray;

#[cfg(feature = "gui")]
fn load_window_icon() -> egui::IconData {
    let png_bytes = include_bytes!("app_icon_256.png");
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    let mut reader = decoder.read_info().expect("failed to read app icon PNG");
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buf)
        .expect("failed to decode app icon PNG");
    let raw = &buf[..info.buffer_size()];

    let rgba = match info.color_type {
        png::ColorType::Rgba => raw.to_vec(),
        png::ColorType::Rgb => {
            let mut out = Vec::with_capacity((info.width * info.height * 4) as usize);
            for chunk in raw.chunks(3) {
                out.extend_from_slice(chunk);
                out.push(255);
            }
            out
        }
        _ => raw.to_vec(),
    };

    egui::IconData {
        rgba,
        width: info.width,
        height: info.height,
    }
}

#[cfg(feature = "gui")]
pub async fn run(
    state: std::sync::Arc<crate::core::state::AppState>,
    log_buffer: log_layer::LogBuffer,
) -> anyhow::Result<()> {
    use crate::platform;

    // Register platform-specific signal handlers
    platform::register_signals(state.get_shutdown_token(), Some(state.clone())).await;

    // Spawn the TFTP server in background
    let server_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = crate::core::run_server(server_state.clone()).await {
            tracing::error!(error = %e, "server error");
            server_state.set_server_state(crate::core::state::ServerState::Error);
        }
    });

    let app_state_for_close = state.clone();

    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([1000.0, 600.0])
        .with_min_inner_size([800.0, 500.0]);

    #[cfg(target_os = "macos")]
    let viewport = viewport.with_icon(std::sync::Arc::new(egui::IconData::default()));

    #[cfg(not(target_os = "macos"))]
    let viewport = viewport.with_icon(std::sync::Arc::new(load_window_icon()));

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    let result = tokio::task::block_in_place(|| {
        eframe::run_native(
            "RustTFTP",
            options,
            Box::new(move |cc| {
                // Create the tray after the native app has initialized its GUI backend.
                let initial_i18n = crate::core::i18n::I18n::new(crate::core::i18n::Lang::parse(
                    &state.config().gui.language,
                ));
                let tray_state = tray::create_tray(&initial_i18n).ok();
                Ok(Box::new(app::TftpApp::new(cc, state, log_buffer, tray_state)))
            }),
        )
        .map_err(|e| anyhow::anyhow!("eframe error: {}", e))
    });

    // Window closed — shut down server
    app_state_for_close.cancel_shutdown();
    result
}
