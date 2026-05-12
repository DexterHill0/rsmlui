<!-- rsmlui:block["Rml"] refid="namespace_rml" -->

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ModalFlag"] refid="_element_document_8h_1a7d8ae93a68ffd74fa25f967d4ae4c113" -->

ModalFlag controls the modal state of the document.

Values:

| Name | Description |
|------|-------------|
| `None` |  |
| `Modal` |  |
| `Keep` |  |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::FocusFlag"] refid="_element_document_8h_1a80d27506b92bb58bb95ef2359f0d1c0f" -->

FocusFlag controls the focus when showing the document.

Values:

| Name | Description |
|------|-------------|
| `None` |  |
| `Document` |  |
| `Keep` |  |
| `Auto` |  |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ScrollFlag"] refid="_element_document_8h_1abbb633cc045674fda25a04e87e36b155" -->

ScrollFlag controls whether an element is scrolled into view when showing the document.

Values:

| Name | Description |
|------|-------------|
| `None` |  |
| `Auto` |  |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::CallbackTextureFunction"] refid="_callback_texture_8h_1a95c3efa093e9cef480822cc4c923b79e" -->

Callback function for generating textures on demand.
 /
#### Parameters
* `texture_interface` The interface used to specify the texture. / 

#### Returns
True on success.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Initialise"] refid="_core_2_core_8h_1a1bb1e1b9f9e28d75f6836826bb7b6cdb" -->
Initialises RmlUi.

RmlUi library core API.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Shutdown"] refid="_core_2_core_8h_1aa7aa401fc80ea801c2d0b681368835e2" -->
Shutdown RmlUi.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetVersion"] refid="_core_2_core_8h_1afef2061d8a3a13c9675f916c6118ed71" -->

Returns the version of this RmlUi library. 
#### Returns
The version number.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SetSystemInterface"] refid="_core_2_core_8h_1aa4fb6343ce82355f50266ec790839ebd" -->

Sets the interface through which all system requests are made. This is not required to be called, but if it is, it must be called before Initialise(). 
#### Parameters
* `system_interface` A non-owning pointer to the application-specified logging interface. @lifetime The interface must be kept alive until after the call to Rml::Shutdown.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetSystemInterface"] refid="_core_2_core_8h_1a31a01c8aede6d25c4cddf2e731d0f437" -->
Returns RmlUi's system interface.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SetRenderInterface"] refid="_core_2_core_8h_1a10038cd15f51b29b2ffc08f025d49c49" -->

Sets the interface through which all rendering requests are made. This is not required to be called, but if it is, it must be called before Initialise(). If no render interface is specified, then all contexts must specify a render interface when created. 
#### Parameters
* `render_interface` A non-owning pointer to the render interface implementation. @lifetime The interface must be kept alive until after the call to Rml::Shutdown.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetRenderInterface"] refid="_core_2_core_8h_1a5ea2002d8fc40f9f2216eb171d1ea125" -->
Returns RmlUi's default's render interface.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SetFileInterface"] refid="_core_2_core_8h_1af9083b722acf6333cc69ebcb7ae5bc82" -->

Sets the interface through which all file I/O requests are made. This is not required to be called, but if it is, it must be called before Initialise(). 
#### Parameters
* `file_interface` A non-owning pointer to the application-specified file interface. @lifetime The interface must be kept alive until after the call to Rml::Shutdown.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetFileInterface"] refid="_core_2_core_8h_1ad9b28d012c3ac15e6febc74fa869cdf8" -->
Returns RmlUi's file interface.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SetFontEngineInterface"] refid="_core_2_core_8h_1a3bb8eb0bfce73278801da3c1d28d906d" -->

Sets the interface through which all font requests are made. This is not required to be called, but if it is, it must be called before Initialise(). 
#### Parameters
* `font_interface` A non-owning pointer to the application-specified font engine interface. @lifetime The interface must be kept alive until after the call to Rml::Shutdown.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetFontEngineInterface"] refid="_core_2_core_8h_1a16a0a8f9750a57b970329788c4f4cf24" -->
Returns RmlUi's font interface.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SetTextInputHandler"] refid="_core_2_core_8h_1afd6cbe4f885b72a3adacf491bfc379a4" -->

