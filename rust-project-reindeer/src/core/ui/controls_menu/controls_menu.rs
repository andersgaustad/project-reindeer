use godot::{classes::{Button, Control, IControl, ScrollContainer, Texture2D, object::ConnectFlags}, obj::WithBaseField, prelude::*};

use crate::core::ui::{controls_menu::{controls_menu_request::ControlsMenuRequest, rebind_control_row::RebindControlRow}, i_sub_menu_state::IState};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct ControlsMenu {
    #[export_group(name = "Button Defaults")]

    #[export]
    #[var]
    #[init(val = "<Unassigned>".into())]
    default_button_text : GString,

    #[export]
    #[var]
    default_button_icon : Option<Gd<Texture2D>>,


    // Non-exported

    #[var]
    #[init(node = "%ScrollContainer")]
    scroll_containter : OnReady<Gd<ScrollContainer>>,


    #[var]
    #[init(node = "%MoveForwardRebind")]
    move_forward_rebind : OnReady<Gd<RebindControlRow>>,

    #[var]
    #[init(node = "%MoveLeftRebind")]
    move_left_rebind : OnReady<Gd<RebindControlRow>>,

    #[var]
    #[init(node = "%MoveBackRebind")]
    move_back_rebind : OnReady<Gd<RebindControlRow>>,

    #[var]
    #[init(node = "%MoveRightRebind")]
    move_right_rebind : OnReady<Gd<RebindControlRow>>,

    #[var]
    #[init(node = "%ToggleSprintRebind")]
    toggle_sprint_rebind : OnReady<Gd<RebindControlRow>>,

    #[var]
    #[init(node = "%MoveUpRebind")]
    move_up_rebind : OnReady<Gd<RebindControlRow>>,

    #[var]
    #[init(node = "%MoveDownRebind")]
    move_down_rebind : OnReady<Gd<RebindControlRow>>,


    #[var]
    #[init(node = "%ShowBodyRebind")]
    show_body_rebind : OnReady<Gd<RebindControlRow>>,

    #[var]
    #[init(node = "%ToggleLightRebind")]
    toggle_light_rebind : OnReady<Gd<RebindControlRow>>,


    #[var]
    #[init(node = "%BackButton")]
    back_button : OnReady<Gd<Button>>,


    base : Base<Control>
}


#[godot_api]
impl IControl for ControlsMenu {
    fn ready(&mut self) {
        let mut rebind_rows = self.get_rebind_control_rows();
        for row in rebind_rows.iter_mut() {
            // notify_waiting_for_input
            let row_gd = row.clone();
            row
                .signals()
                .notify_waiting_for_input()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_mut(
                    self,
                    move |me| {
                        me.on_row_notify_waiting_for_input(row_gd.clone());
                    }
                );
            
            // notify_finished_rebinding
            let row_gd = row.clone();
            row
                .signals()
                .notify_finished_rebinding()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_mut(
                    self,
                    move |me| {
                        me.on_row_notify_finished_rebinding(row_gd.clone());
                    }
                );
        }

        // back_button
        self
            .back_button
            .signals()
            .pressed()
            .connect_other(
                self,  
                Self::on_back_pressed
            );

        self.refresh();
    }
}


#[godot_dyn]
impl IState for ControlsMenu {
    fn do_enter(&mut self) {
        let rows = self.get_rebind_control_rows();

        for row in rows.into_iter() {
            row.into_dyn().dyn_bind_mut().do_enter();
        }

        self.base_mut().set_process_unhandled_input(true);

        self.back_button.grab_focus();

        self
            .scroll_containter
            .set_v_scroll(0);
        
        self.base_mut().show();
    }


    fn do_exit(&mut self) {
        let rows = self.get_rebind_control_rows();

        for row in rows.into_iter() {
            row.into_dyn().dyn_bind_mut().do_exit();
        }

        self.base_mut().set_process_unhandled_input(false);
        self.base_mut().hide();
    }
}


#[godot_api]
impl ControlsMenu {
    #[signal]
    pub fn request(request : ControlsMenuRequest);


    #[func]
    fn on_row_notify_waiting_for_input(&mut self, row : Gd<RebindControlRow>) {
        let mut rows = self.get_rebind_control_rows();

        for other_row in rows.iter_mut() {
            if row == *other_row {
                continue;
            }

            other_row.bind_mut().on_become_overshadowed();
        }
    }


    #[func]
    fn on_row_notify_finished_rebinding(&mut self, row : Gd<RebindControlRow>) {
        let mut rows = self.get_rebind_control_rows();

        for other_row in rows.iter_mut() {
            if row == *other_row {
                continue;
            }

            other_row.bind_mut().on_release_overshadowed();
        }
    } 


    #[func]
    fn on_back_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(ControlsMenuRequest::Back);
    }


    fn refresh(&mut self) {
        self.update_ui();
    }


    fn update_ui(&mut self) {
        let mut rows = self.get_rebind_control_rows();
        for row in rows.iter_mut() {
            row.bind_mut().update_ui_with_bindings();
        }
    }


    fn get_rebind_control_rows(&self) -> [Gd<RebindControlRow>; 9] {
        [
            self.move_forward_rebind.clone(),
            self.move_left_rebind.clone(),
            self.move_back_rebind.clone(),
            self.move_right_rebind.clone(),
            self.toggle_sprint_rebind.clone(),
            self.move_up_rebind.clone(),
            self.move_down_rebind.clone(),
            self.show_body_rebind.clone(),
            self.toggle_light_rebind.clone(),
        ]
    }
}
