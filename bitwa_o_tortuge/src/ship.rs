#[derive(Debug, Clone)]
pub struct Ship {
    score : u8,
    rev : bool,     //informations that the ship revers side should be shown
    show : bool,    //informations that the ship can be shown (it has been clicked at least once and it isn't sunk)
    is_sunk : bool,             
    is_pirate : bool,
    rev_src_l : String,         //source to the revers side of the ship (light version)
    rev_src_d : String,         //source to the revers side of the ship (dark version)
    able_to_move : bool,        //this ship can be moved or rotated
    background_src_l : String,  //source to the image of the ship (light version)
    background_src_d : String,  //source to the image of the ship (dark version)
}

impl Ship {
    pub fn new( 
        score : u8,
        is_pirate : bool,
        bck_src_l : String,
        bck_src_d : String,
        ) -> Ship 
    {
         Ship{
            score:score,
            rev:true,
            show:true,
            is_sunk:false,
            is_pirate:is_pirate,
            rev_src_l:String::from("/images/rewers.jpg"),
            rev_src_d:String::from("/images/rewers2.jpg"),
            able_to_move:true,
            background_src_l:bck_src_l,
            background_src_d:bck_src_d,
        }
    }

    pub fn empty_ship() -> Ship{
        Ship {
            score:0,
            rev:true,
            show:false,
            is_sunk:true,
            is_pirate:true,
            rev_src_l:String::new(),
            rev_src_d:String::new(),
            able_to_move:false,
            background_src_l:String::new(),
            background_src_d:String::new()
        }
    }
    
    pub fn rev(&self) -> bool { self.rev }
    pub fn set_rev(&mut self, rev:bool) { self.rev =rev; }

    pub fn is_sunk(&self) -> bool { self.is_sunk }
    pub fn sunk(&mut self) { self.is_sunk = true; self.show = false; }
    pub fn set_show(&mut self, show : bool){ self.show = show;}

    pub fn background_src_l(&self) -> String { self.background_src_l.clone() }
    pub fn background_src_d(&self) -> String { self.background_src_d.clone() }

    pub fn rev_src_l(&self) -> String { self.rev_src_l.clone() }
    pub fn rev_src_d(&self) -> String { self.rev_src_d.clone() }

    pub fn show(&self) -> bool { self.show }
    pub fn score(&self) -> u8 { self.score }
    pub fn is_pirate(&self) -> bool { self.is_pirate }
}