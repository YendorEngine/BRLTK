use doryen_rs::{Color, Console, Image};

/// Wrapper type around `Image` that allows for asset path enhancements automatically.
///
/// An easy way to load PNG images and blit them on the console
pub struct DoryenImage(pub Image);

impl DoryenImage {
    /// Create an image and load a PNG file.
    /// On the web platform, image loading is asynchronous.
    /// Using blit methods before the image is loaded has no impact on the console.
    pub fn new(file_path: &str) -> Self {
        Self(Image::new(&format!("assets/{file_path}")))
    }

    /// Create an empty image.
    pub fn new_empty(width: u32, height: u32) -> Self {
        Self(Image::new_empty(width, height))
    }

    /// Returns the image's width in pixels or 0 if the image has not yet been loaded
    pub fn width(&self) -> u32 {
        self.0.width()
    }

    /// Returns the image's height in pixels or 0 if the image has not yet been loaded
    pub fn height(&self) -> u32 {
        self.0.height()
    }

    /// get the color of a specific pixel inside the image
    pub fn pixel(&self, x: u32, y: u32) -> Option<Color> {
        self.0.pixel(x, y)
    }

    /// sets the color of a specific pixel inside the image
    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.0.put_pixel(x, y, color)
    }

    /// Check if the image has been loaded.
    /// Since there's no background thread doing the work for you, you have to call some method on image for it to actually load.
    /// Use either [`Image::try_load`], [`Image::get_size`], [`Image::blit`] or [`Image::blit_ex`] to run the loading code.
    pub fn try_load(&mut self) -> bool {
        self.0.try_load()
    }

    /// If the image has already been loaded, return its size, else return None
    pub fn try_get_size(&mut self) -> Option<(u32, u32)> {
        self.0.try_get_size()
    }

    /// blit an image on a console
    ///
    /// x,y are the coordinate of the top left image pixel in the console
    ///
    /// image pixels using the transparent color will be ignored
    pub fn blit(&mut self, con: &mut Console, x: i32, y: i32, transparent: Option<Color>) {
        self.0.blit(con, x, y, transparent)
    }
    /// blit an image on a console
    ///
    /// x,y are the coordinate of the image center in the console
    /// image can be scaled and rotated (angle is in radians)
    /// image pixels using the transparent color will be ignored
    #[allow(clippy::too_many_arguments)]
    pub fn blit_ex(
        &mut self,
        con: &mut Console,
        x: f32,
        y: f32,
        scalex: f32,
        scaley: f32,
        angle: f32,
        transparent: Option<Color>,
    ) {
        self.0.blit_ex(con, x, y, scalex, scaley, angle, transparent)
    }

    /// blit an image on the console, using the subcell characters to achieve twice the normal resolution.
    /// This uses the CHAR_SUBCELL_* ascii codes (from 226 to 232):
    ///
    /// ![subcell_chars](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/subcell/subcell.png)
    ///
    /// Comparison before/after subcell in the chronicles of Doryen :
    ///
    /// ![subcell_comp](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/subcell/subcell_comp.png)
    ///
    /// Pyromancer! screenshot, making full usage of subcell resolution:
    ///
    /// ![subcell_pyro](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/subcell/subcell_pyro.png)
    #[allow(clippy::too_many_arguments)]
    pub fn blit_2x(
        &mut self,
        con: &mut Console,
        dx: i32,
        dy: i32,
        sx: i32,
        sy: i32,
        w: Option<i32>,
        h: Option<i32>,
        transparent: Option<Color>,
    ) {
        self.0.blit_2x(con, dx, dy, sx, sy, w, h, transparent)
    }

    /// blit an image on a console. See [`Image::blit_2x`]
    #[allow(clippy::too_many_arguments)]
    pub fn blit_2x_image(
        img: &image::RgbaImage,
        con: &mut Console,
        dx: i32,
        dy: i32,
        sx: i32,
        sy: i32,
        w: Option<i32>,
        h: Option<i32>,
        transparent: Option<Color>,
    ) {
        Image::blit_2x_image(img, con, dx, dy, sx, sy, w, h, transparent)
    }
}
