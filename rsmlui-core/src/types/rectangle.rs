use glam::{IVec2, Vec2};
use rsmlui_sys::{Rml_Rectanglef, Rml_Rectanglei};

use crate::FromSys;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Rectangle<V> {
    pub p0: V,
    pub p1: V,
}

pub type Rectanglef = Rectangle<Vec2>;
pub type Rectanglei = Rectangle<IVec2>;

const _: () = {
    use std::mem::{align_of, offset_of, size_of};

    // Rectanglef / Rml_Rectanglef
    rsmlui_sys::const_assert_eq!(size_of::<Rectanglef>(), 16);
    rsmlui_sys::const_assert_eq!(align_of::<Rectanglef>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rectanglef, p0), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rectanglef, p1), 8);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Rectanglef>(), 16);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Rectanglef>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Rectanglef, p0), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Rectanglef, p1), 8);

    // Rectanglei / Rml_Rectanglei
    rsmlui_sys::const_assert_eq!(size_of::<Rectanglei>(), 16);
    rsmlui_sys::const_assert_eq!(align_of::<Rectanglei>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rectanglei, p0), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rectanglei, p1), 8);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Rectanglei>(), 16);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Rectanglei>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Rectanglei, p0), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Rectanglei, p1), 8);
};

macro_rules! impl_rectangle {
    ($vec:ty, $scalar:ty, $neg_one:expr, $half:expr) => {
        impl Rectangle<$vec> {
            pub const fn from_position(pos: $vec) -> Self {
                Self { p0: pos, p1: pos }
            }

            pub fn from_position_size(pos: $vec, size: $vec) -> Self {
                Self {
                    p0: pos,
                    p1: pos + size,
                }
            }

            pub const fn from_size(size: $vec) -> Self {
                Self {
                    p0: <$vec>::ZERO,
                    p1: size,
                }
            }

            pub const fn from_corners(top_left: $vec, bottom_right: $vec) -> Self {
                Self {
                    p0: top_left,
                    p1: bottom_right,
                }
            }

            pub const fn make_invalid() -> Self {
                Self {
                    p0: <$vec>::ZERO,
                    p1: <$vec>::splat($neg_one),
                }
            }

            pub const fn position(&self) -> $vec {
                self.p0
            }

            pub fn size(&self) -> $vec {
                self.p1 - self.p0
            }

            pub const fn top_left(&self) -> $vec {
                self.p0
            }

            pub const fn top_right(&self) -> $vec {
                <$vec>::new(self.p1.x, self.p0.y)
            }

            pub const fn bottom_right(&self) -> $vec {
                self.p1
            }

            pub const fn bottom_left(&self) -> $vec {
                <$vec>::new(self.p0.x, self.p1.y)
            }

            pub fn center(&self) -> $vec {
                (self.p0 + self.p1) / $half
            }

            pub const fn left(&self) -> $scalar {
                self.p0.x
            }

            pub const fn right(&self) -> $scalar {
                self.p1.x
            }

            pub const fn top(&self) -> $scalar {
                self.p0.y
            }

            pub const fn bottom(&self) -> $scalar {
                self.p1.y
            }

            pub const fn width(&self) -> $scalar {
                self.p1.x - self.p0.x
            }

            pub const fn height(&self) -> $scalar {
                self.p1.y - self.p0.y
            }

            pub fn extend(&self, v: $vec) -> Self {
                Self {
                    p0: self.p0 - v,
                    p1: self.p1 + v,
                }
            }

            pub fn extend_uniform(&self, v: $scalar) -> Self {
                self.extend(<$vec>::splat(v))
            }

            pub fn extend_asymmetric(&self, top_left: $vec, bottom_right: $vec) -> Self {
                Self {
                    p0: self.p0 - top_left,
                    p1: self.p1 + bottom_right,
                }
            }

            pub fn translate(&self, v: $vec) -> Self {
                Self {
                    p0: self.p0 + v,
                    p1: self.p1 + v,
                }
            }

            pub fn join_point(&self, p: $vec) -> Self {
                Self {
                    p0: self.p0.min(p),
                    p1: self.p1.max(p),
                }
            }

            pub fn join(&self, other: Self) -> Self {
                Self {
                    p0: self.p0.min(other.p0),
                    p1: self.p1.max(other.p1),
                }
            }

            pub fn intersect(&self, other: Self) -> Self {
                debug_assert!(self.valid() && other.valid());

                let p0 = self.p0.max(other.p0);
                let p1 = self.p1.min(other.p1).max(p0);

                Self { p0, p1 }
            }

            pub fn intersect_if_valid(&self, other: Self) -> Self {
                if !self.valid() || !other.valid() {
                    return *self;
                }

                self.intersect(other)
            }

            pub const fn intersects(&self, other: Self) -> bool {
                self.p0.x < other.p1.x
                    && self.p1.x > other.p0.x
                    && self.p0.y < other.p1.y
                    && self.p1.y > other.p0.y
            }

            pub const fn contains(&self, point: $vec) -> bool {
                point.x >= self.p0.x
                    && point.x <= self.p1.x
                    && point.y >= self.p0.y
                    && point.y <= self.p1.y
            }

            pub const fn valid(&self) -> bool {
                self.p0.x <= self.p1.x && self.p0.y <= self.p1.y
            }
        }
    };
}

impl_rectangle!(Vec2, f32, -1.0f32, 2.0f32);
impl_rectangle!(IVec2, i32, -1i32, 2i32);

impl Eq for Rectangle<IVec2> {}

impl FromSys<Rml_Rectanglef> for Rectanglef {
    fn from_sys(value: Rml_Rectanglef) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_Rectanglef> for &Rectanglef {
    fn from_sys(value: &Rml_Rectanglef) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<Rectanglef> for Rml_Rectanglef {
    fn from_sys(value: Rectanglef) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rectanglef> for &Rml_Rectanglef {
    fn from_sys(value: &Rectanglef) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<Rml_Rectanglei> for Rectanglei {
    fn from_sys(value: Rml_Rectanglei) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_Rectanglei> for &Rectanglei {
    fn from_sys(value: &Rml_Rectanglei) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<Rectanglei> for Rml_Rectanglei {
    fn from_sys(value: Rectanglei) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rectanglei> for &Rml_Rectanglei {
    fn from_sys(value: &Rectanglei) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
