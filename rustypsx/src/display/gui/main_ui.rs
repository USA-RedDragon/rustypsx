use egui::Context;
use super::actions::GuiAction;

pub(crate) struct Gui {
    error_message: Option<String>,
    status_message: Option<String>,
    show_breakpoint_panel: bool,
    breakpoint_address_input: String,
}

impl Gui {
    pub(crate) fn new() -> Self {
        Self { 
            error_message: None,
            status_message: None,
            show_breakpoint_panel: false,
            breakpoint_address_input: String::from("0000"),
        }
    }

    /// Create the UI using egui.
    pub(crate) fn ui(&mut self, ctx: &Context, paused: bool, ps1: Option<&crate::psx::PS1>) -> (Option<GuiAction>, bool) {
        let mut action = None;
        let mut any_menu_open = false;

        self.render_debug_panels(ctx, ps1, &mut action);
        self.render_menu_bar(ctx, &mut action, &mut any_menu_open, paused);
        self.render_status_panel(ctx);
        self.render_error_panel(ctx, &mut action);
        
        (action, any_menu_open)
    }

    fn render_menu_bar(&mut self, ctx: &Context, action: &mut Option<GuiAction>, any_menu_open: &mut bool, paused: bool) {
        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    *any_menu_open = true;
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        *action = Some(GuiAction::Exit);
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Emulation", |ui| {
                    *any_menu_open = true;
                    if ui.button("Restart").clicked() {
                        *action = Some(GuiAction::Restart);
                        ui.close_menu();
                    }
                    ui.separator();
                    let pause_text = if paused { "Resume" } else { "Pause" };
                    if ui.button(pause_text).clicked() {
                        *action = Some(GuiAction::TogglePause);
                        ui.close_menu();
                    }
                });
            });
        });
    }

    fn render_status_panel(&mut self, ctx: &Context) {
        if let Some(status_msg) = &self.status_message.clone() {
            let mut clear_status = false;
            egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("✅");
                    ui.label(status_msg);
                    if ui.button("✕").clicked() {
                        clear_status = true;
                    }
                });
            });
            
            if clear_status {
                self.status_message = None;
            }
        }
    }

    fn render_error_panel(&mut self, ctx: &Context, action: &mut Option<GuiAction>) {
        if let Some(error_msg) = &self.error_message.clone() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("🚨 Emulator Crashed");
                ui.separator();
                
                ui.label("The Game Boy emulator has encountered a fatal error and has stopped running.");
                ui.label("The GUI remains open for debugging purposes.");
                
                ui.add_space(10.0);
                
                ui.label("Error Details:");
                ui.group(|ui| {
                    ui.add(egui::TextEdit::multiline(&mut error_msg.as_str())
                        .desired_width(f32::INFINITY)
                        .desired_rows(6)
                        .font(egui::TextStyle::Monospace));
                });
                
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    if ui.button("🔄 Restart Emulation").clicked() {
                        *action = Some(GuiAction::Restart);
                    }
                    
                    if ui.button("Clear Error (Debug Mode)").clicked() {
                        *action = Some(GuiAction::ClearError);
                    }
                });
            });
        }
    }

    pub(crate) fn set_error(&mut self, error_message: String) {
        self.error_message = Some(error_message);
    }

    pub(crate) fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub(crate) fn set_status(&mut self, status_message: String) {
        self.status_message = Some(status_message);
    }

    fn render_debug_panels(&mut self, ctx: &Context, ps1: Option<&crate::psx::PS1>, action: &mut Option<GuiAction>) {
        if self.show_breakpoint_panel {
            self.render_breakpoint_panel(ctx, action, ps1);
        }
    }

    fn render_breakpoint_panel(&mut self, ctx: &Context, action: &mut Option<GuiAction>, ps1: Option<&crate::psx::PS1>) {
        egui::Window::new("Breakpoint Manager")
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Breakpoints");
                ui.separator();

                // Input for new breakpoint address
                ui.horizontal(|ui| {
                    ui.label("Address:");
                    ui.add(egui::TextEdit::singleline(&mut self.breakpoint_address_input)
                        .desired_width(80.0)
                        .font(egui::TextStyle::Monospace));

                    if ui.button("Add").clicked() {
                        // Parse the address from hex string
                        if let Ok(address) = u16::from_str_radix(self.breakpoint_address_input.trim_start_matches("0x"), 16) {
                            *action = Some(GuiAction::SetBreakpoint(address));
                            self.breakpoint_address_input = String::from("0000");
                        }
                    }
                });

                ui.small("Enter address in hex format (e.g., 0100, FFAA)");
                ui.separator();

                // Display current breakpoints if we have access to emulator
                if let Some(ps1) = ps1 {
                    ui.label("Active Breakpoints:");
                    ui.separator();

                    let breakpoints: Vec<u16> = ps1.get_breakpoints().iter().cloned().collect();
                    if breakpoints.is_empty() {
                        ui.label("No breakpoints set");
                    } else {
                        // Sort breakpoints for consistent display
                        let mut sorted_breakpoints = breakpoints.clone();
                        sorted_breakpoints.sort();

                        for &address in &sorted_breakpoints {
                            ui.horizontal(|ui| {
                                ui.monospace(format!("{:04X}", address));
                                if ui.small_button("✕").clicked() {
                                    *action = Some(GuiAction::RemoveBreakpoint(address));
                                }
                            });
                        }

                        ui.separator();
                        if ui.button("Clear All").clicked() {
                            // Remove all breakpoints by sending individual remove actions
                            // We'll handle this in the main loop
                            for &address in &breakpoints {
                                *action = Some(GuiAction::RemoveBreakpoint(address));
                            }
                        }
                    }

                    ui.separator();
                    ui.small("Click ✕ to remove a breakpoint");
                } else {
                    ui.label("Game Boy not available");
                }
            });
    }
}
