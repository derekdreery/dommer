
#[derive(Debug)]
pub struct DomRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl DomRect {
    pub(crate) fn from_web_sys(rect: web_sys::DomRect) -> Self {
        DomRect {
            x: rect.x(),
            y: rect.y(),
            width: rect.width(),
            height: rect.height(),
        }
    }
    /*
    pub(crate) fn into_web_sys(self) -> web_sys::DomRect {
        web_sys::DomRect::new_with_x_and_y_and_width_and_height(self.x, self.y, self.width, self.height)
    }
    */
}
