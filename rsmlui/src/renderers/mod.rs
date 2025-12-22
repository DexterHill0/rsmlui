crate::export_interfaces! {
    default: DefaultRenderInterface;

    #[cfg(feature = "renderer-gl2")]
    gl2::RendererGl2
}
