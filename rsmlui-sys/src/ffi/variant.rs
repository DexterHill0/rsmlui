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

        fn new_variant() -> *mut Variant;

        /// # Safety
        ///
        /// - `variant` must be a valid, non-null pointer returned by [`new_variant`].
        unsafe fn variant_destructor(variant: *mut Variant);

        unsafe fn variant_get_type(variant: *const Variant) -> VariantType;
        unsafe fn variant_clear(variant: *mut Variant);

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

        unsafe fn variant_set_bool(variant: *mut Variant, value: bool);
        unsafe fn variant_set_byte(variant: *mut Variant, value: u8);
        unsafe fn variant_set_char(variant: *mut Variant, value: i8);
        unsafe fn variant_set_float(variant: *mut Variant, value: f32);
        unsafe fn variant_set_double(variant: *mut Variant, value: f64);
        unsafe fn variant_set_int(variant: *mut Variant, value: i32);
        unsafe fn variant_set_int64(variant: *mut Variant, value: i64);
        unsafe fn variant_set_uint(variant: *mut Variant, value: u32);
        unsafe fn variant_set_uint64(variant: *mut Variant, value: u64);
        unsafe fn variant_set_vector2f(variant: *mut Variant, value: Vector2f);
        unsafe fn variant_set_vector3f(variant: *mut Variant, value: Vector3f);
        unsafe fn variant_set_vector4f(variant: *mut Variant, value: Vector4f);
        unsafe fn variant_set_colorb(variant: *mut Variant, value: Colorb);
        unsafe fn variant_set_color_stop_list(variant: *mut Variant, value: &[ColorStop]);
        unsafe fn variant_set_str(variant: *mut Variant, value: &str);
        unsafe fn variant_set_variant(variant: *mut Variant, value: *const Variant);
    }
}

pub use ffi::{
    Variant, VariantType, Vector2f, Vector3f, Vector4f, new_variant, variant_as_bool,
    variant_as_byte, variant_as_char, variant_as_color_stop_list, variant_as_colorb,
    variant_as_double, variant_as_float, variant_as_int, variant_as_int64, variant_as_str,
    variant_as_uint, variant_as_uint64, variant_as_vector2f, variant_as_vector3f,
    variant_as_vector4f, variant_clear, variant_destructor, variant_get_type, variant_set_bool,
    variant_set_byte, variant_set_char, variant_set_color_stop_list, variant_set_colorb,
    variant_set_double, variant_set_float, variant_set_int, variant_set_int64, variant_set_str,
    variant_set_uint, variant_set_uint64, variant_set_variant, variant_set_vector2f,
    variant_set_vector3f, variant_set_vector4f,
};
