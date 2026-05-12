<!-- rsmlui:block["Rml::ElementDocument"] refid="class_rml_1_1_element_document" -->

Represents a document in the dom tree.

C++ include:

```cpp
#include <RmlUi/Core/ElementDocument.h>
```

C++ inheritance: `Rml::Element`
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::ProcessHeader"] refid="class_rml_1_1_element_document_1a502d75fc3665118ff66cbd094523c475" -->
Process given document header.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::GetContext"] refid="class_rml_1_1_element_document_1aa71c7d9d876c1732725d720feb64bc53" -->
Returns the document's context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::SetTitle"] refid="class_rml_1_1_element_document_1a37ca32a9d69ace2428adf333ee821895" -->
Sets the document's title.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::GetTitle"] refid="class_rml_1_1_element_document_1aff57b2a8a048245aa925d02db133ff94" -->
Returns the title of this document.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::GetSourceURL"] refid="class_rml_1_1_element_document_1a99470fcb6e8be4162fdbb06200cb7dc5" -->
Returns the source address of this document.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::GetStyleSheet"] refid="class_rml_1_1_element_document_1a3bc3682f4817af99f7a2122586c3db2e" -->

Returns the document's compiled style sheet. 
:::note
The style sheet may be regenerated when media query parameters change, invalidating the pointer. 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::ReloadStyleSheet"] refid="class_rml_1_1_element_document_1afdb80ff42c479c8dbf8a496b820a7716" -->

Reload the document's style sheet from source files. Styles will be reloaded from <style> tags and external style sheets, but not inline 'style' attributes. 
:::note
The source url originally used to load the document must still be a valid RML document. 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::GetStyleSheetContainer"] refid="class_rml_1_1_element_document_1a9f3f11bca5de169ac6b97596a846c7d1" -->
Returns the document's style sheet container.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::SetStyleSheetContainer"] refid="class_rml_1_1_element_document_1a2f823f22d0793a33f3d7c45dace0e084" -->
Sets the style sheet this document, and all of its children, uses.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::PullToFront"] refid="class_rml_1_1_element_document_1a3b4355914673fff865b3867d9ce0dc07" -->
Brings the document to the front of the document stack.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::PushToBack"] refid="class_rml_1_1_element_document_1ace46df4626f579e9860f7598408956f4" -->
Sends the document to the back of the document stack.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::Show"] refid="class_rml_1_1_element_document_1a25d3f4791ed562a75b02ed70c3450a9c" -->

Show the document. 
#### Parameters
* `modal_flag` Flag controlling the modal state of the document, see the 'ModalFlag' description for details. 

* `focus_flag` Flag controlling the focus, see the 'FocusFlag' description for details. 

* `scroll_flag` Flag controlling scrolling, see the 'ScrollFlag' description for details.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::Hide"] refid="class_rml_1_1_element_document_1a99beeefe68c80ee90e73378d540bf437" -->
Hide the document.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::Close"] refid="class_rml_1_1_element_document_1a52acc4a2b58d92d3e7b01227be39f37b" -->

