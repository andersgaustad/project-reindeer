pub trait IState {
    // May override

    fn do_enter(&mut self) {}
    fn do_exit(&mut self) {}


    // Helper functions - may override

    fn enter(&mut self) {
        self.do_enter();
    }

    fn exit(&mut self) {
        self.do_exit();
    }
}
