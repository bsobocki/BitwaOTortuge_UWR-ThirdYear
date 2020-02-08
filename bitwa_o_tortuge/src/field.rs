use std::collections::HashMap;

pub use crate::ship::Ship;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;
use ggez::{conf::WindowMode, conf::WindowSetup};

#[derive(Debug, Clone)]
pub struct Field {
    number : usize,     //field number on the board (from left to right)
    outdoor : Ship,     //ship that is on the to of the field
    hidden : Ship,      //ship that is under the outdoor ship
    rotation : u8,      //number of the rotation of the field (i -> rotate i%4 * 90 degrees)
    is_light : bool
}

impl Field {
    // const member fields 
    pub const FIELD_SIZE : (f32,f32) = (80.0, 84.0);
    pub const FIELDS_COORDS : [(f32, f32); 12] = [
        (185.0, 88.0),
        (282.0, 87.0),
        (86.0, 185.0),
        (184.0, 183.0),
        (282.0, 183.0),
        (379.0, 183.0),
        (86.0, 282.0),
        (183.0, 282.0),
        (281.0, 282.0),
        (379.0, 281.0),
        (183.0, 381.0),
        (281.0, 379.0) ];

    pub fn new(number : usize, outdoor : Ship) -> Field{
        Field{
            number:number,
            hidden:Ship::empty_ship(),
            outdoor:outdoor,
            rotation:0,
            is_light:true
        }
    }


    pub fn is_on_field(&self, x : f32, y : f32) -> bool{
        let x_bigger_equal : bool =  x >= Field::FIELDS_COORDS[self.number].0;
        let x_less_equal : bool   =  x <= Field::FIELDS_COORDS[self.number].0 + Field::FIELD_SIZE.0;

        let y_bigger_equal : bool =  y >= Field::FIELDS_COORDS[self.number].1;
        let y_less_equal : bool   =  y <= Field::FIELDS_COORDS[self.number].1 + Field::FIELD_SIZE.1;

        x_bigger_equal && x_less_equal && y_bigger_equal && y_less_equal
    }

    pub fn number(&self) -> usize { self.number }
    pub fn image_src(&self) -> String { 
        if self.outdoor.rev() {
            if self.is_light{ 
                return self.outdoor.rev_src_l();
            }
            else {
                return self.outdoor.rev_src_d();
            }
        }
        if self.outdoor.is_sunk() {
            return String::new();
        }
        if self.outdoor.show() {
            if self.is_light {
                return self.outdoor.background_src_l();
            }
            return self.outdoor.background_src_d();
        }
        else {
            if self.is_light{
                return self.outdoor.rev_src_l();
            }
            return self.outdoor.rev_src_d();
        }
    }
    pub fn rotation(&self) -> u8 { self.rotation }
    pub fn set_rev_outdoor(&mut self, rev:bool) { self.outdoor.set_rev(rev); }
    pub fn outdoor(&self) -> Ship { self.outdoor.clone() }
    pub fn hidden(&self) -> Ship { self.hidden.clone() }


    pub fn set_hidden(&mut self, hidden_ship : Ship){ self.hidden = hidden_ship.clone(); }
    pub fn set_outdoor(&mut self, outdoor_ship : Ship){ self.outdoor = outdoor_ship.clone(); }
    pub fn set_rotation(&mut self, rot : u8) { self.rotation = rot }
    pub fn set_light(&mut self, is_light : bool) { self.is_light = is_light }
}

pub fn move_image_by_rotation(rotation : u8) -> (f32, f32) {
    let (w,h) = Field::FIELD_SIZE;
    let rot = rotation % 4;
    match rot {
        0 => (0.0, 0.0),
        1 => (w, 0.0),
        2 => (w, h-3.5),
        3 => (0.0, h-3.5),
        _ => (0.0, 0.0)
    }
}

pub fn move_ship(fields : &mut Vec<Field>, selected : usize, target : usize){
    let sel_out = fields[selected].outdoor();
    let sel_hid = fields[selected].hidden();
    let tar_out = fields[target].outdoor();
    let tar_out = fields[target].hidden();
    let rot_sel = fields[selected].rotation();

    if fields[target].outdoor().show() && fields[selected].outdoor().is_pirate() != fields[target].outdoor().is_pirate() ||
         !fields[target].outdoor().show() {
        fields[target].set_rotation(rot_sel);
        fields[target].set_hidden(tar_out);
        fields[target].set_outdoor(sel_out);
        fields[selected].set_outdoor(Ship::empty_ship());
    }
}