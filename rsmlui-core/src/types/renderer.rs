use bitflags::bitflags;
use rsmlui_macros::{rmldoc, sys_cast};
use rsmlui_sys::{
    Rml_BlendMode, Rml_ClipMaskOperation, Rml_ColorStop, Rml_NumericValue, Rml_Unit, Rml_Vertex,
};

use crate::math::Vec2;
use crate::types::colour::ColorbPremultiplied;

#[sys_cast(bitflags(from = Rml_Unit, repr = i32))]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Unit: i32 {
        const KEYWORD = 1 << 0;
        const STRING  = 1 << 1;
        const COLOUR  = 1 << 2;
        const RATIO   = 1 << 3;
        const NUMBER  = 1 << 4;
        const PERCENT = 1 << 5;
        const PX = 1 << 6;
        const DP = 1 << 7;
        const VW = 1 << 8;
        const VH = 1 << 9;
        const X = 1 << 10;
        const EM = 1 << 11;
        const REM = 1 << 12;
        const INCH = 1 << 13;
        const CM = 1 << 14;
        const MM = 1 << 15;
        const PT = 1 << 16;
        const PC = 1 << 17;
        const DEG = 1 << 18;
        const RAD = 1 << 19;
        const TRANSFORM = 1 << 20;
        const TRANSITION = 1 << 21;
        const ANIMATION = 1 << 22;
        const DECORATOR = 1 << 23;
        const FILTER = 1 << 24;
        const FONTEFFECT = 1 << 25;
        const COLORSTOPLIST = 1 << 26;
        const BOXSHADOWLIST = 1 << 27;
    }
}

impl Unit {
    // `BitOr` is not const
    pub const ANGLE: Unit = Self::DEG.union(Self::RAD);
    pub const DP_SCALABLE_LENGTH: Unit = Self::DP.union(Self::PPI_UNIT);
    pub const LENGTH: Unit = Self::PX
        .union(Self::DP)
        .union(Self::VW)
        .union(Self::VH)
        .union(Self::EM)
        .union(Self::REM)
        .union(Self::PPI_UNIT);
    pub const LENGTH_PERCENT: Unit = Self::LENGTH.union(Self::PERCENT);
    pub const NUMBER_LENGTH_PERCENT: Unit = Self::NUMBER.union(Self::LENGTH).union(Self::PERCENT);
    pub const NUMBER_PERCENT: Unit = Self::NUMBER.union(Self::PERCENT);
    pub const NUMERIC: Unit = Self::NUMBER_LENGTH_PERCENT
        .union(Self::ANGLE)
        .union(Self::X);
    pub const PPI_UNIT: Unit = Self::INCH
        .union(Self::CM)
        .union(Self::MM)
        .union(Self::PT)
        .union(Self::PC);
}

#[sys_cast(enum(from = Rml_BlendMode, repr = i32))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Blend = 0,
    Replace = 1,
}

#[sys_cast(enum(from = Rml_ClipMaskOperation, repr = i32))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ClipMaskOperation {
    Set = 0,
    SetInverse = 1,
    Intersect = 2,
}

#[sys_cast(struct(from = Rml_Vertex), gen_ref, gen_slice)]
#[rmldoc(file = "api_Rml-Vertex.md", name = "Rml::Vertex")]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vertex {
    #[rmldoc(name = "Rml::Vertex::position")]
    pub position: Vec2,
    #[rmldoc(name = "Rml::Vertex::colour")]
    pub colour: ColorbPremultiplied,
    #[rmldoc(name = "Rml::Vertex::tex_coord")]
    pub tex_coord: Vec2,
}

#[sys_cast(struct(from = Rml_NumericValue), gen_ref)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NumericValue {
    pub number: f32,
    pub unit: Unit,
}

#[sys_cast(struct(from = Rml_ColorStop), gen_ref, gen_slice)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorStop {
    pub(crate) color: ColorbPremultiplied,
    pub(crate) position: NumericValue,
}

pub type ColorStopList = Vec<ColorStop>;
pub type ColorStops = [ColorStop];
