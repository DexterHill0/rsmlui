use rsmlui_macros::sys_cast;
use rsmlui_sys::{Rml_Rectanglef, Rml_Rectanglei};

use crate::math::{IVec2, Vec2};

#[sys_cast(
    struct(
        from(
            pair(from = Rml_Rectanglef, self = Rectangle<Vec2>),
            pair(from = Rml_Rectanglei, self = Rectangle<IVec2>),
        )
    ),
    gen_ref
)]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Rectangle<V> {
    pub p0: V,
    pub p1: V,
}

pub type Rectanglef = Rectangle<Vec2>;
pub type Rectanglei = Rectangle<IVec2>;

macro_rules! impl_rectangle {
    ($vec:ty, $scalar:ty, $neg_one:expr, $half:expr) => {
        impl Rectangle<$vec> {
            pub const fn from_position(pos: $vec) -> Self {
                Self { p0: pos, p1: pos }
            }

            pub fn from_position_size(pos: $vec, size: $vec) -> Self {
                Self {
                    p0: pos,
                    p1: <$vec>::new(pos.0.x + size.0.x, pos.0.y + size.0.y),
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
                <$vec>::new(self.p1.x - self.p0.x, self.p1.y - self.p0.y)
            }

            pub const fn top_left(&self) -> $vec {
                self.p0
            }

            pub const fn top_right(&self) -> $vec {
                <$vec>::new(self.p1.0.x, self.p0.0.y)
            }

            pub const fn bottom_right(&self) -> $vec {
                self.p1
            }

            pub const fn bottom_left(&self) -> $vec {
                <$vec>::new(self.p0.0.x, self.p1.0.y)
            }

            pub fn center(&self) -> $vec {
                <$vec>::new(
                    (self.p0.x + self.p1.x) / $half,
                    (self.p0.y + self.p1.y) / $half,
                )
            }

            pub const fn left(&self) -> $scalar {
                self.p0.0.x
            }

            pub const fn right(&self) -> $scalar {
                self.p1.0.x
            }

            pub const fn top(&self) -> $scalar {
                self.p0.0.y
            }

            pub const fn bottom(&self) -> $scalar {
                self.p1.0.y
            }

            pub const fn width(&self) -> $scalar {
                self.p1.0.x - self.p0.0.x
            }

            pub const fn height(&self) -> $scalar {
                self.p1.0.y - self.p0.0.y
            }

            pub fn extend(&self, v: $vec) -> Self {
                Self {
                    p0: <$vec>::new(self.p0.x - v.x, self.p0.y - v.y),
                    p1: <$vec>::new(self.p1.x + v.x, self.p1.y + v.y),
                }
            }

            pub fn extend_uniform(&self, v: $scalar) -> Self {
                self.extend(<$vec>::splat(v))
            }

            pub fn extend_asymmetric(&self, top_left: $vec, bottom_right: $vec) -> Self {
                Self {
                    p0: <$vec>::new(self.p0.x - top_left.x, self.p0.y - top_left.y),
                    p1: <$vec>::new(self.p1.x + bottom_right.x, self.p1.y + bottom_right.y),
                }
            }

            pub fn translate(&self, v: $vec) -> Self {
                Self {
                    p0: <$vec>::new(self.p0.x + v.x, self.p0.y + v.y),
                    p1: <$vec>::new(self.p1.x + v.x, self.p1.y + v.y),
                }
            }

            pub fn join_point(&self, p: $vec) -> Self {
                Self {
                    p0: <$vec>::new(self.p0.x.min(p.x), self.p0.y.min(p.y)),
                    p1: <$vec>::new(self.p1.x.max(p.x), self.p1.y.max(p.y)),
                }
            }

            pub fn join(&self, other: Self) -> Self {
                Self {
                    p0: <$vec>::new(self.p0.x.min(other.p0.x), self.p0.y.min(other.p0.y)),
                    p1: <$vec>::new(self.p1.x.max(other.p1.x), self.p1.y.max(other.p1.y)),
                }
            }

            pub fn intersect(&self, other: Self) -> Self {
                debug_assert!(self.valid() && other.valid());

                let p0x = self.p0.x.max(other.p0.x);
                let p0y = self.p0.y.max(other.p0.y);
                let p1x = self.p1.x.min(other.p1.x).max(p0x);
                let p1y = self.p1.y.min(other.p1.y).max(p0y);

                Self {
                    p0: <$vec>::new(p0x, p0y),
                    p1: <$vec>::new(p1x, p1y),
                }
            }

            pub fn intersect_if_valid(&self, other: Self) -> Self {
                if !self.valid() || !other.valid() {
                    return *self;
                }

                self.intersect(other)
            }

            pub const fn intersects(&self, other: Self) -> bool {
                self.p0.0.x < other.p1.0.x
                    && self.p1.0.x > other.p0.0.x
                    && self.p0.0.y < other.p1.0.y
                    && self.p1.0.y > other.p0.0.y
            }

            pub const fn contains(&self, point: $vec) -> bool {
                point.0.x >= self.p0.0.x
                    && point.0.x <= self.p1.0.x
                    && point.0.y >= self.p0.0.y
                    && point.0.y <= self.p1.0.y
            }

            pub const fn valid(&self) -> bool {
                self.p0.0.x <= self.p1.0.x && self.p0.0.y <= self.p1.0.y
            }
        }
    };
}

impl_rectangle!(Vec2, f32, -1.0f32, 2.0f32);
impl_rectangle!(IVec2, i32, -1i32, 2i32);

impl Eq for Rectanglei {}
