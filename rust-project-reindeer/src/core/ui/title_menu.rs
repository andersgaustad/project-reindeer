use godot::{classes::{Button, Control, IControl}, prelude::*};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct TitleMenu {
    #[var]
    #[init(node = "%StartButton")]
    start_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ExitButton")]
    exit_button : OnReady<Gd<Button>>,

    base : Base<Control>,
}


#[godot_api]
impl IControl for TitleMenu {
    fn ready(&mut self) { 
        // On start
        self
            .start_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_start_pressed
            );

        // On exit
        self
            .exit_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_exit_pressed
            );
    }
}


#[godot_api]
impl TitleMenu {
    #[signal]
    pub fn request_start();


    fn on_start_pressed(&mut self) {
        self
            .signals()
            .request_start()
            .emit();
    }


    fn on_exit_pressed(&mut self) {
        self.base().get_tree().unwrap().quit();
    }
}
