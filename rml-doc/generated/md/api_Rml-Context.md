<!-- rsmlui:block["Rml::Context"] refid="class_rml_1_1_context" -->

A context for storing, rendering, and processing RML documents. Multiple contexts can exist simultaneously.

C++ include:

```cpp
#include <RmlUi/Core/Context.h>
```

C++ inheritance: `Rml::ScriptInterface`
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::Context"] refid="class_rml_1_1_context_1a75321568adaff69b9a3c5190cc471fa9" -->

Constructs a new, uninitialised context. This should not be called directly, use CreateContext() instead. 
#### Parameters
* `name` The name of the context. 

* `render_manager` The render manager used for this context. 

* `text_input_handler` The text input handler used for this context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::~Context"] refid="class_rml_1_1_context_1a2ee8ff31e91997190866985e08bfcfbd" -->
Destroys a context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetName"] refid="class_rml_1_1_context_1adbef34c6572988f1e81dc63fe3cd03e3" -->

Returns the name of the context. 
#### Returns
The context's name.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::SetDimensions"] refid="class_rml_1_1_context_1ac399e689925b56f1afdc646b629052eb" -->

Changes the dimensions of the context. 
#### Parameters
* `dimensions` The new dimensions of the context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetDimensions"] refid="class_rml_1_1_context_1a6c54b0786a61ed58bf0508224e15d804" -->

Returns the dimensions of the context. 
#### Returns
The current dimensions of the context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::SetDensityIndependentPixelRatio"] refid="class_rml_1_1_context_1a7455d26627bc2ff5c75113c32f6adb76" -->

Changes the ratio of the 'dp' unit to the 'px' unit. 
#### Parameters
* `dp_ratio` The new density-independent pixel ratio of the context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetDensityIndependentPixelRatio"] refid="class_rml_1_1_context_1a5160b914e9f22268c9d3a92cebb5825b" -->

Returns the ratio of the 'dp' unit to the 'px' unit. 
#### Returns
The current density-independent pixel ratio of the context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::Update"] refid="class_rml_1_1_context_1ad29543c6850a1b1e3841ac90bfcfbab4" -->

Updates all elements in the context's documents. This must be called before [Context::Render](#render), but after any elements have been changed, added, or removed.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::Render"] refid="class_rml_1_1_context_1a1dc729e27308135518e5af163641c25e" -->
Renders all visible elements in the context's documents.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::CreateDocument"] refid="class_rml_1_1_context_1a9d6188808d3bef3dd66c56a222eb4d11" -->

Creates a new, empty document and places it into this context. 
#### Parameters
* `instancer_name` The name of the instancer used to create the document. 

#### Returns
The new document, or nullptr if no document could be created.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::LoadDocument"] refid="class_rml_1_1_context_1a2ba6bc9ce08f1ba834de53a207af79a3" -->

Load a document into the context. 
#### Parameters
* `document_path` The path to the document to load. The path is passed directly to the file interface which is used to load the file. The default file interface accepts both absolute paths and paths relative to the working directory. 

#### Returns
The loaded document, or nullptr if no document was loaded.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::LoadDocument"] refid="class_rml_1_1_context_1ada94807c223fb9fc11e0990a9ad17c6e" -->

Load a document into the context. 
#### Parameters
* `document_stream` The opened stream, ready to read. 

#### Returns
The loaded document, or nullptr if no document was loaded.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::LoadDocumentFromMemory"] refid="class_rml_1_1_context_1a96da560e1b409141b1a47014a868360a" -->

Load a document into the context. 
#### Parameters
* `document_rml` The string containing the document RML. 

* `source_url` Optional string used to set the document's source URL, or naming the document for log messages. 

#### Returns
The loaded document, or nullptr if no document was loaded.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::UnloadDocument"] refid="class_rml_1_1_context_1a029cdee43160f28bbd17dbe06b2faea1" -->

