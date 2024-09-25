use test_engine::{
    refs::Weak,
    ui::{view, HasText, Label, Setup, ViewData},
};

#[view]
pub struct TodoView {
    #[init]
    label: Label,
}

impl Setup for TodoView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("TODO").place().size(100, 50).center();
    }
}
