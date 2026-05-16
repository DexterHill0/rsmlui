<!-- rsmlui:block["Rml::RenderInterface"] refid="class_rml_1_1_render_interface" -->

The abstract base class for application-specific rendering implementation. Your application must provide a concrete implementation of this class and install it through Rml::SetRenderInterface() in order for anything to be rendered.

C++ include:

```cpp
#include <RmlUi/Core/RenderInterface.h>
```

C++ inheritance: `Rml::NonCopyMoveable`
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::CompileGeometry"] refid="class_rml_1_1_render_interface_1af99ea4cb703dcf51289a2939d134e743" -->

Called by RmlUi when it wants to compile geometry to be rendered later. 
#### Parameters
* `vertices` The geometry's vertex data. 

* `indices` The geometry's index data. 

#### Returns
An application-specified handle to the geometry, or zero if it could not be compiled. @lifetime The pointed-to vertex and index data are guaranteed to be valid and immutable until [ReleaseGeometry()](#releasegeometry) is called with the geometry handle returned here.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::RenderGeometry"] refid="class_rml_1_1_render_interface_1a876ade728607b16304292dd9b8878ccf" -->

Called by RmlUi when it wants to render geometry. 
#### Parameters
* `geometry` The geometry to render. 

* `translation` The translation to apply to the geometry. 

* `texture` The texture to be applied to the geometry, or zero if the geometry is untextured.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::ReleaseGeometry"] refid="class_rml_1_1_render_interface_1ae8a77b2d7c430c96c8086a18e31aeecd" -->

Called by RmlUi when it wants to release geometry. 
#### Parameters
* `geometry` The geometry to release.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::LoadTexture"] refid="class_rml_1_1_render_interface_1a36c7d042bf7c45a09f6dad76708d5f82" -->

Called by RmlUi when a texture is required by the library. 
#### Parameters
* `texture_dimensions` The dimensions of the loaded texture, which must be set by the application. 

* `source` The application-defined image source, joined with the path of the referencing document. 

#### Returns
An application-specified handle identifying the texture, or zero if it could not be loaded.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::GenerateTexture"] refid="class_rml_1_1_render_interface_1a356b2bbf7e99c075bf6016bbac189c4d" -->

Called by RmlUi when a texture is required to be generated from a sequence of pixels in memory. 
#### Parameters
* `source` The raw texture data. Each pixel is made up of four 8-bit values, red, green, blue, and premultiplied alpha, in that order. 

* `source_dimensions` The dimensions, in pixels, of the source data. 

#### Returns
An application-specified handle identifying the texture, or zero if it could not be generated.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::ReleaseTexture"] refid="class_rml_1_1_render_interface_1a104f87b15d682559ecd1706ac72a0e35" -->

Called by RmlUi when a loaded or generated texture is no longer required. 
#### Parameters
* `texture` The texture handle to release.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::EnableScissorRegion"] refid="class_rml_1_1_render_interface_1ac370c947a3a20f448b8e08c468c23b8e" -->

Called by RmlUi when it wants to enable or disable scissoring to clip content. 
#### Parameters
* `enable` True to enable scissoring, false to disable it.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::SetScissorRegion"] refid="class_rml_1_1_render_interface_1a405f512b76c8cfc840a278695b43516e" -->

Called by RmlUi when it wants to change the scissor region. 
#### Parameters
* `region` The region to be rendered. All pixels outside this region should be clipped. 

<div class="warning">

The region should be applied in window coordinates regardless of any active transform. 

</div>


<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::EnableClipMask"] refid="class_rml_1_1_render_interface_1a4495ae0a826dd99408163b52024659c8" -->

Called by RmlUi when it wants to enable or disable the clip mask. 
#### Parameters
* `enable` True to enable the clip mask, false to disable it.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::RenderToClipMask"] refid="class_rml_1_1_render_interface_1a19e13c9bc48cb09174025cae01f5647d" -->

Called by RmlUi when it wants to set or modify the contents of the clip mask. 
#### Parameters
* `operation` Describes how the geometry should affect the clip mask. 

* `geometry` The compiled geometry to render. 

* `translation` The translation to apply to the geometry. 

<div class="warning">

When enabled, the clip mask should hide any rendered contents outside the area of the mask. 

</div>


<div class="warning">

The clip mask applies exclusively to all other functions that render with a geometry handle, in addition to the `[CompositeLayers](#compositelayers)` function while rendering to its destination. 

</div>


<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::SetTransform"] refid="class_rml_1_1_render_interface_1a5cbc5807aeefaaad4f92589ac90f9b2d" -->

Called by RmlUi when it wants the renderer to use a new transform matrix. 
#### Parameters
* `transform` The new transform to apply, or nullptr if no transform applies to the current element. 

<div class="warning">

When nullptr is submitted, the renderer should use an identity transform matrix or otherwise omit the multiplication with the transform. 

</div>


<div class="warning">

The transform applies to all functions that render with a geometry handle, and only those. 

</div>


<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::PushLayer"] refid="class_rml_1_1_render_interface_1a13512a6e677f1743b03a91cf0e537745" -->

Called by RmlUi when it wants to push a new layer onto the render stack, setting it as the new render target. 
#### Returns
An application-specified handle representing the new layer. The value 'zero' is reserved for the initial base layer. 

<div class="warning">

The new layer should be initialized to transparent black within the current scissor region. 

</div>


<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::CompositeLayers"] refid="class_rml_1_1_render_interface_1a9cb814953e14f62c5a0967da3a94d5df" -->

Composite two layers with the given blend mode and apply filters. 
#### Parameters
* `source` The source layer. 

* `destination` The destination layer. 

* `blend_mode` The mode used to blend the source layer onto the destination layer. 

* `filters` A list of compiled filters which should be applied before blending. 

<div class="warning">

Source and destination can reference the same layer. 

</div>


<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::PopLayer"] refid="class_rml_1_1_render_interface_1a67d144e11fa68339dbf597b010ab233a" -->
Called by RmlUi when it wants to pop the render layer stack, setting the new top layer as the render target.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::SaveLayerAsTexture"] refid="class_rml_1_1_render_interface_1a4c3f312d4455ebe52fe6bbc99eb57868" -->

Called by RmlUi when it wants to store the current layer as a new texture to be rendered later with geometry. 
#### Returns
An application-specified handle to the new texture. 

<div class="warning">

The texture should be extracted using the bounds defined by the active scissor region, thereby matching its size. 

</div>


<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::SaveLayerAsMaskImage"] refid="class_rml_1_1_render_interface_1a51dcc9c87746b8a7ed254064506a3f39" -->

Called by RmlUi when it wants to store the current layer as a mask image, to be applied later as a filter. 
#### Returns
An application-specified handle to a new filter representing the stored mask image.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::CompileFilter"] refid="class_rml_1_1_render_interface_1a38c3fad4cf034d9d7278d9e04b03004a" -->

Called by RmlUi when it wants to compile a new filter. 
#### Parameters
* `name` The name of the filter. 

* `parameters` The list of name-value parameters specified for the filter. 

#### Returns
An application-specified handle representing the compiled filter.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::ReleaseFilter"] refid="class_rml_1_1_render_interface_1a3a4ca46fc750598be6ec471bbd450a09" -->

Called by RmlUi when it no longer needs a previously compiled filter. 
#### Parameters
* `filter` The handle to a previously compiled filter.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::CompileShader"] refid="class_rml_1_1_render_interface_1a09e2bf35c4a89e10fbbe590e05389ea5" -->

Called by RmlUi when it wants to compile a new shader. 
#### Parameters
* `name` The name of the shader. 

* `parameters` The list of name-value parameters specified for the filter. 

#### Returns
An application-specified handle representing the shader.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::RenderShader"] refid="class_rml_1_1_render_interface_1a344489200dc4c19d9739f4d784f4ddf8" -->

Called by RmlUi when it wants to render geometry using the given shader. 
#### Parameters
* `shader` The handle to a previously compiled shader. 

* `geometry` The handle to a previously compiled geometry. 

* `translation` The translation to apply to the geometry. 

* `texture` The texture to use when rendering the geometry, or zero for no texture.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RenderInterface::ReleaseShader"] refid="class_rml_1_1_render_interface_1a63b2208fc2400024d7cdffac9483943d" -->

Called by RmlUi when it no longer needs a previously compiled shader. 
#### Parameters
* `shader` The handle to a previously compiled shader.

<!-- /rsmlui:block -->

