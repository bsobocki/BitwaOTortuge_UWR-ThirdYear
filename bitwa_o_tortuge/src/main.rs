use ggez;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::input::keyboard;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;
use ggez::{conf::WindowMode, conf::WindowSetup};

use std::collections::HashMap;

pub mod field;
pub mod ship;
pub mod funcs;
pub use field::Field;
pub use ship::Ship;
pub use funcs::*;

const WINDOW_W: f32 = 546.0;
const WINDOW_H: f32 = 550.0;


// STATE

struct State {
    fields : Vec<Field>,
    fields_images : Vec<graphics::Image>,
    playground_image : graphics::Image,
    stroke_image : graphics::Image,
    stroke_coords : [f32; 2],
    selected_field_index : usize,
    black_points : u8,
    red_points : u8
}

impl State
{
    fn new(ctx: &mut Context) -> GameResult<State>
    {
        let mut ships = Vec::<Ship>::new();
        let mut fields = Vec::<Field>::new();
        let mut fields_images = Vec::new();
        let random_indexes = random_fields_indexes();
        
        // create red ships
        let mut num : u8 = 1;
        for i in 0..6{
            if i>1 && i%2 == 0 { num +=1; }
            let ship_src_l = format!("/images/r{}.jpg", num);
            let ship_src_d = format!("/images/r{}d.jpg", num);
            let ship = Ship::new( num, false, String::from(ship_src_l), String::from(ship_src_d));
            ships.push(ship);
        }

        // create black ships
        num = 1;
        for i in 0..6{
            if i>1 && i%2 == 0 { num +=1; }
            let ship_src_l = format!("/images/b{}.jpg", num);
            let ship_src_d = format!("/images/b{}d.jpg", num);
            let ship = Ship::new( num, true, String::from(ship_src_l), String::from(ship_src_d));
            ships.push(ship);
        }

        for i in 0..12{
            fields.push(Field::new(
                i, 
                ships[random_indexes[i]].clone()
                ));

            let src = fields[i].image_src();
            if !src.is_empty(){
                fields_images.push(graphics::Image::new(ctx, src.as_str())?);
            }
        }

        let playground_image = graphics::Image::new(ctx, "/images/playfield.jpg")?;
        let stroke_image = graphics::Image::new(ctx, "/images/stroke.jpg")?;

        let s = State {
                fields,
                fields_images,
                playground_image,
                stroke_image,
                stroke_coords:[0.0, 0.0],
                selected_field_index : 12,
                black_points : 0,
                red_points : 0
        };
        
        Ok(s)
    }

    fn reset_stroke(&mut self){
        self.stroke_coords = [0.0, 0.0]; // stroke unavailable
        self.selected_field_index = 12;
    }
}


// EVENT HANDLER

impl ggez::event::EventHandler for State{


    // UPDATE 
    
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {        
        for i in 0..12{
            let src : String = self.fields[i].image_src();
            if !src.is_empty(){
                self.fields_images[i] = graphics::Image::new(ctx, src.as_str())?;
            }
        }
        Ok(())
    }


    // DRAW

    fn draw (&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        
        let pi = 3.1415926535;

        let bck = Point2::new(0.0, 0.0);
        graphics::draw(ctx, &self.playground_image, (bck,))?;

        if is_stroke_available(self.stroke_coords) {
            let strk = Point2::new(self.stroke_coords[0], self.stroke_coords[1]);
            graphics::draw(ctx, &self.stroke_image, (strk,));
        }

        for i in 0..12{
            let coords = Field::FIELDS_COORDS[i];
            let rotation = self.fields[i].rotation();
            let moved = field::move_image_by_rotation(rotation);
            let (x,y) = (coords.0 + moved.0, coords.1 + moved.1);
            let shp = Point2::new(x, y);    
            let rotate = (rotation as f32) * pi/2.0;
            let draw_param = graphics::DrawParam::new().dest(shp).rotation(rotate);

            if !self.fields[i].outdoor().is_sunk() {
                graphics::draw(ctx, &self.fields_images[i], draw_param)?;
            }
        }

        graphics::present(ctx);
        Ok(())
    } 


    // MOUSE BUTTON DOWN

    fn mouse_button_down_event(
        &mut self, 
        ctx: &mut Context, 
        button: ggez::event::MouseButton, 
        x: f32, 
        y: f32)
    {
        if ggez::input::mouse::button_pressed(ctx, ggez::event::MouseButton::Left)
        {
            for i in 0..12{
                if self.fields[i].is_on_field(x,y)
                {
                    let (xx, yy) = Field::FIELDS_COORDS[i];
                    self.stroke_coords = [xx-5.0, yy-7.0];
                    self.fields[i].set_rev_outdoor(false);
                    self.selected_field_index = i;
                }
            }
        }
        else if ggez::input::mouse::button_pressed(ctx, ggez::event::MouseButton::Right)
        {
            self.reset_stroke()
        }
    }


