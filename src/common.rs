use crate::constants::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};


//Axis aligned bounding box for collision detection
pub struct AABB {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32
}

impl AABB {

    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> AABB {
        AABB { top, bottom, left, right }
    }

    pub fn collide(&self, other: &AABB) -> bool {
        //[0,0] is in the top left corner so Y is kind of reversed
        let y_overlap = self.top <= other.bottom && other.top <= self.bottom;
        let x_overlap = self.left <= other.right && other.left <= self.right;
        y_overlap && x_overlap
    }

    pub fn collide_with_point(&self, x: f32, y: f32) -> bool {
        let y_overlap = self.top <= y && self.bottom >= y;
        let x_overlap = self.left <= x && self.right >= x;
        y_overlap && x_overlap
    }
}


//dimensions of display, scaled real display and game area
#[derive(Clone,Copy)]
pub struct DisplayInfo {
    pub display_width: f32,
    pub display_height: f32,
    pub scaled_width: f32,
    pub scaled_height: f32,
    pub game_area: GameArea,
    pub scale_factor: (f32, f32),
}

impl DisplayInfo {
    pub fn new(width: i32, height: i32, scaled_width: i32, scaled_height: i32) -> DisplayInfo {
        DisplayInfo {
            display_width: width as f32,
            display_height: height as f32,
            scaled_width: scaled_width as f32,
            scaled_height: scaled_height as f32,
            game_area: GameArea::new(width, height),
            scale_factor: (width as f32 / scaled_width as f32, height as f32 / scaled_height as f32)
        }
    }
}


#[derive(Clone,Copy)]
pub struct GameArea {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

impl GameArea {
    pub fn new(screen_width: i32, screen_height: i32) -> GameArea {

        GameArea {
            left: (screen_width as f32 - GAME_AREA_WIDTH ) / 2.0,
            right: (screen_width as f32 + GAME_AREA_WIDTH ) / 2.0,
            top: (screen_height as f32 - GAME_AREA_HEIGHT ) / 2.0,
            bottom: (screen_height as f32 + GAME_AREA_HEIGHT ) / 2.0
        }
    }
}