Sets the implementation for handling text input events. This is not required to be called. 
#### Parameters
* `text_input_handler` A non-owning pointer to the application-specified implementation of a text input handler. @lifetime The instance must be kept alive until after the call to Rml::Shutdown. 

:::note
Be aware that you might be overriding a custom backend implementation. 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetTextInputHandler"] refid="_core_2_core_8h_1a307a260e18352d559d8ce8d573a91ba0" -->
Returns RmlUi's default implementation of a text input handler.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::CreateContext"] refid="_core_2_core_8h_1a8cca6c0159b5d735ed26fa83e7bdd20b" -->

Creates a new element context. 
#### Parameters
* `name` The new name of the context. This must be unique. 

* `dimensions` The initial dimensions of the new context. 

* `render_interface` The custom render interface to use, or nullptr to use the default. 

* `text_input_handler` The custom text input handler to use, or nullptr to use the default. @lifetime If specified, the render interface and the text input handler must be kept alive until after the call to Rml::Shutdown. Alternatively, the render interface can be destroyed after all contexts it belongs to have been destroyed, and a subsequent call has been made to Rml::ReleaseRenderManagers. 

#### Returns
A non-owning pointer to the new context, or nullptr if the context could not be created.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RemoveContext"] refid="_core_2_core_8h_1a9bf3571371235633c794c47612c38a22" -->

Removes and destroys a context. 
#### Parameters
* `name` The name of the context to remove. 

#### Returns
True if the name is a valid context, false otherwise.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetContext"] refid="_core_2_core_8h_1af7d55096db19246a79a89e3edea097b8" -->

Fetches a previously constructed context by name. 
#### Parameters
* `name` The name of the desired context. 

#### Returns
The desired context, or nullptr if no context exists with the given name.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetContext"] refid="_core_2_core_8h_1ace2615026febf77cfc717d44419985a1" -->

Fetches a context by index. 
#### Parameters
* `index` The index of the desired context. If this is outside the valid range of contexts, it will be clamped. 

#### Returns
The requested context, or nullptr if no contexts exist.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetNumContexts"] refid="_core_2_core_8h_1a8c887534730df4b8f52f1c6859eaf910" -->

Returns the number of active contexts. 
#### Returns
The total number of active RmlUi contexts.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::LoadFontFace"] refid="_core_2_core_8h_1a2dddbcab95a33c0d265b1aa09051c814" -->

Adds a new font face from file to the font engine. The face's family, style, and weight will be determined from the face itself. 
#### Parameters
* `file_path` The path to the file to load the face from. The path is passed directly to the file interface which is used to load the file. The default file interface accepts both absolute paths and paths relative to the working directory. 

* `fallback_face` True to use this font face for unknown characters in other font faces. 

* `weight` The weight to load when the font face contains multiple weights, otherwise the weight to register the font as. By default, it loads all found font weights. 

* `face_index` The index of the font face within a font collection. 

#### Returns
True if the face was loaded successfully, false otherwise.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::LoadFontFace"] refid="_core_2_core_8h_1a51c118ba84dbfd1cf419f515771988d5" -->

Adds a new font face from file to the font engine. The face's family, style, and weight are given by the parameters. 
#### Parameters
* `data` The font data. 

* `family` The family to register the font as. 

* `style` The style to register the font as. 

* `weight` The weight to load when the font face contains multiple weights, otherwise the weight to register the font as. By default, it loads all found font weights. 

* `fallback_face` True to use this font face for unknown characters in other font faces. 

* `face_index` The index of the font face within a font collection. 

