use godot::{classes::{Button, Control, IControl, OptionButton, RichTextLabel, TextEdit, Texture2D}, prelude::*};

use crate::core::{levels::main_level::main_level_constructor_info::GodotMainLevelConstructorInfo, maze::maze::{Maze, NewMazeError}, ui::{i_sub_menu_state::ISubMenuState, main_menu::load_map_menu_request::LoadMapMenuRequest}};


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

    #[export]
    #[var]
    #[init(val = tree_density_id_to_density_default())]
    tree_density_id_to_density : PackedArray<f32>,

    #[export]
    #[var]
    #[init(val = outer_forest_rings_id_to_rings_default())]
    outer_forest_rings_id_to_rings : PackedArray<i32>,


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

    #[var]
    #[init(node = "%TreeDensityOptionDropDownButton")]
    tree_density_option_button : OnReady<Gd<OptionButton>>,
    default_tree_density_id : i32,

    #[var]
    #[init(node = "%ForestSizeDropDownButton")]
    forest_size_option_button : OnReady<Gd<OptionButton>>,
    default_forest_size_id : i32,
    

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
        self.default_tree_density_id = self.tree_density_option_button.get_selected_id();

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
        self.tree_density_option_button.select(self.default_tree_density_id);
        self.forest_size_option_button.select(self.default_forest_size_id);
    }
}


#[godot_api]
impl LoadMapMenu {
    #[signal]
    pub fn notify_maze_created(info : Gd<GodotMainLevelConstructorInfo>);

    #[signal]
    pub fn request(request : LoadMapMenuRequest);


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
            .request()
            .emit(LoadMapMenuRequest::Back);
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

                let tree_density = self.get_tree_density();
                let outer_forest_rings = self.get_outer_forest_rings();

                let info = GodotMainLevelConstructorInfo::new(
                    maze,
                    seed,
                    tree_density,
                    outer_forest_rings,
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


    fn get_tree_density(&self) -> f32 {
        const DEFAULT_DENSITY : f32 = 0.25;

        let selected_tree_density_id = self.tree_density_option_button.get_selected_id();
        let density_opt = self.tree_density_id_to_density.get(selected_tree_density_id as usize);

        match density_opt {
            Some(some) => some,
            None => {
                godot_error!(
                    "Got density id of {}, but density array only holds {} ids! Defaulting to {}",
                    selected_tree_density_id,
                    self.tree_density_id_to_density.len(),
                    DEFAULT_DENSITY
                );
                DEFAULT_DENSITY
            }
        }
    }


    fn get_outer_forest_rings(&self) -> i32 {
        const DEFAULT_OUTER_FOREST_RINGS : i32 = 0;

        let selected_forest_rings_id = self.forest_size_option_button.get_selected_id();
        let forest_rings_opt = self.outer_forest_rings_id_to_rings.get(selected_forest_rings_id as usize);
        match forest_rings_opt {
            Some(some) => some,
            None => {
                godot_error!(
                    "Got outer forest ring id of {}, but forest ring array only holds {} ids! Defaulting to {}",
                    selected_forest_rings_id,
                    self.outer_forest_rings_id_to_rings.len(),
                    DEFAULT_OUTER_FOREST_RINGS
                );
                DEFAULT_OUTER_FOREST_RINGS
            },
        }
    }
}


// Utility

fn tree_density_id_to_density_default() -> PackedArray<f32> {
    PackedArray::from_iter([
        0.10,
        0.25,
        1.00,
    ])
}


fn outer_forest_rings_id_to_rings_default() -> PackedArray<i32> {
    PackedArray::from_iter(0..=2)
}
