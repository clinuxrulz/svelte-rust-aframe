use crate::component_designer::ComponentDesigner;
use sodium_rust::Cell;
use sodium_rust::CellSink;
use sodium_rust::SodiumCtx;

pub struct Sketch {}

#[derive(Clone)]
pub enum AppView {
    LogInOrRegister,
    ComponentDesigner(ComponentDesigner),
}

pub struct App {
    pub sodium_ctx: SodiumCtx,
    pub c_current_view: Cell<AppView>,
    cs_current_view: CellSink<AppView>,
}

impl App {
    pub fn new() -> App {
        let sodium_ctx = SodiumCtx::new();
        let cs_current_view = sodium_ctx.new_cell_sink(AppView::LogInOrRegister);
        App {
            sodium_ctx: sodium_ctx.clone(),
            c_current_view: cs_current_view.cell(),
            cs_current_view,
        }
    }

    pub fn log_in(&mut self) {
        self.cs_current_view
            .send(AppView::ComponentDesigner(ComponentDesigner {}));
    }
}