    // MOUSE MOTION
    
    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32){
        for i in 0..12{
            if self.fields[i].is_on_field(x, y)
            {
                self.fields[i].set_light(false);
            }
            else 
            {
                self.fields[i].set_light(true);
            }
        }
    }


    // KEY DOWN

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        let selected = self.selected_field_index;
        let rotation = self.fields[selected].rotation();
        let value = self.fields[selected].outdoor().score();

        match key {
            // Quit if Shift+Q is pressed.
            KeyCode::Q => {
                if mods.contains(KeyMods::CTRL) {
                    println!("Terminating!");
                    event::quit(ctx);
                } 
            },

            KeyCode::Left => {
                if selected < 12 {
                    let rot = match self.fields[selected].rotation(){
                        0 => 3,
                        1 => 0,
                        2 => 1,
                        3 => 2,
                        _ => 0
                    };
                    self.fields[selected].set_rotation(rot);
                }
            },

            KeyCode::Right => {
                if selected < 12 {
                    let rot = match self.fields[selected].rotation(){
                        0 => 1,
                        1 => 2,
                        2 => 3,
                        3 => 0,
                        _ => 0
                    };
                    self.fields[selected].set_rotation(rot);
                }
            },

            KeyCode::Numpad1 => {
                if selected < 12 && can_move(rotation, value, 1) {
                    let index = match selected {
                        0|1|8|9 => selected + 2,
                        3|4|5 => selected + 3,
                        _ => selected
                    };
                    field::move_ship(&mut self.fields, selected, index);
                    self.reset_stroke()
                }
            },

            KeyCode::Numpad2 => {
                if selected < 12 && can_move(rotation, value, 2) {
                    let index = match selected {
                        0|1|7|8 => selected + 3,
                        3|4|5 => selected + 4,
                        _ => selected
                    };
                    field::move_ship(&mut self.fields, selected, index);
                    self.reset_stroke()
                }
            },

            KeyCode::Numpad3 => {
                if selected < 12 && can_move(rotation, value, 3) {
                    let index = match selected {
                        0|1|6|7 => selected + 4,
                        2|3|4 => selected + 5,
                        _ => selected
                    };
                    field::move_ship(&mut self.fields, selected, index);
                    self.reset_stroke()
                }
            },

            KeyCode::Numpad4 => {
                if selected < 12 && can_move(rotation, value, 4) {
                    let index = match selected {
                        1|3|4|5|7|8|9|11 => selected - 1,
                        _ => selected
                    };
                    field::move_ship(&mut self.fields, selected, index);
                    self.reset_stroke()
                }
            },

            KeyCode::Numpad6 => {
                if selected < 12 && can_move(rotation, value, 6) {
                    let index = match selected {
                        0|2|3|4|6|7|8|10 => selected + 1,
                        _ => selected
                    };
                    field::move_ship(&mut self.fields, selected, index);
                    self.reset_stroke()
                }
            },

            KeyCode::Numpad7 => {
                if selected < 12 && can_move(rotation, value, 7) {
                    let index = match selected {
                        4|5|10|11 => selected - 4,
                        7|8|9 => selected - 5,
                        _ => selected
                    };
                    field::move_ship(&mut self.fields, selected, index);
                    self.reset_stroke()
                }
            },

            KeyCode::Numpad8 => {
                if selected < 12 && can_move(rotation, value, 8) {
                    let index = match selected {
                        3|4|10|11 => selected - 3,
                        6|7|8|9 => selected - 4,
                        _ => selected
                    };
                    field::move_ship(&mut self.fields, selected, index);
                    self.reset_stroke()
                }
            },

            KeyCode::Numpad9 => {
                if selected < 12 && can_move(rotation, value, 9) {
                    let index = match selected {
                        2|3|10|11 => selected - 2,
                        6|7|8 => selected - 3,
                        _ => selected
                    };
                    field::move_ship(&mut self.fields, selected, index);
                    self.reset_stroke()
                }
            },
            _ => (),
        }

        println!("points of reds:{}  |   points of black: {}", self.red_points,self.black_points);
    }
}


// MAIN FUNCTION

fn main() {
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Fog of War", "Jack Mordaunt")
        .window_mode(WindowMode::default()
        .dimensions(WINDOW_W, WINDOW_H))
        .window_setup(WindowSetup::default()
        .title("Bitwa O Tortuge"))
        .build()
        .unwrap();
    let State = &mut State::new(ctx).unwrap();
    event::run(ctx, event_loop, State).unwrap();
}