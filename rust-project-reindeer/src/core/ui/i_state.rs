pub trait IState {
    // May override

    fn enter(&mut self) {}
    fn exit(&mut self) {}
}
