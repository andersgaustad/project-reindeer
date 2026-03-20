use godot::{classes::{Control, IControl, object::ConnectFlags}, prelude::*};
use strum::{EnumCount, VariantArray};

use crate::core::ui::{controls_menu::{controls_menu::ControlsMenu, controls_menu_request::ControlsMenuRequest}, i_sub_menu_state::IState, letter_menu::{letter_menu::LetterMenu, letter_menu_request::LetterMenuRequest}, options_menu::{options_menu::OptionsMenu, options_menu_request::OptionsMenuRequest}, pause_menu::{pause_menu_face::PauseMenuFace, pause_menu_face_request::PauseMenuFaceRequest, pause_menu_request::PauseMenuRequest, pause_menu_state::PauseMenuState}};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct PauseMenuStateMachine {
    #[var]
    #[init(node = "%PauseMenuFace")]
    face_pause_menu : OnReady<Gd<PauseMenuFace>>,

    #[var]
    #[init(node = "%LetterMenu")]
    letter_menu : OnReady<Gd<LetterMenu>>,

    #[var]
    #[init(node = "%OptionsMenu")]
    options_menu : OnReady<Gd<OptionsMenu>>,

    #[var]
    #[init(node = "%ControlsMenu")]
    controls_menu : OnReady<Gd<ControlsMenu>>,

    #[var(get, set = set_state)]
    #[init(val = PauseMenuState::Face)]
    state : PauseMenuState,


    #[init(val = false)]
    is_active : bool,

    
    base : Base<Control>,
}


#[godot_api]
impl IControl for PauseMenuStateMachine {
    fn ready(&mut self) {
        // Signals
        
        // face_pause_menu
        self
            .face_pause_menu
            .signals()
            .request()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
                Self::on_pause_menu_face_request
            );
        
        self
            .letter_menu
            .signals()
            .request()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(self,
                Self::on_letter_menu_request
            );
        
        // options_menu
        self
            .options_menu
            .signals()
            .request()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
                Self::on_options_menu_request
            );
        
        // controls_menu
        self
            .controls_menu
            .signals()
            .request()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
                Self::on_controls_menu_request
            );
        

        // Make it so face knows of letter menu
        self.face_pause_menu.bind_mut().set_letter_menu(Some(self.letter_menu.clone()));

        self.refresh();
    }
}


#[godot_dyn]
impl IState for PauseMenuStateMachine {
    fn do_enter(&mut self) {
        self.is_active = true;
        self.base_mut().show();
    }


    fn do_exit(&mut self) {
        self.is_active = false;
        self.exit_all_substates();
        self.base_mut().hide();
    }
}


#[godot_api]
impl PauseMenuStateMachine {
    #[signal]
    pub fn request(request : PauseMenuRequest);


    #[func]
    pub fn set_state(&mut self, new_state : PauseMenuState) {
        // Set
        self.state = new_state;

        if !self.base().is_node_ready() {
            return;
        }

        self.exit_all_substates();

        if self.is_active {
            let mut active_submenu = self.get_submenu_control(new_state);
            active_submenu.dyn_bind_mut().do_enter();
            active_submenu.show();
        }
        
    }


    #[func]
    fn on_pause_menu_face_request(&mut self, request : PauseMenuFaceRequest) {
        match request {
            PauseMenuFaceRequest::Start => self.signals().request().emit(PauseMenuRequest::Start),
            PauseMenuFaceRequest::Resume => self.signals().request().emit(PauseMenuRequest::Resume),
            PauseMenuFaceRequest::ToMainMenu => self.signals().request().emit(PauseMenuRequest::ToMainMenu),
            
            PauseMenuFaceRequest::ToMail => self.set_state(PauseMenuState::Mail),
            PauseMenuFaceRequest::ToOptions => self.set_state(PauseMenuState::Options),
            PauseMenuFaceRequest::ToControls => self.set_state(PauseMenuState::Controls),
        }
    }


    #[func]
    fn on_letter_menu_request(&mut self, request : LetterMenuRequest) {
        match request {
            LetterMenuRequest::Back => self.set_state(PauseMenuState::Face),
        }
    }


    #[func]
    fn on_options_menu_request(&mut self, request : OptionsMenuRequest) {
        match request {
            OptionsMenuRequest::Exit => self.set_state(PauseMenuState::Face),
        }
    }


    #[func]
    fn on_controls_menu_request(&mut self, request : ControlsMenuRequest) {
        match request {
            ControlsMenuRequest::Back => self.set_state(PauseMenuState::Face),
        }
    }


    pub fn refresh(&mut self) {
        let current_state = self.state;
        self.set_state(current_state);
    }


    fn exit_all_substates(&mut self) {
        let all_submenus = self.get_all_submenu_controls();

        for mut submenu in all_submenus {
            submenu.dyn_bind_mut().do_exit();
            submenu.hide();
        }

    }


    fn get_submenu_control(&self, state : PauseMenuState) -> DynGd<Control, dyn IState> {
        match state {
            PauseMenuState::Face => self.face_pause_menu.clone().into_dyn().upcast(),
            PauseMenuState::Mail => self.letter_menu.clone().into_dyn().upcast(),
            PauseMenuState::Options => self.options_menu.clone().into_dyn().upcast(),
            PauseMenuState::Controls => self.controls_menu.clone().into_dyn().upcast(),
        }
    }


    fn get_all_submenu_controls(&self) -> [DynGd<Control, dyn IState>; PauseMenuState::COUNT] {
        let states : &[PauseMenuState; PauseMenuState::COUNT] = PauseMenuState::VARIANTS.try_into().unwrap();

        states.map(|state| {
            self.get_submenu_control(state)
        })
    }


    pub fn send_mail_to_letter_menu(&mut self, mail : GString) {
        self.letter_menu.bind_mut().send_mail(mail);
    }
}
