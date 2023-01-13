use eframe::egui::{self, Ui};
use std::{sync::mpsc::{channel, Receiver, Sender}, thread};
use crate::simulation::{SimulationState, Command};
use crate::image::{FlatImg, fit_image_size};

pub fn run(
    recv: Receiver<SimulationState<FlatImg>>,
    commands: Sender<Command>,
) {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cellular Automaton",
        options,
        Box::new(|cc| {
            Box::new(MyApp::new(cc, recv, commands))
        }),
    );
}

struct MyApp {
    receiver: Receiver<SimulationState<FlatImg>>,
    commands: Sender<Command>,
    simulation_state: SimulationState<FlatImg>,
    ui_state: UiState,
}

struct UiState {
    delay_value: u64,
    height_slider_value: usize,
    width_slider_value: usize,
}


impl MyApp {
    fn new(cc: &eframe::CreationContext, recv_simulation_state: Receiver<SimulationState<FlatImg>>, send_command: Sender<Command>) -> MyApp {
        let (send, recv) = channel();
        let ctx = cc.egui_ctx.clone();
        thread::spawn(move || {
            loop {
                let x = recv_simulation_state.recv();
                match x {
                    Ok(v) => {
                        match send.send(v) {
                            Ok(_) => {},
                            Err(_) => println!("could not forward simulation state"),
                        }
                        ctx.request_repaint();
                    },
                    Err(e) => println!("error receiving simulation state: {}", e),
                }
            }
        });

        MyApp { 
            receiver: recv,
            commands: send_command,
            // TODO instead of creating this check below if it is empty
            simulation_state: SimulationState {
                step: 0,
                running: false,
                data: FlatImg {
                    img: vec![],
                    width: 0,
                    height: 0,
                }
            },
            ui_state: UiState { delay_value: 0, height_slider_value: 10, width_slider_value: 10 }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.recv_simulation_state();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                self.build_image(ui, _frame);
                self.build_controls(ui);
            });
        });
    }
}

impl MyApp {
    fn recv_simulation_state(&mut self) {
        let ib = self.receiver.try_recv();
        match ib {
            Ok(x) => {
                self.simulation_state = x;
            },
            _ => {},
        }
    }

    fn send_command(&self, command: Command) {
        match self.commands.send(command) {
            Ok(_) => {},
            Err(_) => {
                println!("could not send command");
            },
        }
    }

    fn build_image(&self, ui: &mut Ui, frame: &eframe::Frame) {
        let flat_img = &self.simulation_state.data;
        if flat_img.width == 0 || flat_img.height == 0 {
            return;
        }

        // compute the size of the texture
        let window_size = frame.info().window_info.size;
        let wx = window_size.x;
        let wy = window_size.y - 200.0;
        // fit image to window while keeping aspect ratio
        let image_size = fit_image_size(wx, wy, flat_img.width as f32, flat_img.height as f32);

        let color_image = egui::ColorImage::from_rgb(
            [flat_img.width, flat_img.height],
            &flat_img.img,
        );
        let texture = ui.ctx().load_texture(
            "img",
            color_image,
            egui::TextureOptions::NEAREST,
        );
        // TODO do we need to scale these values for aspect ratios
        ui.image(&texture, image_size);
    }

    fn build_controls(&mut self, ui: &mut Ui) {
        ui.label(format!("Step: {}", self.simulation_state.step));
        if ui.button("Start").clicked() {
            self.send_command(Command::Start);
        };
        if ui.button("Stop").clicked() {
            self.send_command(Command::Stop);
        };
        if ui.button("Step").clicked() {
            self.send_command(Command::SingleStep);
        };
        ui.horizontal(|ui|{
            ui.vertical(|ui| {
                ui.horizontal(|ui|{
                    ui.label("Width:");
                    ui.add(egui::Slider::new(&mut self.ui_state.width_slider_value , 10..=1000));
                });
                ui.horizontal(|ui|{
                    ui.label("Height:");
                    ui.add(egui::Slider::new(&mut self.ui_state.height_slider_value , 10..=1000));
                });
            });
            if ui.button("Restart").clicked() {
                self.send_command(Command::Reset(self.ui_state.height_slider_value, self.ui_state.width_slider_value));
            };
        });
        ui.horizontal(|ui| {
            ui.label("Delay:");
            ui.add(egui::Slider::new(&mut self.ui_state.delay_value , 0..=2000));
            if ui.button("Set").clicked() {
                self.send_command(Command::ChangeDelay(std::time::Duration::from_millis(self.ui_state.delay_value)));
            };
        });
    }
}