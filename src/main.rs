use egui::Style;
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
}

impl TabViewer {
    fn new() -> TabViewer {
        Self {
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
