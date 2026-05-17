use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use rsmlui_macros::rmldoc;
use rsmlui_sys::{Rml_Colourb, Rml_ColourbPremultiplied};

use crate::utils::conversions::{FromSys, IntoSys};

#[rmldoc(file = "api_Rml-Colour.md", name = "Rml::Colour")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Colorb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Alpha premultiplied version.
#[rmldoc(file = "api_Rml-Colour.md", name = "Rml::Colour")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ColorbPremultiplied {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[rmldoc(file = "api_Rml-Colour.md")]
impl Colorb {
    #[rmldoc(
        name = "Rml::Colour::Colour",
        refid = "class_rml_1_1_colour_1ac73a949b42ecff51c9cf2303bd2c1a03"
    )]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    #[rmldoc(
        name = "Rml::Colour::Colour",
        refid = "class_rml_1_1_colour_1a54146996d171f328f01fb5bbe2e4ab8a"
    )]
    pub const fn new_single(rgb: u8, alpha: u8) -> Self {
        Self {
            r: rgb,
            g: rgb,
            b: rgb,
            a: alpha,
        }
    }

    pub fn to_premultiplied(self) -> ColorbPremultiplied {
        let a = self.a as u32;
        ColorbPremultiplied {
            r: ((self.r as u32 * a) / 255) as u8,
            g: ((self.g as u32 * a) / 255) as u8,
            b: ((self.b as u32 * a) / 255) as u8,
            a: self.a,
        }
    }

    pub fn to_premultiplied_with_opacity(self, opacity: f32) -> ColorbPremultiplied {
        let new_alpha = self.a as f32 * opacity;
        let factor = new_alpha / 255.0;
        ColorbPremultiplied {
            r: (self.r as f32 * factor) as u8,
            g: (self.g as f32 * factor) as u8,
            b: (self.b as f32 * factor) as u8,
            a: new_alpha as u8,
        }
    }
}

impl ColorbPremultiplied {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_non_premultiplied(self) -> Colorb {
        let a = self.a as u32;
        Colorb {
            r: (self.r as u32 * 255).checked_div(a).unwrap_or(0) as u8,
            g: (self.g as u32 * 255).checked_div(a).unwrap_or(0) as u8,
            b: (self.b as u32 * 255).checked_div(a).unwrap_or(0) as u8,
            a: self.a,
        }
    }
}

impl From<Colorb> for ColorbPremultiplied {
    fn from(value: Colorb) -> Self {
        value.to_premultiplied()
    }
}

impl From<ColorbPremultiplied> for Colorb {
    fn from(value: ColorbPremultiplied) -> Self {
        value.to_non_premultiplied()
    }
}

macro_rules! impl_colour_ops {
    ($t:ty) => {
        // Channels wrap on overflow.
        #[rmldoc(file = "api_Rml-Colour.md")]
        impl Add for $t {
            type Output = Self;

            #[rmldoc(name = "Rml::Colour::operator+")]
            fn add(self, rhs: Self) -> Self {
                Self {
                    r: self.r.wrapping_add(rhs.r),
                    g: self.g.wrapping_add(rhs.g),
                    b: self.b.wrapping_add(rhs.b),
                    a: self.a.wrapping_add(rhs.a),
                }
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl Sub for $t {
            type Output = Self;

            #[rmldoc(name = "Rml::Colour::operator-")]
            fn sub(self, rhs: Self) -> Self {
                Self {
                    r: self.r.wrapping_sub(rhs.r),
                    g: self.g.wrapping_sub(rhs.g),
                    b: self.b.wrapping_sub(rhs.b),
                    a: self.a.wrapping_sub(rhs.a),
                }
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl Mul<f32> for $t {
            type Output = Self;

            #[rmldoc(name = "Rml::Colour::operator*")]
            fn mul(self, rhs: f32) -> Self {
                Self {
                    r: (self.r as f32 * rhs) as u8,
                    g: (self.g as f32 * rhs) as u8,
                    b: (self.b as f32 * rhs) as u8,
                    a: (self.a as f32 * rhs) as u8,
                }
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl Div<f32> for $t {
            type Output = Self;

            #[rmldoc(name = "Rml::Colour::operator/")]
            fn div(self, rhs: f32) -> Self {
                Self {
                    r: (self.r as f32 / rhs) as u8,
                    g: (self.g as f32 / rhs) as u8,
                    b: (self.b as f32 / rhs) as u8,
                    a: (self.a as f32 / rhs) as u8,
                }
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl AddAssign for $t {
            #[rmldoc(name = "Rml::Colour::operator+=")]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl SubAssign for $t {
            #[rmldoc(name = "Rml::Colour::operator-=")]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl MulAssign<f32> for $t {
            #[rmldoc(name = "Rml::Colour::operator*=")]
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl DivAssign<f32> for $t {
            #[rmldoc(name = "Rml::Colour::operator/=")]
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }
    };
}

impl_colour_ops!(Colorb);
impl_colour_ops!(ColorbPremultiplied);

// Hardcoded layout assertions for safe transmutes.
// If the C++ side changes and bindgen regenerates, these will catch the
// mismatch before any transmute silently reinterprets wrong bytes.
//
// I think ideally these would be done with something like bytemuck, but
// deriving bytemuck on bindgen structs is a bit difficult.
const _: () = {
    use std::mem::{align_of, offset_of, size_of};
    rsmlui_sys::const_assert_eq!(size_of::<Colorb>(), 4);
    rsmlui_sys::const_assert_eq!(align_of::<Colorb>(), 1);
    rsmlui_sys::const_assert_eq!(offset_of!(Colorb, r), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Colorb, g), 1);
    rsmlui_sys::const_assert_eq!(offset_of!(Colorb, b), 2);
    rsmlui_sys::const_assert_eq!(offset_of!(Colorb, a), 3);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Colourb>(), 4);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Colourb>(), 1);
    rsmlui_sys::const_assert_eq!(size_of::<ColorbPremultiplied>(), 4);
    rsmlui_sys::const_assert_eq!(align_of::<ColorbPremultiplied>(), 1);
    rsmlui_sys::const_assert_eq!(offset_of!(ColorbPremultiplied, r), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(ColorbPremultiplied, g), 1);
    rsmlui_sys::const_assert_eq!(offset_of!(ColorbPremultiplied, b), 2);
    rsmlui_sys::const_assert_eq!(offset_of!(ColorbPremultiplied, a), 3);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_ColourbPremultiplied>(), 4);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_ColourbPremultiplied>(), 1);
};

impl FromSys<Rml_Colourb> for Colorb {
    fn from_sys(value: Rml_Colourb) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_Colourb> for &Colorb {
    fn from_sys(value: &Rml_Colourb) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_Colourb> for Colorb {
    fn into_sys(self) -> Rml_Colourb {
        unsafe { std::mem::transmute(self) }
    }
}

impl FromSys<Rml_ColourbPremultiplied> for ColorbPremultiplied {
    fn from_sys(value: Rml_ColourbPremultiplied) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_ColourbPremultiplied> for ColorbPremultiplied {
    fn into_sys(self) -> Rml_ColourbPremultiplied {
        unsafe { std::mem::transmute(self) }
    }
}
