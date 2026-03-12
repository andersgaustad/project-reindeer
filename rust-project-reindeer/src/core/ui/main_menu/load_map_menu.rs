use godot::{classes::{Button, ColorPickerButton, Control, HSlider, IControl, OptionButton, RichTextLabel, SpinBox, TextEdit, Texture2D, object::ConnectFlags}, prelude::*};

use crate::core::{levels::main_level::main_level_constructor_info::{GodotMainLevelConstructorInfo, MainLevelConstructorInfo}, maze::maze::{Maze, NewMazeError}, ui::{i_sub_menu_state::IState, main_menu::load_map_menu_request::LoadMapMenuRequest}};


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

    #[var]
    #[init(node = "%TurningCostSpinBox")]
    turn_cost_spin_box : OnReady<Gd<SpinBox>>,

    #[var]
    #[init(node = "%TurningCostSlider")]
    turn_cost_slider : OnReady<Gd<HSlider>>,

    #[var]
    #[init(node = "%ColorAPickerButton")]
    color_a_picker : OnReady<Gd<ColorPickerButton>>,

    #[var]
    #[init(node = "%ColorBPickerButton")]
    color_b_picker : OnReady<Gd<ColorPickerButton>>,

    #[var(get, set = set_turn_cost)]
    turn_cost : u32,
    default_turn_cost : u32,
    

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
                Self::on_maze_text_changed
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
        
        // Turn cost - Text change
        self
            .turn_cost_spin_box
            .signals()
            .value_changed()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
                Self::on_turn_cost_spin_box_value_changed
            );
        
        // Turn cost - Slider change
        self
            .turn_cost_slider
            .signals()
            .value_changed()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
                Self::on_turn_cost_slider_value_changed
            );

        self.default_feedback_text = self.feedback_text.get_text();
        self.default_tree_density_id = self.tree_density_option_button.get_selected_id();

        let set_turn_cost = self.turn_cost_slider.get_value() as u32;
        self.turn_cost = set_turn_cost;
        self.default_turn_cost = set_turn_cost;

        self.refresh();
    }
}


#[godot_dyn]
impl IState for LoadMapMenu {
    fn do_enter(&mut self) {
        self.maze_text_edit.grab_focus();

        self.refresh();
    }
    

    fn do_exit(&mut self) {
        self.maze_text_edit.clear();
        self.feedback_text.set_text(&self.default_feedback_text);

        self.set_show_advanced_options(false);
        self.seed_text_edit.clear();
        self.tree_density_option_button.select(self.default_tree_density_id);
        self.forest_size_option_button.select(self.default_forest_size_id);

        self.set_turn_cost(self.default_turn_cost);
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


    #[func]
    pub fn set_turn_cost(&mut self, turn_cost : u32) {
        // Set
        self.turn_cost = turn_cost;

        if !self.base().is_node_ready() {
            return;
        }

        let turn_cost_f64 = turn_cost as f64;

        // Slider
        let slider = &mut self.turn_cost_slider;
        let value = slider.get_value();
        if value != turn_cost_f64 {
            slider.set_value(turn_cost_f64);
        }

        // Spin box
        let spin_box = &mut self.turn_cost_spin_box;
        let value = spin_box.get_value();
        if value != turn_cost_f64 {
            spin_box.set_value(turn_cost_f64);
        }
    }


    #[func]
    fn on_maze_text_changed(&mut self) {
        let empty_text_field = self.text_is_empty();

        let load_text = if empty_text_field {
            "Load Default"

        } else {
            "Load Maze"
        };

        self.load_button.set_text(load_text);
    }


    #[func]
    fn on_cancel_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(LoadMapMenuRequest::Back);
    }


    #[func]
    fn on_show_or_hide_advanced_options_pressed(&mut self) {
        let showing_advanced_options = self.show_advanced_options;
        self.set_show_advanced_options(!showing_advanced_options);
    }


    #[func]
    fn on_turn_cost_spin_box_value_changed(&mut self, value : f64) {
        let as_u32 = value as u32;

        self.run_deferred(move |me| {
            me.set_turn_cost(as_u32);
        });
    }


    #[func]
    fn on_turn_cost_slider_value_changed(&mut self, value : f64) {
        let as_u32 = value as u32;

        self.run_deferred(move |me| {
            me.set_turn_cost(as_u32);
        });
    }


    fn refresh(&mut self) {
        let show_advanced_options = self.show_advanced_options;
        self.set_show_advanced_options(show_advanced_options);

        self.on_maze_text_changed();

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
                let cost_per_rotation = self.turn_cost;
                let color_a = self.color_a_picker.get_pick_color();
                let color_b = self.color_b_picker.get_pick_color();

                let inner = MainLevelConstructorInfo {
                    maze,
                    seed,
                    tree_density,
                    outer_forest_rings,
                    cost_per_rotation,
                    color_a,
                    color_b,
                };

                let info = GodotMainLevelConstructorInfo::new(inner);

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
