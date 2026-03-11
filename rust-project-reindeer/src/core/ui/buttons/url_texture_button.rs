use godot::{classes::{ITextureButton, Os, TextureButton}, prelude::*};


#[derive(GodotClass)]
#[class(init, base=TextureButton)]
pub struct URLTextureButton {
    #[export]
    #[var]
    url : GString,

    #[export]
    #[var]
    #[init(val = 1.3)]
    hover_color_multiplier : f32,


    // Non-exported

    #[var(get, set = set_is_hovered)]
    #[init(val = false)]
    is_hovered : bool,

    #[var]
    default_modulate : Color,

    
    base : Base<TextureButton>
}


#[godot_api]
impl ITextureButton for URLTextureButton {
    fn ready(&mut self) {
        let gd = self.to_gd();

        // on_pressed
        self
            .base_mut()
            .signals()
            .pressed()
            .connect_other(
                &gd,
                Self::on_pressed
            );
        
        // mouse_entered
        self
            .base_mut()
            .signals()
            .mouse_entered()
            .connect_other(
                &gd,
                Self::on_hover_started
            );
        
        // mouse_exited
        self
            .base_mut()
            .signals()
            .mouse_exited()
            .connect_other(
                &gd,
                Self::on_hover_ended
            );

        self.default_modulate = self.base().get_modulate();
        
        self.refresh();
    }
}


#[godot_api]
impl URLTextureButton {
    #[func]
    fn set_is_hovered(&mut self, is_hovered : bool) {
        // Set
        self.is_hovered = is_hovered;

        let color = {
            let mut color = self.default_modulate;
            if is_hovered {
                color *= self.hover_color_multiplier;
            }

            color
        };

        self.base_mut().set_modulate(color);
    }


    #[func]
    fn on_pressed(&mut self) {
        let mut os = Os::singleton();

        let url = &self.url;
        if !self.url.is_empty() {
            os.shell_open(url);
        }
    }


    #[func]
    fn on_hover_started(&mut self) {
        self.set_is_hovered(true);
    }


    #[func]
    fn on_hover_ended(&mut self) {
        self.set_is_hovered(false);
    }    


    fn refresh(&mut self) {
        let hovered = self.is_hovered;
        self.set_is_hovered(hovered);
    }


    pub fn reset(&mut self) {
        self.on_hover_ended();
    }
}
