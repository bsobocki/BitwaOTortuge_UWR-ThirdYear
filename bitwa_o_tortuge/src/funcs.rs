extern crate rand;


// ADDITIONAL FUNCTIONS

pub fn random_fields_indexes() -> Vec<usize> {
    let mut indexes : Vec<usize> = (0..12).into_iter().collect();

    for i in 0..12{
        // rand % (12-i)  -->   choose index among (12-i) indexes
        // + i   -->   the first i indexes are set up, we want to set the next one, 
        // so we should move forward by i elements
        let a : usize = rand::random::<usize>()%(12-i) + i;
        let buf = indexes[i];
        indexes[i] = indexes[a];
        indexes[a] = buf;
    }
    indexes
}

pub fn can_move(rotate : u8, value : u8, direction : u8) -> bool {
    match rotate {
        0 => {
            match direction {
                1|7 => {
                    if value > 1 { return true; }
                    return false;
                },
                4 => {
                    if value != 2 { return true; }
                    return false;
                },
                _ => { return false; }
            }
        },
        1 => {
            match direction {
                7|9 => {
                    if value > 1 { return true; }
                    return false;
                },
                8 => {
                    if value != 2 { return true; }
                    return false;
                },
                _ => { return false; }
            }
        },
        2 => {
            match direction {
                3|9 => {
                    if value > 1 { return true; }
                    return false;
                },
                6 => {
                    if value != 2 { return true; }
                    return false;
                },
                _ => { return false; }
            }
        },
        _ => {
            match direction {
                1|3 => {
                    if value > 1 { return true; }
                    return false;
                },
                2 => {
                    if value != 2 { return true; }
                    return false;
                },
                _ => { return false; }
            }
        } 
    }
}

pub fn is_stroke_available(stroke_coords : [f32; 2]) -> bool {
    stroke_coords[0] != 0.0 && stroke_coords[1] != 0.0
}