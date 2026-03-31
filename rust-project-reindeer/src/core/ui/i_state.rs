pub trait IState {
    fn enter(&mut self) {}
    fn exit(&mut self) {}
}
