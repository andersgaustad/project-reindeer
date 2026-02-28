use godot::{classes::{Button, Control, IControl, RichTextLabel, TextEdit}, prelude::*};

use crate::core::{maze::maze::{Maze, NewMazeError}, ui::main_menu::i_main_menu_sub_menu::IMainMenuSubMenu};


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
        
        
        self.default_feedback_text = self.feedback_text.get_text();

        self.on_text_changed();
    }
}


#[godot_dyn]
impl IMainMenuSubMenu for LoadMapMenu {
    fn reset(&mut self) {
        self.maze_text_edit.clear();
        self.feedback_text.set_text(&self.default_feedback_text);
    }
}


#[godot_api]
impl LoadMapMenu {
    #[signal]
    pub fn notify_maze_created(maze : Gd<Maze>);

    #[signal]
    pub fn request_cancel();



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
            Ok(ok) => {
                feedback_text.set_text(
                    &format!(
                        "[color=#{}]Maze succesfully loaded![/color]",
                        self.ok_color.to_html()
                    )
                );

                self
                    .signals()
                    .notify_maze_created()
                    .emit(&ok);
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
