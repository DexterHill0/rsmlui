use std::mem::transmute;

use glam::{IVec2, Mat4, Vec2};
use rsmlui_macros::rmldoc;
use rsmlui_sys::core;
use rsmlui_sys::interfaces::Opaque;
use rsmlui_sys::render_interface::{
    RenderInterfaceBridge, RmlRenderInterface, RustRenderInterface, new_rust_render_interface,
    render_interface_default_compile_filter, render_interface_default_compile_shader,
    render_interface_default_composite_layers, render_interface_default_enable_clip_mask,
    render_interface_default_pop_layer, render_interface_default_push_layer,
    render_interface_default_release_filter, render_interface_default_release_shader,
    render_interface_default_render_shader, render_interface_default_render_to_clip_mask,
    render_interface_default_save_layer_as_mask_image,
    render_interface_default_save_layer_as_texture, render_interface_default_set_transform,
    rust_render_interface_destructor,
};
use sealed::sealed;

use crate::containers::dictionary::Dictionary;
use crate::interfaces::{InterfaceHandle, IntoRawInterface, OwnedInterface, RawInterface};
use crate::types::aliases::{BlendMode, ClipMaskOperation};
use crate::types::colour::Colorb;
use crate::types::handles::{
    CompiledFilterHandle, CompiledGeometryHandle, CompiledShaderHandle, LayerHandle, TextureHandle,
};
use crate::types::rectangle::Rectanglei;
use crate::types::renderer::{ColorStopList, ColorStops, Vertex};
use crate::utils::conversions::{FromSys, IntoSys};

/// The receiver type for all [`RenderInterface`] methods.
pub type RenderInterfaceHandle<T> = InterfaceHandle<T, RustRenderInterface>;