Close the document. 
:::note
The destruction of the document is deferred until the next call to [Context::Update()](api_Rml-Context.md#update). 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::CreateElement"] refid="class_rml_1_1_element_document_1a8408ac3afe9cfda9382b56c128b7f9a3" -->

Creates the named element. 
#### Parameters
* `name` The tag name of the element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::CreateTextNode"] refid="class_rml_1_1_element_document_1a7072caf2565893c19ea5cfebd3047569" -->

Create a text element with the given text content. 
#### Parameters
* `text` The text content of the text element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::IsModal"] refid="class_rml_1_1_element_document_1ac15fb33f8839be9e5431a3015c06ca6e" -->

Does the document have modal display set. 
#### Returns
True if the document is hogging focus.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::FindNextTabElement"] refid="class_rml_1_1_element_document_1ab4aad82180487713239b6e1170f9b57a" -->

Finds the next tabbable element in the document tree, starting at the given element, possibly wrapping around the document. 
#### Parameters
* `current_element` The element to start from. 

* `forward` True to search forward, false to search backward. 

#### Returns
The next tabbable element, or nullptr if none could be found.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::LoadInlineScript"] refid="class_rml_1_1_element_document_1abd97e532cf659539929cc940f7344326" -->

Loads an inline script into the document. Note that the base implementation does nothing, but script plugins can hook into this method. 
#### Parameters
* `content` The script content. 

* `source_path` Path of the script the source comes from, useful for debug information. 

* `source_line` Line of the script the source comes from, useful for debug information.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::LoadExternalScript"] refid="class_rml_1_1_element_document_1aabe77504a64d821917533d5c592cebee" -->

Loads an external script into the document. Note that the base implementation does nothing, but script plugins can hook into this method. 
#### Parameters
* `source_path` The script file path.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::UpdateDocument"] refid="class_rml_1_1_element_document_1afc405c50331bbd9c67cee8ed434caf36" -->

Updates the document, including its layout. Users must call this manually before requesting information such as the size or position of an element if any element in the document was recently changed, unless [Context::Update](api_Rml-Context.md#update) has already been called after the change. This has a performance penalty, only call when necessary.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::OnPropertyChange"] refid="class_rml_1_1_element_document_1ab4efef48719f0dc3bcef3d1e46bfe218" -->
Repositions the document if necessary.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::ProcessDefaultAction"] refid="class_rml_1_1_element_document_1ac306de2e4519e5e697b650a8a70b3011" -->
Processes any events specially handled by the document.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::OnResize"] refid="class_rml_1_1_element_document_1acded12522de94ac0dfb563dd10602f33" -->
Called during update if the element size has been changed.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::IsFocusableFromModal"] refid="class_rml_1_1_element_document_1abda2c3562096194e181d5090d59fbe29" -->
Returns whether the document can receive focus during click when another document is modal.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::SetFocusableFromModal"] refid="class_rml_1_1_element_document_1ac8f02d5983613504babade95df4e77d2" -->
Sets whether the document can receive focus when another document is modal.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::SearchFocusSubtree"] refid="class_rml_1_1_element_document_1a26408f9bdcf09638575e28bfc8346342" -->
Searches forwards or backwards for a focusable element in the given subtree.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::FindNextNavigationElement"] refid="class_rml_1_1_element_document_1a3752cea8855fa436560de5f8b703ff49" -->
Find the next element to navigate to, starting at the current element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::DirtyLayout"] refid="class_rml_1_1_element_document_1a00309c1ad2441f336d3ad13c1b56d939" -->
Sets the dirty flag on the layout so the document will format its children before the next render.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::IsLayoutDirty"] refid="class_rml_1_1_element_document_1a5d7e066795e774d475b984e44755826c" -->
Returns true if the document has been marked as needing a re-layout.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::DirtyMediaQueries"] refid="class_rml_1_1_element_document_1aeb28ea38eae42abac22a38d8c9c6458d" -->
Notify the document that media query-related properties have changed and that style sheets need to be re-evaluated.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::DirtyVwAndVhProperties"] refid="class_rml_1_1_element_document_1a50129a0e013e3dee224962a4d2ca8f57" -->
Updates all sizes defined by the 'vw' and the 'vh' units.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::UpdateLayout"] refid="class_rml_1_1_element_document_1a2b7a56d88c4dca2bf5f6e1d8e55f7b53" -->
Updates the layout if necessary.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::UpdatePosition"] refid="class_rml_1_1_element_document_1a8089c7d529ac316d6f339b5e771d611a" -->
Updates the position of the document based on the style properties.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::ElementDocument::DirtyPosition"] refid="class_rml_1_1_element_document_1a9f8e0d73fe997676b2d299ef7c4af697" -->
Sets the dirty flag for document positioning.

<!-- /rsmlui:block -->

