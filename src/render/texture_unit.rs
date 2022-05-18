use core::fmt;
use web_sys::WebGl2RenderingContext as GL;

#[derive(Clone, Copy)]
pub enum TextureUnit {
    Velodrome = 1,
    Stone = 2,
    NormalMap = 3,
    Stad = 4,
}

impl fmt::Display for TextureUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match self {
            TextureUnit::Velodrome => "Velodrome",
            TextureUnit::Stone => "Stone",
            TextureUnit::NormalMap => "NormalMap",
            TextureUnit::Stad => "Stad",
        };
        write!(f, "{}", x)
    }
}

impl TextureUnit {
    /// gl.TEXTURE1, gl.TEXTURE2 ... etc. Useful for `gl.active_texture`
    #[allow(non_snake_case)]
    pub fn TEXTURE_N(&self) -> u32 {
        match self {
            TextureUnit::Velodrome => GL::TEXTURE1,
            TextureUnit::Stone => GL::TEXTURE2,
            TextureUnit::NormalMap => GL::TEXTURE3,
            TextureUnit::Stad => GL::TEXTURE4,
        }
    }

    /// 0, 1, 2, ... etc. Useful for `gl.uniform1i` calls
    pub fn texture_unit(&self) -> i32 {
        *self as i32
    }
}
