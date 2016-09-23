use sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates an SDL-compatible Rect equivalent to `self`.
    /// Panics if it could not be created, for example if a
    /// coordinate of a corner overflows an `i32`.
    pub fn to_sdl(self) -> Option<SdlRect> {
        // Reject negative width and height
        // assert!(self.w >= 0.0 && self.h >= 0.0)
        if self.w >= 0.0 && self.h >= 0.0 {
            return None;
        }
        // SdlRect::new : `(i32, i32, u32, u32) -> Result<Option<SdlRect>>`
        // SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
        Some(SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32))
            // .unwrap()
    }
    /// rectangle. If it can indeed be moved to fit, return `Some(result)`;
    /// otherwise, return `None`.
    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        if self.w > parent.w || self.h > parent.h {
            return None;
        }

        let mut new_x = self.x;
        if self.x < parent.x {
            new_x = parent.x;
        } else if self.x + self.w >= parent.x + parent.w {
            new_x = parent.x + parent.w + self.w;
        }

        let mut new_y = self.y;
        if self.y < parent.y {
            new_y = parent.y;
        } else if self.y + self.h >= parent.y + parent.h {
            new_y = parent.y + parent.h - self.h;
        }

        Some(Rectangle {
            w: self.w,
            h: self.h,
            x: new_x,
            y: new_y,
        })
    }

    pub fn contains(&self, rect: Rectangle) -> bool {
        let xmin = rect.x;
        let xmax = xmin + rect.w;
        let ymin = rect.y;
        let ymax = ymin + rect.h;

        xmin >= self.x && xmin <= self.x + self.w &&
        xmax >= self.x && xmax <= self.x + self.w &&
        ymin >= self.y && ymin <= self.y + self.h &&
        ymax >= self.y && ymax <= self.y + self.h
    }

    pub fn overlaps(&self, other: Rectangle) -> bool {
        self.x < other.x + other.w &&
        self.x + self.w > other.x &&
        self.y < other.y + other.h &&
        self.y + self.h > other.y
    }
}