Unload the given document. 
#### Parameters
* `document` The document to unload. 

:::note
The destruction of the document is deferred until the next call to [Context::Update()](#update). 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::UnloadAllDocuments"] refid="class_rml_1_1_context_1a6713747bac32bed6d64cfdd5f07e0527" -->

Unloads all loaded documents. 
:::note
The destruction of the documents is deferred until the next call to [Context::Update()](#update). 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::EnableMouseCursor"] refid="class_rml_1_1_context_1a527177c0a51be83e1858bb2e9a9f5e9e" -->

Enable or disable handling of the mouse cursor from this context. When enabled, changes to the cursor name are passed to the system interface. 
#### Parameters
* `enable` True to enable mouse cursor handling, false to disable.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ActivateTheme"] refid="class_rml_1_1_context_1a2bb2a908875f6753a15f75dc4a3da35c" -->

Activate or deactivate a media theme. Themes can be used in RCSS media queries. 
#### Parameters
* `theme_name[in]` The name of the theme to (de)activate. 

* `activate` True to activate the given theme, false to deactivate.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::IsThemeActive"] refid="class_rml_1_1_context_1a6c9c8644470f72646d4badb631474db6" -->

Check if a given media theme has been activated. 
#### Parameters
* `theme_name` The name of the theme. 

#### Returns
True if the theme is activated.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetDocument"] refid="class_rml_1_1_context_1a7d6b32bdc34572c408afeaa277eaaed2" -->

Returns the first document in the context with the given id. 
#### Parameters
* `id` The id of the desired document. 

#### Returns
The document (if it was found), or nullptr if no document exists with the ID.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetDocument"] refid="class_rml_1_1_context_1a4bec1a25aab0c728de17bbbe966c14e0" -->

Returns a document in the context by index. 
#### Parameters
* `index` The index of the desired document. 

#### Returns
The document (if one exists with this index), or nullptr if the index was invalid.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetNumDocuments"] refid="class_rml_1_1_context_1af171c25c2d4b18135b790ab353aa319a" -->
Returns the number of documents in the context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetHoverElement"] refid="class_rml_1_1_context_1aa9d6e639dfb02c7a657f77e94a07b0ae" -->

Returns the hover element. 
#### Returns
The element the mouse cursor is hovering over.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetFocusElement"] refid="class_rml_1_1_context_1ab88fe6a3c283fd43616a0f3f02e3292c" -->

Returns the focus element. 
#### Returns
The element with input focus.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetRootElement"] refid="class_rml_1_1_context_1a7ecf37fe4ac64445238ce2cee0a244b2" -->

Returns the root element that holds all the documents 
#### Returns
The root element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetElementAtPoint"] refid="class_rml_1_1_context_1ab08d74bd72b1434a2faafdb714fd7685" -->

Returns the youngest descendent of the given element which is under the given point in screen coordinates. 
#### Parameters
* `point` The point to test. 

* `ignore_element` If set, this element and its descendents will be ignored. 

* `element` Used internally. 

#### Returns
The element under the point, or nullptr if nothing is.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::PullDocumentToFront"] refid="class_rml_1_1_context_1a7d0a98813383582ce526bb3111ef8cc4" -->

Brings the document to the front of the document stack. 
#### Parameters
* `document` The document to pull to the front of the stack.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::PushDocumentToBack"] refid="class_rml_1_1_context_1ae3ed2978969201c731913ad6de1a9b2d" -->

Sends the document to the back of the document stack. 
#### Parameters
* `document` The document to push to the bottom of the stack.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::UnfocusDocument"] refid="class_rml_1_1_context_1ae0a322f4341dc8e06ad2e0e60d0b21b6" -->

Remove the document from the focus history and focus the previous document. 
#### Parameters
* `document` The document to unfocus.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::AddEventListener"] refid="class_rml_1_1_context_1a565e031a43e97c209d2f796994e6eefa" -->

Adds an event listener to the context's root element. 
#### Parameters
* `event` The name of the event to attach to. 

* `listener` Listener object to be attached. 

* `in_capture_phase` True if the listener is to be attached to the capture phase, false for the bubble phase.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::RemoveEventListener"] refid="class_rml_1_1_context_1a596e8d8586b9dd11d1b6e11fbc062d35" -->

Removes an event listener from the context's root element. 
#### Parameters
* `event` The name of the event to detach from. 

* `listener` Listener object to be detached. 

* `in_capture_phase` True to detach from the capture phase, false from the bubble phase.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessKeyDown"] refid="class_rml_1_1_context_1ae1224bf04ab04c48ba04faa0b3e80c67" -->

Sends a key down event into this context. 
#### Parameters
* `key_identifier` The key pressed. 

* `key_modifier_state` The state of key modifiers (shift, control, caps-lock, etc.) keys; this should be generated by ORing together members of the Input::KeyModifier enumeration. 

#### Returns
True if the event was not consumed (ie, was prevented from propagating by an element), false if it was.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessKeyUp"] refid="class_rml_1_1_context_1ac46b9506c73212a3ac20387f2653f276" -->

Sends a key up event into this context. 
#### Parameters
* `key_identifier` The key released. 

* `key_modifier_state` The state of key modifiers (shift, control, caps-lock, etc.) keys; this should be generated by ORing together members of the Input::KeyModifier enumeration. 

#### Returns
True if the event was not consumed (ie, was prevented from propagating by an element), false if it was.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessTextInput"] refid="class_rml_1_1_context_1a8ebc3af157caeff7de77165716393171" -->

Sends a single Unicode character as text input into this context. 
#### Parameters
* `character` The Unicode code point to send into this context. 

#### Returns
True if the event was not consumed (ie, was prevented from propagating by an element), false if it was.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessTextInput"] refid="class_rml_1_1_context_1a1eb37c3339a44947644ed8f57ee5bb3e" -->
Sends a single ascii character as text input into this context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessTextInput"] refid="class_rml_1_1_context_1a2f34194ca96c72923accbc4c28c6cbb7" -->

Sends a string of text as text input into this context. 
#### Parameters
* `string` The UTF-8 string to send into this context. 

#### Returns
True if the event was not consumed (ie, was prevented from propagating by an element), false if it was.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessMouseMove"] refid="class_rml_1_1_context_1a01020500dd688677cdf739d8637ec912" -->

Sends a mouse movement event into this context. 
#### Parameters
* `x` The x-coordinate of the mouse cursor, in window-coordinates (ie, 0 should be the left of the client area). 

* `y` The y-coordinate of the mouse cursor, in window-coordinates (ie, 0 should be the top of the client area). 

* `key_modifier_state` The state of key modifiers (shift, control, caps-lock, etc.) keys; this should be generated by ORing together members of the Input::KeyModifier enumeration. 

#### Returns
True if the mouse is not interacting with any elements in the context (see 'IsMouseInteracting'), otherwise false.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessMouseButtonDown"] refid="class_rml_1_1_context_1afa1d18d8561a9cfb09ead898ab67dbe3" -->

Sends a mouse-button down event into this context. 
#### Parameters
* `button_index` The index of the button that was pressed. Left: 0, Right: 1, Middle: 2. 

* `key_modifier_state` The state of key modifiers (shift, control, caps-lock, etc.) keys; this should be generated by ORing together members of the Input::KeyModifier enumeration. 

#### Returns
True if the mouse is not interacting with any elements in the context (see 'IsMouseInteracting'), otherwise false.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessMouseButtonUp"] refid="class_rml_1_1_context_1ac371ccafa6fbd233d43962bd94e1c465" -->

Sends a mouse-button up event into this context. 
#### Parameters
* `button_index` The index of the button that was release. Left: 0, Right: 1, Middle: 2. 

* `key_modifier_state` The state of key modifiers (shift, control, caps-lock, etc.) keys; this should be generated by ORing together members of the Input::KeyModifier enumeration. 

#### Returns
True if the mouse is not interacting with any elements in the context (see 'IsMouseInteracting'), otherwise false.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessMouseWheel"] refid="class_rml_1_1_context_1a6ea62f6c76c0598c7dc0ae25b68df0d1" -->

Sends a mousescroll event into this context.

> Deprecated: Please use the Vector2f version of this function.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessMouseWheel"] refid="class_rml_1_1_context_1a3e3f4b6b4fa42e8a069f886849b14ac8" -->

Sends a mousescroll event into this context, and scrolls the document unless the event was stopped from propagating. 
#### Parameters
* `wheel_delta` The mouse-wheel movement this frame, with positive values being directed right and down. 

* `key_modifier_state` The state of key modifiers (shift, control, caps-lock, etc.) keys; this should be generated by ORing together members of the Input::KeyModifier enumeration. 

#### Returns
True if the event was not consumed (ie, was prevented from propagating by an element), false if it was.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessMouseLeave"] refid="class_rml_1_1_context_1a7a238de49ca01f4ddeab3382f58f9a37" -->

Tells the context the mouse has left the window. This removes any hover state from all elements and prevents '[Update()](#update)' from setting the hover state for elements under the mouse. 
#### Returns
True if the mouse is not interacting with any elements in the context (see 'IsMouseInteracting'), otherwise false. 

:::note
The mouse is considered activated again after the next call to '[ProcessMouseMove()](#processmousemove)'. 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessTouchMove"] refid="class_rml_1_1_context_1a601d814c3cff4864dd91108ba50cbc90" -->

Process touch movements for this context. 
#### Parameters
* `touches` List of touches. 

* `key_modifier_state` The state of key modifiers (shift, control, caps-lock, etc.) keys; this should be generated by ORing together members of the Input::KeyModifier enumeration. 

#### Returns
True if no touch points are interacting with any elements in the context, otherwise false.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessTouchStart"] refid="class_rml_1_1_context_1a2077cdb9e5cedd5ed609d144c191ce43" -->

Process touch start (press) for this context. 
#### Parameters
* `touches` List of touches. 

* `key_modifier_state` The state of key modifiers (shift, control, caps-lock, etc.) keys; this should be generated by ORing together members of the Input::KeyModifier enumeration. 

#### Returns
True if no touch points are interacting with any elements in the context, otherwise false.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessTouchEnd"] refid="class_rml_1_1_context_1afbaab5aeaf635112840b470371fd5dfb" -->

Process touch end (release) for this context. 
#### Parameters
* `touches` List of touches. 

* `key_modifier_state` The state of key modifiers (shift, control, caps-lock, etc.) keys; this should be generated by ORing together members of the Input::KeyModifier enumeration. 

#### Returns
True if no touch points are interacting with any elements in the context, otherwise false.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::ProcessTouchCancel"] refid="class_rml_1_1_context_1a0f72e5ad89fe17b605b08db4b4a8b73c" -->

Process touch cancel for this context. 
#### Parameters
* `touches` List of touches. 

#### Returns
True if no touch points are interacting with any elements in the context, otherwise false.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::IsMouseInteracting"] refid="class_rml_1_1_context_1a8db4a9a220d477420f2b429b1ba9601d" -->

Returns a hint on whether the mouse is currently interacting with any elements in this context, based on previously submitted 'ProcessMouse...()' commands. 
:::note
Interaction is determined irrespective of background and opacity. See the RCSS property 'pointer-events' to disable interaction for specific elements. 

:::

#### Returns
True if the mouse hovers over or has activated an element in this context, otherwise false.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::SetDefaultScrollBehavior"] refid="class_rml_1_1_context_1a729f538d3013a2c438ca2298c0897a1b" -->

Sets the default scroll behavior, such as for mouse wheel processing and scrollbar interaction. 
#### Parameters
* `scroll_behavior` The default smooth scroll behavior, set to instant to disable smooth scrolling. 

* `speed_factor` A factor for adjusting the final smooth scrolling speed, must be strictly positive, defaults to 1.0.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetRenderManager"] refid="class_rml_1_1_context_1ad40f449209fb5af359298eefbc84e44d" -->
Retrieves the render manager which can be used to submit changes to the render state.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetTextInputHandler"] refid="class_rml_1_1_context_1a9623dfa6bddb816dadc8cdf5ae7be9b3" -->
Retrieves the text input handler.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::SetInstancer"] refid="class_rml_1_1_context_1ab68305c8a6e502ff4db0bfc99c4abf1a" -->

Sets the instancer to use for releasing this object. 
#### Parameters
* `instancer` The context's instancer.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::CreateDataModel"] refid="class_rml_1_1_context_1aaced3d1a5358f6e9ad90e37e6a91b1ad" -->

Creates a data model. The returned constructor can be used to bind data variables. Elements can bind to the model using the attribute 'data-model="name"'. 
#### Parameters
* `name` The name of the data model. 

* `data_type_register` The data type register to use for the data model, or null to use the default register. 

* `allow_missing_variables` If true, allows variables to be bound after document load. Views referencing not-yet-bound variables will silently produce default values until the variable is bound and dirtied. 

#### Returns
A constructor for the data model, or empty if it could not be created.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetDataModel"] refid="class_rml_1_1_context_1aff26ab0a1c49aff815817895d1bcc5dc" -->

Retrieves the constructor for an existing data model. The returned constructor can be used to add additional bindings to an existing model. 
#### Parameters
* `name` The name of the data model. 

#### Returns
A constructor for the data model, or empty if it could not be found.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetDataModels"] refid="class_rml_1_1_context_1a6a07a86692cdd132f8e411a5f73acfaa" -->

Retrieves all data models in this context. 
#### Returns
A map of all data models in this context, keyed by their name.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::RemoveDataModel"] refid="class_rml_1_1_context_1a9c3bb9393cc7aeb65c6989ece77da412" -->

Removes the given data model. This also removes all data views, controllers, and bindings contained by the data model. 
:::warning
Invalidates all handles and constructors pointing to the data model. 

:::

#### Parameters
* `name` The name of the data model. 

#### Returns
True if successfully removed, false if no data model was found.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::SetDocumentsBaseTag"] refid="class_rml_1_1_context_1a4cc644db77df331bb66b3e42bf25c3ff" -->

Sets the base tag name of documents before creation. Default: "body". 
#### Parameters
* `tag` The name of the base tag. Example: "html"

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetDocumentsBaseTag"] refid="class_rml_1_1_context_1a9ca79833968fc47cb070eb802aadb2e9" -->

Gets the base tag name of documents. 
#### Returns
The current base tag name of documents.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::RequestNextUpdate"] refid="class_rml_1_1_context_1ac9d591a2d79ffdb95ae85db8a9ac4bf2" -->

Updates the time until [Update()](#update) should get called again. This can be used by elements and the application to implement on-demand rendering and thus drastically save CPU/GPU and reduce power consumption during inactivity. The context stores the lowest requested timestamp, which can later be retrieved using [GetNextUpdateDelay()](#getnextupdatedelay). 
#### Parameters
* `delay` Maximum time until next update.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Context::GetNextUpdateDelay"] refid="class_rml_1_1_context_1a2b36d77000929802bc5600a65d447125" -->

Get the max delay until [Update()](#update) and [Render()](#render) should get called again. An application can choose to only call update and render once the time has elapsed, but there's no harm in doing so more often. The returned value can be infinity, in which case [Update()](#update) should be invoked after user input was received. A value of 0 means "render
as fast as possible", for example if an animation is playing. 
#### Returns
Time until the next update is expected.

<!-- /rsmlui:block -->