#### Returns
True if the face was loaded successfully, false otherwise. @lifetime The pointed to 'data' must remain available until after the call to Rml::Shutdown.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::LoadFontFace"] refid="_core_2_core_8h_1ab0e7ea047d96afb96b9a5aefddbb929e" -->

Adds a new font face from memory to the font engine. The face's family, style, and weight are given by the parameters. 
#### Parameters
* `data` The font data. 

* `family` The family to register the font as. 

* `style` The style to register the font as. 

* `weight` The weight to load when the font face contains multiple weights, otherwise the weight to register the font as. By default, it loads all found font weights. 

* `fallback_face` True to use this font face for unknown characters in other font faces. 

* `face_index` The index of the font face within a font collection. 

#### Returns
True if the face was loaded successfully, false otherwise. @lifetime The pointed to 'data' must remain available until after the call to Rml::Shutdown.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RegisterPlugin"] refid="_core_2_core_8h_1a9b872792e1c356ef2e922ad0610a45d3" -->
Registers a generic RmlUi plugin.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::UnregisterPlugin"] refid="_core_2_core_8h_1a3fc74ccfa8de7f4b6feffd2c35884b7c" -->
Unregisters a generic RmlUi plugin.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::RegisterEventType"] refid="_core_2_core_8h_1ad52e61a1ee210aafdcb2a776c12e0049" -->

Registers a new event type. If the type already exists, it will replace custom event types, but not internal types. 
#### Parameters
* `type` The new event type. 

* `interruptible` Whether the event can be interrupted during dispatch. 

* `bubbles` Whether the event executes the bubble phase. If false, only capture and target phase is executed. 

* `default_action_phase` Defines during which phase(s) the 'Element::ProcessDefaultAction' method is called. 

#### Returns
The EventId of the newly created type, or existing type if 'type' is an internal type.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::GetTextureSourceList"] refid="_core_2_core_8h_1a54917ce87819f15d66b14beb84372981" -->
Returns a list of source URLs to textures in all loaded documents.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ReleaseTextures"] refid="_core_2_core_8h_1a7f70ae59934bbd8aaf4ef4a873203c19" -->

Forces all texture handles loaded and generated by RmlUi to be released. 
#### Parameters
* `render_interface` Release all textures belonging to the given interface, or nullptr to release all textures in all interfaces.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ReleaseTexture"] refid="_core_2_core_8h_1a8cb69f6551240db8aeb7879d56855322" -->

Releases a specified texture by name from memory, returning 'true' if successful and 'false' if not found. 
#### Parameters
* `source` The texture source to match. 

* `render_interface` Release any matching texture belonging to the given interface, or nullptr to look in all interfaces. 

#### Returns
True if any texture was released.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ReleaseCompiledGeometry"] refid="_core_2_core_8h_1aff05d0f270984655b34f84aacbf50cdb" -->

Forces all compiled geometry handles generated by RmlUi to be released. 
#### Parameters
* `render_interface` Release all geometry belonging to the given interface, or nullptr to release all geometry in all interfaces.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ReleaseFontResources"] refid="_core_2_core_8h_1a17d9207b91dbb06e6fb47056f487e774" -->

Releases unused font textures and rendered glyphs to free up memory, and regenerates actively used fonts. 
:::note
Invalidates all existing FontFaceHandles returned from the font engine. 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ReleaseRenderManagers"] refid="_core_2_core_8h_1a8ca4eb626d02c5cd512bb2a42b38bc62" -->

Releases render managers that are not used by any contexts. 
:::note
Any resources referring to the render manager in user space must be cleared first, including callback textures and compiled geometry. 

:::

:::note
Also releases font resources, which invalidates all existing FontFaceHandles returned from the font engine. 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::CreateString"] refid="_string_utilities_8h_1af6c8d29d70586432cc6ef562454563a8" -->
Construct a string using sprintf-style syntax.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::FormatString"] refid="_string_utilities_8h_1ad5ccf4050283d38ab3e0df8d817478fd" -->
Format to a string using sprintf-style syntax.

<!-- /rsmlui:block -->

