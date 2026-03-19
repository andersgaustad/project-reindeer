use godot::{classes::{Button, Control, IControl, InputEvent, ScrollContainer, Texture2D, object::ConnectFlags}, obj::WithBaseField, prelude::*};

use crate::{core::{audio::{i_sfx_manager::ISFXManager, sfx_entry::SFXEntry}, run::{i_has_run::IHasRun, run::Run}, ui::{controls_menu::{controls_menu_request::ControlsMenuRequest, rebind_control_row::RebindControlRow}, i_sub_menu_state::IState}, utility::node_utility}, input_map::UI_CANCEL};


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


    run : Option<Gd<Run>>,


    base : Base<Control>
}


#[godot_api]
impl IControl for ControlsMenu {
    fn ready(&mut self) {
        let gd = self.to_gd();

        self.run = node_utility::try_find_parent_of_type(gd.upcast());

        let back_button_path = self.back_button.get_path();

        let rebind_rows = self.get_rebind_control_rows();

        // Iterate over vertical neighbors
        let n_rows = rebind_rows.len();
        for i in 0..n_rows {
            let Some(row) = rebind_rows.get(i).cloned() else {
                continue;
            };

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
            
            // Focus
            let mut left_most_button = row.bind().get_rebind_button_1();
            left_most_button.set_focus_neighbor(Side::LEFT, &back_button_path);

            let mut buttons = row.bind().get_rebind_buttons();
            for button in buttons.iter() {
                let button = button.clone();
                let button_gd = button.clone();

                // scroll_container__ensure_control_visible
                button
                    .signals()
                    .focus_entered()
                    .builder()
                    .flags(ConnectFlags::DEFERRED)
                    .connect_other_gd(
                        &self.scroll_containter.clone(),
                        move |mut scroll_container| {
                            scroll_container.ensure_control_visible(&button_gd);
                        }
                    );
            }
            
            let north_neighbor_opt = (|| {
                let north_i = i.checked_sub(1)?;
                rebind_rows.get(north_i).cloned()
            })();

            let south_neighbor_opt = rebind_rows.get(i + 1).cloned();

            if let Some(north_neighbor) = north_neighbor_opt {
                let neighbor_buttons = north_neighbor.bind().get_rebind_buttons();

                for (row_button, neighbor) in buttons.iter_mut().zip(neighbor_buttons) {
                    row_button.set_focus_neighbor(Side::TOP, &neighbor.get_path());
                }
            }

            if let Some(south_neighbor) = south_neighbor_opt {
                let neighbor_buttons = south_neighbor.bind().get_rebind_buttons();

                for (row_button, neighbor) in buttons.iter_mut().zip(neighbor_buttons) {
                    row_button.set_focus_neighbor(Side::BOTTOM, &neighbor.get_path());
                }               
            }
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


    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        if event.is_action_pressed(UI_CANCEL) {
            self.on_back_pressed();
            return;
        }
    }
}


#[godot_dyn]
impl IHasRun for ControlsMenu {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


#[godot_dyn]
impl IState for ControlsMenu {
    fn do_enter(&mut self) {
        let rows = self.get_rebind_control_rows();

        for row in rows.iter() {
            let row = row.clone();
            row.into_dyn::<dyn IState>().dyn_bind_mut().do_enter();
        }

        self.base_mut().set_process_unhandled_input(true);

        self
            .scroll_containter
            .set_v_scroll(0);

        let first_row_opt = rows.first().cloned();
        if let Some(first_row) = first_row_opt {
            let mut first_button = first_row.bind().get_rebind_button_1();
            first_button.grab_focus();
        }
        
        self.base_mut().show();
    }


    fn do_exit(&mut self) {
        let rows = self.get_rebind_control_rows();

        for row in rows.into_iter() {
            row.into_dyn::<dyn IState>().dyn_bind_mut().do_exit();
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
        let mut sfx = self.get_sfx_mananger();
        sfx.play(SFXEntry::Click);

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
