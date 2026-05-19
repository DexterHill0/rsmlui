#[cxx::bridge]
mod ffi {

    #[namespace = "Rml"]
    unsafe extern "C++" {
        type Variant;

        type Vector2f = crate::Rml_Vector2f;
        type Vector3f = crate::Rml_Vector3f;
        type Vector4f = crate::Rml_Vector4f;

        #[cxx_name = "Colourb"]
        type Colorb = crate::Rml_Colourb;
        type ColorStop = crate::Rml_ColorStop;
    }

    #[namespace = "Rml::Variant"]
    unsafe extern "C++" {
        #[cxx_name = "Type"]
        type VariantType = crate::Rml_Variant_Type;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Variant.h");

        unsafe fn variant_get_type(variant: *const Variant) -> VariantType;

        unsafe fn variant_as_bool<'a>(variant: *const Variant) -> &'a bool;
        unsafe fn variant_as_byte<'a>(variant: *const Variant) -> &'a u8;
        unsafe fn variant_as_char<'a>(variant: *const Variant) -> &'a i8;
        unsafe fn variant_as_float<'a>(variant: *const Variant) -> &'a f32;
        unsafe fn variant_as_double<'a>(variant: *const Variant) -> &'a f64;
        unsafe fn variant_as_int<'a>(variant: *const Variant) -> &'a i32;
        unsafe fn variant_as_int64<'a>(variant: *const Variant) -> &'a i64;
        unsafe fn variant_as_uint<'a>(variant: *const Variant) -> &'a u32;
        unsafe fn variant_as_uint64<'a>(variant: *const Variant) -> &'a u64;

        unsafe fn variant_as_vector2f<'a>(variant: *const Variant) -> &'a Vector2f;
        unsafe fn variant_as_vector3f<'a>(variant: *const Variant) -> &'a Vector3f;
        unsafe fn variant_as_vector4f<'a>(variant: *const Variant) -> &'a Vector4f;

        unsafe fn variant_as_colorb<'a>(variant: *const Variant) -> &'a Colorb;
        unsafe fn variant_as_color_stop_list<'a>(variant: *const Variant) -> &'a [ColorStop];
        unsafe fn variant_as_str<'a>(variant: *const Variant) -> &'a str;
    }
}

pub use ffi::{
    Variant, VariantType, Vector2f, Vector3f, Vector4f, variant_as_bool, variant_as_byte,
    variant_as_char, variant_as_color_stop_list, variant_as_colorb, variant_as_double,
    variant_as_float, variant_as_int, variant_as_int64, variant_as_str, variant_as_uint,
    variant_as_uint64, variant_as_vector2f, variant_as_vector3f, variant_as_vector4f,
    variant_get_type,
};