/// An owned, heap-pinned render interface. Construct via [`OwnedInterface::new`].
pub type OwnedRenderInterface<T> = OwnedInterface<T, RustRenderInterface>;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterKind {
    Opacity {
        value: f32,
    },
    Blur {
        sigma: f32,
    },
    DropShadow {
        sigma: f32,
        color: Colorb,
        offset: Vec2,
    },
    Brightness {
        value: f32,
    },
    Contrast {
        value: f32,
    },
    Invert {
        value: f32,
    },
    Grayscale {
        value: f32,
    },
    Sepia {
        value: f32,
    },
    HueRotate {
        radians: f32,
    },
    Saturate {
        value: f32,
    },
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum ShaderKind {
    LinearGradient {
        repeating: bool,
        p0: Vec2,
        p1: Vec2,
        length: f32,
        color_stop_list: ColorStopList,
    },
    RadialGradient {
        repeating: bool,
        center: Vec2,
        radius: Vec2,
        color_stop_list: ColorStopList,
    },
    ConicGradient {
        repeating: bool,
        center: Vec2,
        angle: f32,
        color_stop_list: ColorStopList,
    },
    Custom {
        value: String,
        dimensions: Vec2,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadedTexture {
    pub dimensions: IVec2,
    pub handle: TextureHandle,
}

/// Implement this trait to create a custom [`Rml::RenderInterface`] for RmlUi.
///
/// Every method has a default implementation that forwards to the C++ base class behaviour.
///
/// Methods receive `self: &mut RenderInterfaceHandle<Self>` rather than `&mut self` in order
/// to ensure correct initialisation and lifetime of values.
/// [`RenderInterfaceHandle`] implements [`Deref`] and [`DerefMut`] to give access to the
/// underlying user data.
///
/// ## Dyn-compatibility
///
/// Every method is marked `where Self: Sized`, which keeps the trait dyn-compatible. Therefore,
/// methods are not callable through a `dyn RenderInterface` trait object. Dispatch must always
/// go through the [`InterfaceHandle`].
///
/// [`Deref`]: std::ops::Deref
/// [`DerefMut`]: std::ops::DerefMut
/// [`Rml::RenderInterface`]: https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/interfaces/renderer.html
#[rmldoc(file = "api_Rml-RenderInterface.md", name = "Rml::RenderInterface")]
pub trait RenderInterface {
    #[rmldoc(name = "Rml::RenderInterface::CompileGeometry")]
    fn compile_geometry(
        self: &mut RenderInterfaceHandle<Self>,
        vertices: &[Vertex],
        indices: &[i32],
    ) -> CompiledGeometryHandle
    where
        Self: Sized;

    #[rmldoc(name = "Rml::RenderInterface::RenderGeometry")]
    fn render_geometry(
        self: &mut RenderInterfaceHandle<Self>,
        geometry: CompiledGeometryHandle,
        translation: Vec2,
        texture: TextureHandle,
    ) where
        Self: Sized;

    #[rmldoc(name = "Rml::RenderInterface::ReleaseGeometry")]
    fn release_geometry(self: &mut RenderInterfaceHandle<Self>, geometry: CompiledGeometryHandle)
    where
        Self: Sized;

    #[rmldoc(name = "Rml::RenderInterface::LoadTexture")]
    fn load_texture(self: &mut RenderInterfaceHandle<Self>, source: &str) -> Option<LoadedTexture>
    where
        Self: Sized;

    #[rmldoc(name = "Rml::RenderInterface::GenerateTexture")]
    fn generate_texture(
        self: &mut RenderInterfaceHandle<Self>,
        source: &[u8],
        texture_dimensions: IVec2,
    ) -> Option<TextureHandle>
    where
        Self: Sized;

    #[rmldoc(name = "Rml::RenderInterface::ReleaseTexture")]
    fn release_texture(self: &mut RenderInterfaceHandle<Self>, texture: TextureHandle)
    where
        Self: Sized;

    #[rmldoc(name = "Rml::RenderInterface::EnableScissorRegion")]
    fn enable_scissor_region(self: &mut RenderInterfaceHandle<Self>, enable: bool)
    where
        Self: Sized;

    #[rmldoc(name = "Rml::RenderInterface::SetScissorRegion")]
    fn set_scissor_region(self: &mut RenderInterfaceHandle<Self>, region: Rectanglei)
    where
        Self: Sized;

    #[rmldoc(name = "Rml::RenderInterface::EnableClipMask")]
    fn enable_clip_mask(self: &mut RenderInterfaceHandle<Self>, enable: bool)
    where
        Self: Sized,
    {
        unsafe { render_interface_default_enable_clip_mask(self.bridge_ptr(), enable) }
    }

    #[rmldoc(name = "Rml::RenderInterface::RenderToClipMask")]
    fn render_to_clip_mask(
        self: &mut RenderInterfaceHandle<Self>,
        operation: ClipMaskOperation,
        geometry: CompiledGeometryHandle,
        translation: Vec2,
    ) where
        Self: Sized,
    {
        unsafe {
            render_interface_default_render_to_clip_mask(
                self.bridge_ptr(),
                operation,
                geometry,
                translation.into_sys(),
            )
        }
    }

    #[rmldoc(name = "Rml::RenderInterface::SetTransform")]
    fn set_transform(self: &mut RenderInterfaceHandle<Self>, transform: &Mat4)
    where
        Self: Sized,
    {
        unsafe {
            render_interface_default_set_transform(
                self.bridge_ptr(),
                <&Mat4 as IntoSys<&rsmlui_sys::Rml_Matrix4f>>::into_sys(transform) as *const _,
            )
        }
    }

    #[rmldoc(name = "Rml::RenderInterface::PushLayer")]
    fn push_layer(self: &mut RenderInterfaceHandle<Self>) -> LayerHandle
    where
        Self: Sized,
    {
        unsafe { render_interface_default_push_layer(self.bridge_ptr()) }
    }

    #[rmldoc(name = "Rml::RenderInterface::CompositeLayers")]
    fn composite_layers(
        self: &mut RenderInterfaceHandle<Self>,
        source: LayerHandle,
        destination: LayerHandle,
        blend_mode: BlendMode,
        filters: &[CompiledFilterHandle],
    ) where
        Self: Sized,
    {
        unsafe {
            render_interface_default_composite_layers(
                self.bridge_ptr(),
                source,
                destination,
                blend_mode,
                filters,
            )
        }
    }

    #[rmldoc(name = "Rml::RenderInterface::PopLayer")]
    fn pop_layer(self: &mut RenderInterfaceHandle<Self>)
    where
        Self: Sized,
    {
        unsafe { render_interface_default_pop_layer(self.bridge_ptr()) }
    }

    #[rmldoc(name = "Rml::RenderInterface::SaveLayerAsTexture")]
    fn save_layer_as_texture(self: &mut RenderInterfaceHandle<Self>) -> TextureHandle
    where
        Self: Sized,
    {
        unsafe { render_interface_default_save_layer_as_texture(self.bridge_ptr()) }
    }

    #[rmldoc(name = "Rml::RenderInterface::SaveLayerAsMaskImage")]
    fn save_layer_as_mask_image(self: &mut RenderInterfaceHandle<Self>) -> CompiledFilterHandle
    where
        Self: Sized,
    {
        unsafe { render_interface_default_save_layer_as_mask_image(self.bridge_ptr()) }
    }

    #[rmldoc(name = "Rml::RenderInterface::CompileFilter")]
    fn compile_filter(
        self: &mut RenderInterfaceHandle<Self>,
        kind: FilterKind,
    ) -> CompiledFilterHandle
    where
        Self: Sized,
    {
        let name = match kind {
            FilterKind::Opacity { .. } => "opacity",
            FilterKind::Blur { .. } => "blur",
            FilterKind::DropShadow { .. } => "drop-shadow",
            FilterKind::Brightness { .. } => "brightness",
            FilterKind::Contrast { .. } => "contrast",
            FilterKind::Invert { .. } => "invert",
            FilterKind::Grayscale { .. } => "grayscale",
            FilterKind::Sepia { .. } => "sepia",
            FilterKind::HueRotate { .. } => "hue-rotate",
            FilterKind::Saturate { .. } => "saturate",
        };

        // FIXME: null-ptr is okay because it's unused in the default, but still not ideal
        unsafe {
            render_interface_default_compile_filter(self.bridge_ptr(), name, std::ptr::null())
        }
    }

    #[rmldoc(name = "Rml::RenderInterface::ReleaseFilter")]
    fn release_filter(self: &mut RenderInterfaceHandle<Self>, filter: CompiledFilterHandle)
    where
        Self: Sized,
    {
        unsafe { render_interface_default_release_filter(self.bridge_ptr(), filter) }
    }

    #[rmldoc(name = "Rml::RenderInterface::CompileShader")]
    fn compile_shader(
        self: &mut RenderInterfaceHandle<Self>,
        kind: ShaderKind,
    ) -> CompiledShaderHandle
    where
        Self: Sized,
    {
        let name = match kind {
            ShaderKind::LinearGradient { .. } => "linear-gradient",
            ShaderKind::RadialGradient { .. } => "radial-gradient",
            ShaderKind::ConicGradient { .. } => "conic-gradient",
            ShaderKind::Custom { .. } => "shader",
        };

        // FIXME: null-ptr is okay because it's unused in the default, but still not ideal
        unsafe {
            render_interface_default_compile_shader(self.bridge_ptr(), name, std::ptr::null())
        }
    }

    #[rmldoc(name = "Rml::RenderInterface::RenderShader")]
    fn render_shader(
        self: &mut RenderInterfaceHandle<Self>,
        shader: CompiledShaderHandle,
        geometry: CompiledGeometryHandle,
        translation: Vec2,
        texture: TextureHandle,
    ) where
        Self: Sized,
    {
        unsafe {
            render_interface_default_render_shader(
                self.bridge_ptr(),
                shader,
                geometry,
                translation.into_sys(),
                texture,
            )
        }
    }

    #[rmldoc(name = "Rml::RenderInterface::ReleaseShader")]
    fn release_shader(self: &mut RenderInterfaceHandle<Self>, shader: CompiledShaderHandle)
    where
        Self: Sized,
    {
        unsafe { render_interface_default_release_shader(self.bridge_ptr(), shader) }
    }
}

// The sys crate uses `RenderInterfaceBridge` as the dispatch from C++ to Rust.
// This impl forwards each call to the user's `RenderInterface` implementation.
unsafe impl<T: RenderInterface> RenderInterfaceBridge for RenderInterfaceHandle<T> {
    #[inline]
    unsafe fn compile_geometry(
        &mut self,
        vertices: &[rsmlui_sys::Rml_Vertex],
        indices: &[i32],
    ) -> CompiledGeometryHandle {
        T::compile_geometry(self, FromSys::from_sys(vertices), indices)
    }

    #[inline]
    unsafe fn render_geometry(
        &mut self,
        geometry: rsmlui_sys::Rml_CompiledGeometryHandle,
        translation: rsmlui_sys::render_interface::Vector2f,
        texture: rsmlui_sys::Rml_TextureHandle,
    ) {
        T::render_geometry(self, geometry, FromSys::from_sys(translation), texture);
    }

    #[inline]
    unsafe fn release_geometry(&mut self, geometry: rsmlui_sys::Rml_CompiledGeometryHandle) {
        T::release_geometry(self, geometry);
    }

    #[inline]
    unsafe fn load_texture(
        &mut self,
        texture_dimensions: *mut rsmlui_sys::render_interface::Vector2i,
        source: &str,
    ) -> rsmlui_sys::Rml_TextureHandle {
        let texture = T::load_texture(self, source);

        match texture {
            Some(texture) => {
                // Safety: RmlUi gives us a valid pointer
                unsafe { *texture_dimensions = texture.dimensions.into_sys() }

                texture.handle
            },
            // As per docs:
            // > The value zero (0) is reserved for invalid handles, and should only be used to indicate an error while trying to load the texture.
            None => 0,
        }
    }

    #[inline]
    unsafe fn generate_texture(
        &mut self,
        source: &[u8],
        source_dimensions: rsmlui_sys::render_interface::Vector2i,
    ) -> rsmlui_sys::Rml_TextureHandle {
        let texture = T::generate_texture(self, source, FromSys::from_sys(source_dimensions));

        texture.unwrap_or(0)
    }

    #[inline]
    unsafe fn release_texture(&mut self, texture: rsmlui_sys::Rml_TextureHandle) {
        T::release_texture(self, texture);
    }

    #[inline]
    unsafe fn enable_scissor_region(&mut self, enable: bool) {
        T::enable_scissor_region(self, enable);
    }

    #[inline]
    unsafe fn set_scissor_region(&mut self, region: rsmlui_sys::render_interface::Rectanglei) {
        T::set_scissor_region(self, FromSys::from_sys(region));
    }

    #[inline]
    unsafe fn enable_clip_mask(&mut self, enable: bool) {
        T::enable_clip_mask(self, enable);
    }

    #[inline]
    unsafe fn render_to_clip_mask(
        &mut self,
        operation: rsmlui_sys::render_interface::ClipMaskOperation,
        geometry: rsmlui_sys::Rml_CompiledGeometryHandle,
        translation: rsmlui_sys::render_interface::Vector2f,
    ) {
        T::render_to_clip_mask(self, operation, geometry, FromSys::from_sys(translation));
    }

    #[inline]
    unsafe fn set_transform(&mut self, transform: *const rsmlui_sys::render_interface::Matrix4f) {
        // Safety: `transform` is a valid non-null pointer
        T::set_transform(self, FromSys::from_sys(unsafe { &*transform }));
    }

    #[inline]
    unsafe fn push_layer(&mut self) -> rsmlui_sys::Rml_LayerHandle {
        T::push_layer(self)
    }

    #[inline]
    unsafe fn composite_layers(
        &mut self,
        source: rsmlui_sys::Rml_LayerHandle,
        destination: rsmlui_sys::Rml_LayerHandle,
        blend_mode: rsmlui_sys::render_interface::BlendMode,
        filters: &[rsmlui_sys::Rml_CompiledFilterHandle],
    ) {
        T::composite_layers(self, source, destination, blend_mode, filters);
    }

    #[inline]
    unsafe fn pop_layer(&mut self) {
        T::pop_layer(self);
    }

    #[inline]
    unsafe fn save_layer_as_texture(&mut self) -> rsmlui_sys::Rml_TextureHandle {
        T::save_layer_as_texture(self)
    }

    #[inline]
    unsafe fn save_layer_as_mask_image(&mut self) -> rsmlui_sys::Rml_CompiledFilterHandle {
        T::save_layer_as_mask_image(self)
    }

    #[inline]
    unsafe fn compile_filter(
        &mut self,
        name: &str,
        parameters: *const rsmlui_sys::render_interface::Dictionary,
    ) -> rsmlui_sys::Rml_CompiledFilterHandle {
        // Safety: `parameters` is a valid non-null pointer
        let dictionary = unsafe { Dictionary::from_raw(parameters) };

        let kind = match name {
            "opacity" => {
                let value: &f32 = dictionary
                    .try_get("value")
                    .expect("missing `value` for opacity filter");

                FilterKind::Opacity { value: *value }
            },
            "blur" => {
                let sigma: &f32 = dictionary
                    .try_get("sigma")
                    .expect("missing `sigma` for blur filter");

                FilterKind::Blur { sigma: *sigma }
            },
            "brightness" => {
                let value: &f32 = dictionary
                    .try_get("value")
                    .expect("missing `value` for brightness filter");

                FilterKind::Brightness { value: *value }
            },
            "contrast" => {
                let value: &f32 = dictionary
                    .try_get("value")
                    .expect("missing `value` for contrast filter");

                FilterKind::Contrast { value: *value }
            },
            "invert" => {
                let value: &f32 = dictionary
                    .try_get("value")
                    .expect("missing `value` for invert filter");

                FilterKind::Invert { value: *value }
            },
            "grayscale" => {
                let value: &f32 = dictionary
                    .try_get("value")
                    .expect("missing `value` for grayscale filter");

                FilterKind::Grayscale { value: *value }
            },
            "sepia" => {
                let value: &f32 = dictionary
                    .try_get("value")
                    .expect("missing `value` for sepia filter");

                FilterKind::Sepia { value: *value }
            },
            "hue-rotate" => {
                let value: &f32 = dictionary
                    .try_get("value")
                    .expect("missing `value` for hue-rotate filter");

                FilterKind::HueRotate { radians: *value }
            },
            "saturate" => {
                let value: &f32 = dictionary
                    .try_get("value")
                    .expect("missing `value` for saturate filter");

                FilterKind::Saturate { value: *value }
            },
            "drop-shadow" => {
                let sigma: &f32 = dictionary
                    .try_get("sigma")
                    .expect("missing `sigma` for drop-shadow filter");
                let color: &Colorb = dictionary
                    .try_get("color")
                    .expect("missing `color` for drop-shadow filter");
                let offset: &Vec2 = dictionary
                    .try_get("offset")
                    .expect("missing `offset` for drop-shadow filter");

                FilterKind::DropShadow {
                    sigma: *sigma,
                    color: *color,
                    offset: *offset,
                }
            },
            _ => panic!("unknown filter name: {}", name),
        };

        T::compile_filter(self, kind)
    }

    #[inline]
    unsafe fn release_filter(&mut self, filter: rsmlui_sys::Rml_CompiledFilterHandle) {
        T::release_filter(self, filter);
    }

    #[inline]
    unsafe fn compile_shader(
        &mut self,
        name: &str,
        parameters: *const rsmlui_sys::render_interface::Dictionary,
    ) -> rsmlui_sys::Rml_CompiledShaderHandle {
        // Safety: `parameters` is a valid non-null pointer
        let dictionary = unsafe { Dictionary::from_raw(parameters) };

        let kind = match name {
            "linear-gradient" => {
                let repeating: &bool = dictionary
                    .try_get("repeating")
                    .expect("missing `repeating` for linear-gradient shader");
                let p0: &Vec2 = dictionary
                    .try_get("p0")
                    .expect("missing `p0` for linear-gradient shader");
                let p1: &Vec2 = dictionary
                    .try_get("p1")
                    .expect("missing `p1` for linear-gradient shader");
                let length: &f32 = dictionary
                    .try_get("length")
                    .expect("missing `length` for linear-gradient shader");
                let color_stop_list: &ColorStops = dictionary
                    .try_get("color_stop_list")
                    .expect("missing `color_stop_list` for linear-gradient shader");

                ShaderKind::LinearGradient {
                    repeating: *repeating,
                    p0: *p0,
                    p1: *p1,
                    length: *length,
                    color_stop_list: color_stop_list.to_vec(),
                }
            },
            "radial-gradient" => {
                let repeating: &bool = dictionary
                    .try_get("repeating")
                    .expect("missing `repeating` for radial-gradient shader");
                let center: &Vec2 = dictionary
                    .try_get("center")
                    .expect("missing `center` for radial-gradient shader");
                let radius: &Vec2 = dictionary
                    .try_get("radius")
                    .expect("missing `radius` for radial-gradient shader");

                let color_stop_list: &ColorStops = dictionary
                    .try_get("color_stop_list")
                    .expect("missing `color_stop_list` for radial-gradient shader");

                ShaderKind::RadialGradient {
                    repeating: *repeating,
                    center: *center,
                    radius: *radius,
                    color_stop_list: color_stop_list.to_vec(),
                }
            },
            "conic-gradient" => {
                let repeating: &bool = dictionary
                    .try_get("repeating")
                    .expect("missing `repeating` for conic-gradient shader");
                let center: &Vec2 = dictionary
                    .try_get("center")
                    .expect("missing `center` for conic-gradient shader");
                let angle: &f32 = dictionary
                    .try_get("angle")
                    .expect("missing `angle` for conic-gradient shader");
                let color_stop_list: &ColorStops = dictionary
                    .try_get("color_stop_list")
                    .expect("missing `color_stop_list` for conic-gradient shader");

                ShaderKind::ConicGradient {
                    repeating: *repeating,
                    center: *center,
                    angle: *angle,
                    color_stop_list: color_stop_list.to_vec(),
                }
            },
            "shader" => {
                let dimensions: &Vec2 = dictionary
                    .try_get("dimensions")
                    .expect("missing `dimensions` for custom shader");
                let value: &str = dictionary
                    .try_get("value")
                    .expect("missing `value` for custom shader");

                ShaderKind::Custom {
                    value: value.to_string(),
                    dimensions: *dimensions,
                }
            },
            _ => panic!("unknown shader name: {}", name),
        };

        T::compile_shader(self, kind)
    }

    #[inline]
    unsafe fn render_shader(
        &mut self,
        shader: rsmlui_sys::Rml_CompiledShaderHandle,
        geometry: rsmlui_sys::Rml_CompiledGeometryHandle,
        translation: rsmlui_sys::render_interface::Vector2f,
        texture: rsmlui_sys::Rml_TextureHandle,
    ) {
        T::render_shader(
            self,
            shader,
            geometry,
            FromSys::from_sys(translation),
            texture,
        );
    }

    #[inline]
    unsafe fn release_shader(&mut self, shader: rsmlui_sys::Rml_CompiledShaderHandle) {
        T::release_shader(self, shader);
    }
}

#[sealed]
impl<T: RenderInterface> super::OwnedInterfaceHandle<RustRenderInterface> for T {
    fn init_bridge(handle: &mut RenderInterfaceHandle<T>) {
        // The fat pointer data component is the address of the heap-allocated InterfaceHandle.
        // That address is stable for the lifetime of the OwnedInterface.
        let fat_ptr: *mut dyn RenderInterfaceBridge = handle;

        let (data, meta) = fat_ptr.to_raw_parts();

        let meta_raw: *const () = unsafe { transmute(meta) };

        let cpp =
            unsafe { new_rust_render_interface(meta_raw as *const Opaque, data as *mut Opaque) };

        handle.bridge = cpp;
    }

    unsafe fn destroy(handle: &mut RenderInterfaceHandle<T>) {
        unsafe { rust_render_interface_destructor(handle.bridge_ptr()) }
    }

    fn assert_not_registered(handle: &InterfaceHandle<Self, RustRenderInterface>) {
        let current_interface_ptr = core::get_render_interface();

        let self_ptr = unsafe { handle.bridge_ptr() }.cast();

        assert_ne!(
            current_interface_ptr, self_ptr,
            "OwnedInterface dropped while still registered as RmlUI's render interface"
        );
    }
}

// Implemented on a shared borrow so the value and C++ object outlives the pointer.
impl<T: RenderInterface> IntoRawInterface<RmlRenderInterface>
    for &OwnedInterface<T, RustRenderInterface>
{
    fn into_raw(self) -> RawInterface<RmlRenderInterface> {
        // `RustRenderInterface` is a subclass of `RmlRenderInterface` so the cast is valid.
        RawInterface::new(self.as_sys_ptr().cast())
    }
}
