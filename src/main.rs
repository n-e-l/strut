use std::ops::RangeInclusive;
use egui::{vec2, Color32, Pos2, Rect, Sense, Slider, Style, TextStyle, Vec2};
use egui_dock::{DockArea, DockState};
use egui_dock::tab_viewer::OnCloseResponse;
use egui_dock::NodeIndex;
use egui_dock::SurfaceIndex;
use egui::{Ui, WidgetText};
use std::sync::{Arc, Mutex};
use ash::vk::{Image, ImageView};
use cen::app::App;
use cen::app::app::AppConfig;
use cen::app::gui::{GuiComponent, GuiSystem};
use cen::graphics::Renderer;
use cen::graphics::renderer::RenderComponent;
use cen::vulkan::CommandBuffer;
use dotenv::dotenv;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use egui_extras::{Size, StripBuilder};

struct Application {
    tree: DockState<String>,
    tab_viewer: TabViewer,
}

impl Application {

    fn new() -> Application {
        Self {
            tab_viewer: TabViewer::new(),
            tree: DockState::new(vec!["main".to_string()])
        }
    }
}

struct TabViewer {
    beats: usize,
    code: String
}

impl TabViewer {
    fn new() -> TabViewer {
        Self {
            beats: 4,
            code: String::new()
        }
    }
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.as_str().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            _ => {
                ui.label(tab.as_str());
                let pixels_per_point = ui.ctx().pixels_per_point();
                let available_width = ui.available_width();

                ui.horizontal(|ui| {
                    ui.spacing_mut().slider_width = 300.0;
                    ui.add(
                        Slider::new(&mut self.beats, RangeInclusive::new(0, 90))
                    );
                });

                let width = 20.;
                let height = 100.;

                let size_pixels = vec2( ui.available_width() * pixels_per_point, height );
                let size_points = size_pixels / pixels_per_point + Vec2::splat(2.0);
                let (response, painter) = ui.allocate_painter(size_points, Sense::hover());

                let mut cursor_pixel = Pos2::new(
                    response.rect.min.x * pixels_per_point,
                    response.rect.min.y * pixels_per_point,
                )
                    .ceil();
                for i in 0..self.beats {
                    let rect_points = Rect::from_min_size(
                        Pos2::new(cursor_pixel.x, cursor_pixel.y),
                        vec2(width, height)
                    );
                    painter.rect_filled(rect_points / pixels_per_point, 0.0, Color32::WHITE);
                    cursor_pixel.x += width * 2.;
                }

                let mut code = r"
t = {}
t = { a = 1, b = 2 }
t.a = function() ... end

t = { ['hello'] = 200 }
t.hello
                ";
                let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
                egui_extras::syntax_highlighting::code_view_ui(ui, &theme, code.clone(), "rust");

                CodeEditor::default()
                    .id_source("code editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(ColorTheme::GRUVBOX)
                    .with_syntax(Syntax::rust())
                    .with_numlines(true)
                    .show(ui, &mut self.code);
            }
        }
    }

    fn context_menu(
        &mut self,
        ui: &mut Ui,
        tab: &mut Self::Tab,
        _surface: SurfaceIndex,
        _node: NodeIndex,
    ) {
        match tab.as_str() {
            // "Simple Demo" => self.simple_demo_menu(ui),
            _ => {
                ui.label(tab.to_string());
                ui.label("This is a context menu");
            }
        }
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> OnCloseResponse {
        // self.open_tabs.remove(tab);
        OnCloseResponse::Close
    }

    fn is_closeable(&self, tab: &Self::Tab) -> bool {
        ["Inspector", "Style Editor"].contains(&tab.as_str())
    }
}

impl GuiComponent for Application {

    fn initialize_gui(&mut self, gui: &mut GuiSystem) {
    }

    fn gui(&mut self, gui: &GuiSystem, ctx: &egui_dock::egui::Context) {
        DockArea::new(&mut self.tree)
            .show(ctx, &mut self.tab_viewer);
    }
}

impl RenderComponent for Application {
    fn initialize(&mut self, renderer: &mut Renderer) {
    }

    fn render(&mut self, renderer: &mut Renderer, cb: &mut CommandBuffer, i: &Image, iv: &ImageView) {
    }
}

fn main() {
    // Initialize .env environment variables
    dotenv().ok();

    let application = Arc::new(Mutex::new(Application::new()));
    App::run(
        AppConfig::default()
            .width(1180)
            .height(1180)
            .log_fps(true)
            .resizable(true)
            .vsync(false),
        application.clone(),
        Some(application.clone())
    );
}
