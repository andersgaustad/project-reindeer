use godot::{classes::{Button, Control, IControl, RichTextLabel, TextEdit, Texture2D}, prelude::*};

use crate::core::{levels::main_level::main_level_constructor_info::MainLevelConstructorInfo, maze::maze::{Maze, NewMazeError}, ui::i_sub_menu_state::ISubMenuState};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct LoadMapMenu {
    #[export_group(name = "Colors")]

    #[export]
    #[var]
    #[init(val = Color::from_html("#0B6623").unwrap())]
    ok_color : Color,

    #[export]
    #[var]
    #[init(val = Color::from_html("#FF2400").unwrap())]
    error_color : Color,


    #[export_group(name = "Advanced Load Map Options")]

    #[export]
    #[var]
    open_advanced_options_button_texture : OnEditor<Gd<Texture2D>>,

    #[export]
    #[var]
    close_advanced_options_button_texture : OnEditor<Gd<Texture2D>>,


    // Non-exported

    #[var(get, set = set_show_advanced_options)]
    #[init(val = false)]
    show_advanced_options : bool,

    #[var]
    #[init(node = "%MazeTextEdit")]
    maze_text_edit : OnReady<Gd<TextEdit>>,

    #[var]
    #[init(node = "%LoadButton")]
    load_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%CancelButton")]
    cancel_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%FeedbackText")]
    feedback_text : OnReady<Gd<RichTextLabel>>,
    default_feedback_text : GString,

    #[var]
    #[init(node = "%ExpandAdvancedOptionsButton")]
    expand_advanced_options_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%AdvancedOptions")]
    advanced_options : OnReady<Gd<Control>>,
    
    #[var]
    #[init(node = "%SeedTextEdit")]
    seed_text_edit : OnReady<Gd<TextEdit>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for LoadMapMenu {
    fn ready(&mut self) {
        // Signals

        // On text change:
        self
            .maze_text_edit
            .signals()
            .text_changed()
            .connect_other(
                self,
                Self::on_text_changed
            );
        
        // On load
        self
            .load_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::try_load_text_or_default
            );

        // On cancel
        self
            .cancel_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_cancel_pressed
            );
        
        // On advanced options
        self
            .expand_advanced_options_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_show_or_hide_advanced_options_pressed
            );
        
        self.default_feedback_text = self.feedback_text.get_text();

        self.refresh();
    }
}


#[godot_dyn]
impl ISubMenuState for LoadMapMenu {
    fn enter(&mut self) {
        self.maze_text_edit.grab_focus();
    }

    fn reset(&mut self) {
        self.maze_text_edit.clear();
        self.feedback_text.set_text(&self.default_feedback_text);

        self.set_show_advanced_options(false);
        self.seed_text_edit.clear();
    }
}


#[godot_api]
impl LoadMapMenu {
    #[signal]
    pub fn notify_maze_created(info : Gd<MainLevelConstructorInfo>);

    #[signal]
    pub fn request_cancel();


    #[func]
    pub fn set_show_advanced_options(&mut self, show_advanced_options : bool) {
        // Set
        self.show_advanced_options = show_advanced_options;

        let texture = if show_advanced_options {
            self.close_advanced_options_button_texture.clone()

        } else {
            self.open_advanced_options_button_texture.clone()
        };
        
        self.expand_advanced_options_button.set_button_icon(&texture);

        self.advanced_options.set_visible(show_advanced_options);
    }


    fn on_text_changed(&mut self) {
        let empty_text_field = self.text_is_empty();

        let load_text = if empty_text_field {
            "Load Default"

        } else {
            "Load Maze"
        };

        self.load_button.set_text(load_text);
    }


    fn on_cancel_pressed(&mut self) {
        self
            .signals()
            .request_cancel()
            .emit();
    }


    fn on_show_or_hide_advanced_options_pressed(&mut self) {
        let showing_advanced_options = self.show_advanced_options;
        self.set_show_advanced_options(!showing_advanced_options);
    }


    fn refresh(&mut self) {
        let show_advanced_options = self.show_advanced_options;
        self.set_show_advanced_options(show_advanced_options);

        self.on_text_changed();

    }


    fn try_load_text_or_default(&mut self) {
        let text_is_empty = self.text_is_empty();
        let maze_text_edit = &mut self.maze_text_edit;
        if text_is_empty {
            let placeholder_text = maze_text_edit.get_placeholder();
            maze_text_edit.set_text(&placeholder_text);
        }

        let text = maze_text_edit.get_text().to_string();

        let maze_result = Maze::try_new_gd_from_str(&text);
        let feedback_text = &mut self.feedback_text;
        match maze_result {
            Ok(maze) => {
                feedback_text.set_text(
                    &format!(
                        "[color=#{}]Maze succesfully loaded![/color]",
                        self.ok_color.to_html()
                    )
                );

                let raw_seed_input_text = self.seed_text_edit.get_text();
                let seed = if !raw_seed_input_text.is_empty() {
                    raw_seed_input_text
                } else {
                    self.seed_text_edit.get_tooltip_text()
                };

                let info = MainLevelConstructorInfo::new(
                    maze,
                    seed
                );

                self
                    .signals()
                    .notify_maze_created()
                    .emit(&info);
            },

            Err(e) => {
                let NewMazeError {
                    error,
                    line_and_column_index_opt
                } = e;

                feedback_text.set_text(
                    &format!(
                        "[color=#{}]Error: {}[/color]",
                        self.error_color.to_html(),
                        &error
                    )
                );

                if let Some((origin_line, origin_column)) = line_and_column_index_opt {
                    let caret_line = origin_line;
                    let caret_column = origin_column + 1;

                    self.maze_text_edit.select(origin_line, origin_column, caret_line, caret_column);
                }
            },
        }
    }


    fn text_is_empty(&self) -> bool {
        self.maze_text_edit.get_text().is_empty()
    }
}
