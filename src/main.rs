use eframe::egui;

fn main() -> eframe::Result<()> {
    let app_id = "nl.one.winger.calc";
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id(app_id)
            .with_inner_size([320.0, 490.0])
            .with_min_inner_size([300.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Fancy Calculator",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(Calculator::new()))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals.window_fill = egui::Color32::from_rgb(26, 26, 26);
    style.visuals.panel_fill = egui::Color32::from_rgb(26, 26, 26);
    ctx.set_style(style);
}

struct Calculator {
    current_value: String,
    total_value: f64,
    input_value: bool,
    check_sum: bool,
    op: String,
    previous_text: String,
}

impl Calculator {
    fn new() -> Self {
        Self {
            current_value: "0".to_string(),
            total_value: 0.0,
            input_value: true,
            check_sum: false,
            op: String::new(),
            previous_text: String::new(),
        }
    }

    fn number_enter(&mut self, num: char) {
        if self.input_value {
            self.current_value = num.to_string();
            self.input_value = false;
        } else {
            if num == '0' && self.current_value == "0" {
                return;
            }
            self.current_value.push(num);
        }
    }

    fn add_decimal(&mut self) {
        if self.input_value {
            self.current_value = "0.".to_string();
            self.input_value = false;
        } else if !self.current_value.contains('.') {
            self.current_value.push('.');
        }
    }

    fn math_operation(&mut self, op: char) {
        if self.check_sum {
            self.sum_of_total();
        }
        self.total_value = self.current_value.parse::<f64>().unwrap_or(0.0);
        self.input_value = true;
        self.check_sum = true;
        self.op = op.to_string();
        self.previous_text = format!("{} {}", self.total_value, op);
    }

    fn sum_of_total(&mut self) {
        if self.check_sum {
            if let Ok(current) = self.current_value.parse::<f64>() {
                match self.op.as_str() {
                    "+" => self.total_value += current,
                    "-" => self.total_value -= current,
                    "×" => self.total_value *= current,
                    "÷" => {
                        if current == 0.0 {
                            self.clear_all();
                            self.current_value = "Error".to_string();
                            return;
                        }
                        self.total_value /= current;
                    }
                    _ => {}
                }
                self.current_value = self.format_to_2_decimals(self.total_value);
                self.check_sum = false;
                self.input_value = true;
                self.previous_text = String::new();
            }
        }
    }

    fn format_to_2_decimals(&self, value: f64) -> String {
        let formatted = format!("{:.2}", value);
        // Remove trailing zeros after decimal point
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
        trimmed.to_string()
    }

    fn sign_change(&mut self) {
        if let Ok(val) = self.current_value.parse::<f64>() {
            self.current_value = self.format_to_2_decimals(-val);
        }
    }

    fn percent(&mut self) {
        if let Ok(val) = self.current_value.parse::<f64>() {
            self.current_value = self.format_to_2_decimals(val / 100.0);
        }
    }

    fn clear_all(&mut self) {
        self.current_value = "0".to_string();
        self.total_value = 0.0;
        self.input_value = true;
        self.check_sum = false;
        self.op = String::new();
        self.previous_text = String::new();
    }

    fn update_display(&mut self) {
        if self.current_value.len() > 12 {
            if let Ok(val) = self.current_value.parse::<f64>() {
                // Use scientific notation for very large numbers
                if val.abs() >= 1e12 {
                    self.current_value = format!("{:.2e}", val);
                } else {
                    self.current_value = self.format_to_2_decimals(val);
                }
            }
        }
    }
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.vertical(|ui: &mut egui::Ui| {
                ui.add_space(20.0);

                // Previous calculation display
                ui.label(
                    egui::RichText::new(&self.previous_text)
                        .size(18.0)
                        .color(egui::Color32::from_rgb(136, 136, 136)),
                );

                ui.add_space(10.0);

                // Main display
                ui.label(
                    egui::RichText::new(&self.current_value)
                        .size(56.0)
                        .color(egui::Color32::WHITE)
                        .strong(),
                );

                ui.add_space(20.0);

                // Button grid using vertical layout with horizontal rows
                let button_size = egui::vec2(70.0, 60.0);
                let spacing = 8.0;
                let row_spacing = 8.0;

                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, row_spacing);

                    // Row 1: C, ±, %, ÷
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(spacing, 0.0);
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("C").size(22.0).strong(),
                                    egui::Color32::from_rgb(80, 80, 80),
                                ),
                            )
                            .clicked()
                        {
                            self.clear_all();
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("±").size(22.0).strong(),
                                    egui::Color32::from_rgb(80, 80, 80),
                                ),
                            )
                            .clicked()
                        {
                            self.sign_change();
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("%").size(22.0).strong(),
                                    egui::Color32::from_rgb(80, 80, 80),
                                ),
                            )
                            .clicked()
                        {
                            self.percent();
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("÷")
                                        .size(22.0)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                    egui::Color32::from_rgb(255, 149, 0),
                                ),
                            )
                            .clicked()
                        {
                            self.math_operation('÷');
                        }
                    });

                    // Row 2: 7, 8, 9, ×
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(spacing, 0.0);
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("7").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('7');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("8").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('8');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("9").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('9');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("×")
                                        .size(22.0)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                    egui::Color32::from_rgb(255, 149, 0),
                                ),
                            )
                            .clicked()
                        {
                            self.math_operation('×');
                        }
                    });

                    // Row 3: 4, 5, 6, -
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(spacing, 0.0);
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("4").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('4');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("5").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('5');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("6").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('6');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("-")
                                        .size(22.0)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                    egui::Color32::from_rgb(255, 149, 0),
                                ),
                            )
                            .clicked()
                        {
                            self.math_operation('-');
                        }
                    });

                    // Row 4: 1, 2, 3, +
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(spacing, 0.0);
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("1").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('1');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("2").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('2');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("3").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('3');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("+")
                                        .size(22.0)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                    egui::Color32::from_rgb(255, 149, 0),
                                ),
                            )
                            .clicked()
                        {
                            self.math_operation('+');
                        }
                    });

                    // Row 5: 0 (span 2), ., =
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(spacing, 0.0);
                        let zero_button_width = button_size.x * 2.0 + spacing;
                        if ui
                            .add_sized(
                                [zero_button_width, button_size.y],
                                button_style(
                                    egui::RichText::new("0").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.number_enter('0');
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new(".").size(22.0).strong(),
                                    egui::Color32::from_rgb(45, 45, 45),
                                ),
                            )
                            .clicked()
                        {
                            self.add_decimal();
                        }
                        if ui
                            .add_sized(
                                button_size,
                                button_style(
                                    egui::RichText::new("=")
                                        .size(22.0)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                    egui::Color32::from_rgb(255, 149, 0),
                                ),
                            )
                            .clicked()
                        {
                            self.sum_of_total();
                        }
                    });
                });
            });

            self.update_display();
            ctx.request_repaint();
        });
    }
}

fn button_style(text: egui::RichText, bg_color: egui::Color32) -> egui::Button<'static> {
    egui::Button::new(text)
        .fill(bg_color)
        .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT))
}
