<!-- rsmlui:block["Rml::Element"] refid="class_rml_1_1_element" -->

A generic element in the DOM tree.

C++ include:

```cpp
#include <RmlUi/Core/Element.h>
```

C++ inheritance: `Rml::ScriptInterface`, `Rml::EnableObserverPtr< Element >`
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetEventDispatcher"] refid="class_rml_1_1_element_1adc9e8cca7e343bd1b5d50028eb62a7ff" -->
Access the event dispatcher for this element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetEventDispatcherSummary"] refid="class_rml_1_1_element_1a27f1a685f85088576157f6027928abcd" -->
Returns event types with the number of listeners for debugging.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetElementBackgroundBorder"] refid="class_rml_1_1_element_1ad509edfd72acff3d597aa00f43d5a0fb" -->
Access the element background and border.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetElementScroll"] refid="class_rml_1_1_element_1aa1b0ba7dd916cb4a985cce5018ec38a8" -->
Returns the element's scrollbar functionality.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetClosestScrollableContainer"] refid="class_rml_1_1_element_1a7fcc589b2e31a112678f0d1254237c0b" -->
Returns the element's nearest scroll container for any non-zero axis in the scroll delta, if any. Defaults to both axes.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetTransformState"] refid="class_rml_1_1_element_1ab1aae6ff4b866e696f633067de5fe053" -->
Returns the element's transform state.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetDataModel"] refid="class_rml_1_1_element_1af526a0f839d103db3ea16e179c41af7d" -->
Returns the data model of this element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetInstancer"] refid="class_rml_1_1_element_1a790d7b3e470d7866857662425492c4bf" -->

Sets the instancer to use for releasing this element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `instancer` | `ElementInstancer *` | Instancer to set on this element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::ProcessDefaultAction"] refid="class_rml_1_1_element_1a4f0bcf5d99d5e4ab30c0a052d71a818a" -->

Called when an emitted event propagates to this element, for event types with default actions. Note: See 'EventSpecification' for the events that call this function and during which phase.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `event` | `Event &` | The event to process. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetComputedValues"] refid="class_rml_1_1_element_1a1364ed2d87dd8f460309867f40698ddd" -->
Return the computed values of the element's properties. These values are updated as appropriate on every [Context::Update](api_Rml-Context.md#update).

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::UpdateProperties"] refid="class_rml_1_1_element_1ad6cd5e3d3c40efa13d9d55e410bc1b5d" -->
Updates definition, computed values, and runs OnPropertyChange on this element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::ForceLocalStackingContext"] refid="class_rml_1_1_element_1a9b38dd9e3627028a288a8577e2f70228" -->
Forces the element to generate a local stacking context, regardless of the value of its z-index property.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnUpdate"] refid="class_rml_1_1_element_1a6fa107115e455534deb199677b6132a7" -->
Called during the update loop after children are updated.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnRender"] refid="class_rml_1_1_element_1a133acbdd9cd0d71656238cf92bf637bf" -->
Called during render after backgrounds, borders, decorators, but before children, are rendered.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnResize"] refid="class_rml_1_1_element_1a2c4dd745ce20891436ec8d86b2d7208f" -->
Called during update if the element size has been changed.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnLayout"] refid="class_rml_1_1_element_1aab188f29b61181d646f031aaa0085d42" -->
Called during a layout operation, when the element is being positioned and sized.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnDpRatioChange"] refid="class_rml_1_1_element_1a4c69d4529b41260794a22644f832a879" -->
Called when the 'dp'-ratio has been changed.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnStyleSheetChange"] refid="class_rml_1_1_element_1a09ea4bd802796e3769f6816775ed9ba2" -->
Called when the current document's compiled style sheet has been changed. This may result in changed sprites.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnAttributeChange"] refid="class_rml_1_1_element_1ab37ddcff96e6d570f5a36505da7fd656" -->

Called when attributes on the element are changed.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `changed_attributes` | `const ElementAttributes &` | Dictionary of attributes changed on the element. Attribute value will be empty if it was unset. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnPropertyChange"] refid="class_rml_1_1_element_1acd072b7136423d6fac484fb62d39fe11" -->

Called when properties on the element are changed.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `changed_properties` | `const PropertyIdSet &` | The properties changed on the element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnPseudoClassChange"] refid="class_rml_1_1_element_1aff1a3de89f8f8faf2735a0c530f886e0" -->

Called when a pseudo class on the element is changed.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `pseudo_class` | `const String &` | The pseudo class changed on the element. |
| `activate` | `bool` | True if the pseudo class was activated. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnChildAdd"] refid="class_rml_1_1_element_1a2c4f9c6a305a49fe313a545bc6522201" -->

Called when a child node has been added up to two levels below us in the hierarchy.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `child` | `[Element](#element) *` | The element that has been added. This may be this element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OnChildRemove"] refid="class_rml_1_1_element_1a16b0b1530656eeb46369a3d97c600296" -->

Called when a child node has been removed up to two levels below us in the hierarchy.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `child` | `[Element](#element) *` | The element that has been removed. This may be this element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::DirtyLayout"] refid="class_rml_1_1_element_1a05ddbbef5ba2415154428575012b33cf" -->
Forces a re-layout of this element, and any other elements required.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::IsLayoutDirty"] refid="class_rml_1_1_element_1a8fd3d54c6bb04c5a2369c0546c11a71a" -->
Returns true if the element has been marked as needing a re-layout.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetRML"] refid="class_rml_1_1_element_1a71c8de1740ac853c773a742765e74abb" -->

Returns the RML of this element and all children.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `content` | `String &` | The content of this element and those under it, in XML form. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::OverridePseudoClass"] refid="class_rml_1_1_element_1a5c6f8d0fc60b3c01ef661059613f75ab" -->

Sets or removes an overriding pseudo-class on the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `target_element` | `[Element](#element) *` | The element to set or remove the pseudo class on. |
| `pseudo_class` | `const String &` | The pseudo class to activate or deactivate. |
| `activate` | `bool` | True if the pseudo-class is to be activated, false to be deactivated. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::StartAnimation"] refid="class_rml_1_1_element_1ac795e516e6c0133d38f2368a5419d07a" -->
Start an animation, replacing any existing animations of the same property name. If start_value is null, the element's current value is used.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::AddAnimationKeyTime"] refid="class_rml_1_1_element_1ace313ca641c28f574d56bb9e9a1a4f2b" -->
Add a key to an animation, extending its duration. If target_value is null, the element's current value is used.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::StartTransition"] refid="class_rml_1_1_element_1a124744a45cde838fa0048ca526b445a8" -->

Start a transition of the given property on this element. If an animation exists for the property, the call will be ignored. If a transition exists for this property, it will be replaced. 
#### Returns
True if the transition was added or replaced.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::HandleTransitionProperty"] refid="class_rml_1_1_element_1afb3052686faa05c8672aff6b2b9d2631" -->
Removes all transitions that are no longer part of the element's 'transition' property.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::HandleAnimationProperty"] refid="class_rml_1_1_element_1a7d3f5044b9d7c4b9db8c84b25895dfe5" -->
Starts new animations and removes animations no longer part of the element's 'animation' property.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::AdvanceAnimations"] refid="class_rml_1_1_element_1a4a51bd2a2546cf3169bf36ce7689e3d7" -->
Advances the animations (including transitions) forward in time.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetClass"] refid="class_rml_1_1_element_1ac77024b5e3b71545026c1890f5c4cb84" -->

Sets or removes a class on the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `class_name` | `const String &` | The name of the class to add or remove from the class list. |
| `activate` | `bool` | True if the class is to be added, false to be removed. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::IsClassSet"] refid="class_rml_1_1_element_1a56799babd9717b4f2754bd94c152140e" -->

Checks if a class is set on the element. 
#### Returns
True if the class is set on the element, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `class_name` | `const String &` | The name of the class to check for. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetClassNames"] refid="class_rml_1_1_element_1ad78b6795ab26e88a9e0d341ebaa7eaed" -->

Specifies the entire list of classes for this element. This will replace any others specified.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `class_names` | `const String &` | The list of class names to set on the style, separated by spaces. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetClassNames"] refid="class_rml_1_1_element_1abb9065b9c1f40a7210f34d6bc7698392" -->

Return the active class list. 
#### Returns
The space-separated list of classes active on the element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetStyleSheet"] refid="class_rml_1_1_element_1a08ec080ce13b19723f33bccf82043ce2" -->

Returns the active style sheet for this element. This may be nullptr. 
#### Returns
The element's style sheet.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetAddress"] refid="class_rml_1_1_element_1ab5cf686d5ddb3b023cecb02768f3b326" -->

Fills a string with the full address of this element. 
#### Returns
The address of the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `include_pseudo_classes` | `bool` | True to include the pseudo-classes of the leaf element. |
| `include_parents` | `bool` | True to include the address of all parent elements. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetOffset"] refid="class_rml_1_1_element_1ad75a7baa9307ab985754dbfdb9b0ce0b" -->

Sets the position of this element, as a two-dimensional offset from another element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `offset` | `Vector2f` | The offset (in pixels) of our primary box's top-left border corner from our offset parent's top-left border corner. |
| `offset_parent` | `[Element](#element) *` | The element this element is being positioned relative to. |
| `offset_fixed` | `bool` | True if the element is fixed in place (and will not scroll), false if not. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetRelativeOffset"] refid="class_rml_1_1_element_1a8b0b14f9ca94fad7753bae831d8ce315" -->

Returns the position of the top-left corner of one of the areas of this element's primary box, relative to its offset parent's top-left border corner. 
#### Returns
The relative offset.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `area` | `BoxArea` | The desired area position. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetAbsoluteOffset"] refid="class_rml_1_1_element_1aea7392c5da2a5a30ea232ad4411ac586" -->

Returns the position of the top-left corner of one of the areas of this element's primary box, relative to the element root. 
#### Returns
The absolute offset.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `area` | `BoxArea` | The desired area position. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetClipArea"] refid="class_rml_1_1_element_1a8cf531a6146fe13892d06ae569748f8f" -->

Sets an alternate area to use as the clip area.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `clip_area` | `BoxArea` | The box area to use as the element's clip area. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetClipArea"] refid="class_rml_1_1_element_1aa17a9ba43944f1b485e21a1778e65f79" -->

Returns the area the element uses as its clip area. 
#### Returns
The box area used as the element's clip area.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetScrollableOverflowRectangle"] refid="class_rml_1_1_element_1aa52a104574ba5f328a21173c0f8afb63" -->

Sets the dimensions of the element's scrollable overflow rectangle. This is the tightest fitting box surrounding all of this element's logical children, and the element's padding box.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `scrollable_overflow_rectangle` | `Vector2f` | The dimensions of the box's scrollable content. |
| `clamp_scroll_offset` | `bool` | Clamp scroll offset to the new rectangle, should only be set when the final overflow size has been determined. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetBox"] refid="class_rml_1_1_element_1a6104e53df1c34148ae210b3db5333758" -->

Sets the box describing the size of the element, and removes all others.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `box` | `const Box &` | The new dimensions box for the element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::AddBox"] refid="class_rml_1_1_element_1aa676eb5891a8a5f2fe003ccfe3e67bf8" -->

Adds a box to the end of the list describing this element's geometry.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `box` | `const Box &` | The auxiliary box for the element. |
| `offset` | `Vector2f` | The offset of the box relative to the top left border corner of the element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetBox"] refid="class_rml_1_1_element_1a93ea28541151c1a94986821df58bb413" -->

Returns the main box describing the size of the element. 
#### Returns
The box.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetBox"] refid="class_rml_1_1_element_1a6ed2e33bfe8aba8e5c4c0a559d8e6e89" -->

Returns one of the boxes describing the size of the element. 
#### Returns
The requested box.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `index` | `int` | The index of the desired box, with 0 being the main box. If outside of bounds, the main box will be returned. |
| `offset` | `Vector2f &` | The offset of the box relative to the element's border box. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetRenderBox"] refid="class_rml_1_1_element_1ab6cc47095b13e6722bba97c348e9d0f3" -->

Returns one of the render boxes describing how to generate the geometry for the corresponding element's box. 
#### Returns
The requested render box.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `fill_area` | `BoxArea` | The box area that acts as the background, or fill, of the render box. |
| `index` | `int` | The index of the desired box, with 0 being the main box. If outside of bounds, the main render box will be returned. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetNumBoxes"] refid="class_rml_1_1_element_1a4105b4c0d057044fa35474525f9d314d" -->

Returns the number of boxes making up this element's geometry. 
#### Returns
the number of boxes making up this element's geometry.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetBaseline"] refid="class_rml_1_1_element_1aa3f093e2e9a8b8518e4a050ae3309b66" -->

Returns the baseline of the element, in pixel offset from the element's bottom margin edge (positive up). 
#### Returns
The element's baseline. The default element will return 0.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetIntrinsicDimensions"] refid="class_rml_1_1_element_1a77e39ac210a15c6dec896f7186fa8fd4" -->

Gets the intrinsic dimensions of this element, if it is a replaced element with an inherent size. This size will only be overridden by a styled width or height. 
#### Returns
True if the element is a replaced element with intrinsic dimensions, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `dimensions` | `Vector2f &` | The dimensions to size, if appropriate. |
| `ratio` | `float &` | The intrinsic ratio (width/height), if appropriate. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::IsReplaced"] refid="class_rml_1_1_element_1af598628b3ee18b8895bf17f85eb35abc" -->

Returns true if the element is replaced, thereby handling its own rendering. 
#### Returns
True if the element is a replaced element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::IsPointWithinElement"] refid="class_rml_1_1_element_1af8948b767f61cc2ab6805f01f9d79d97" -->

Checks if a given point in screen coordinates lies within the bordered area of this element. 
#### Returns
True if the element is within this element, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `point` | `Vector2f` | The point to test. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::IsVisible"] refid="class_rml_1_1_element_1a943e48dc78ee4bf9bfe40cc18ff02b1f" -->

Returns the visibility of the element. 
#### Returns
True if the element is visible, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `include_ancestors` | `bool` | Check parent elements for visibility |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetZIndex"] refid="class_rml_1_1_element_1acccf98e6642f986ba03a65a383294504" -->

Returns the z-index of the element. 
#### Returns
The element's z-index.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetFontFaceHandle"] refid="class_rml_1_1_element_1a99660a76851404f13ea5bf3a57892e39" -->

Returns the element's font face handle. 
#### Returns
The element's font face handle.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetProperty"] refid="class_rml_1_1_element_1aa79905d425be756a39ea93925ceb7834" -->

Sets a local property override on the element. 
#### Returns
True if the property parsed successfully, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | The name of the new property. |
| `value` | `const String &` | The new property to set. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetProperty"] refid="class_rml_1_1_element_1a56a5f9477bc430680d007a777bd10e63" -->

Sets a local property override on the element to a pre-parsed value. 
#### Returns
True if the property was set successfully, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `id` | `PropertyId` | The id of the new property. |
| `property` | `const Property &` | The parsed property to set. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::RemoveProperty"] refid="class_rml_1_1_element_1a84a418b4187344e461cb5ed0bab8950d" -->

Removes a local property override on the element; its value will revert to that defined in the style sheet.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | The name of the local property definition to remove. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetProperty"] refid="class_rml_1_1_element_1ae291f6f9507884dccb785346d84c9005" -->

Returns one of this element's properties. If the property is not defined for this element and not inherited from an ancestor, the default value will be returned. Cascading variables ("var()") are fully resolved. 
#### Returns
The value of this property for this element, or nullptr if no property exists with the given name. 

<div class="warning">

When resolving any cascading variables ("var()") in the property, correctness is only guaranteed after a context update. @lifetime The returned pointer is invalidated on any following call into RmlUi, please copy by value to store the property around. 

</div>


Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | The name of the property to fetch the value for. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetProperty"] refid="class_rml_1_1_element_1aeb2e363b71c249fe4c8bf73f8c654c81" -->

Returns the values of one of this element's properties. 
#### Returns
The value of this property.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | The name of the property to get. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetLocalProperty"] refid="class_rml_1_1_element_1a34c13f59c4d10609e6b19de1037a902e" -->

Returns one of this element's local style properties. Cascading variables ("var()") are not resolved. 
#### Returns
The value of this property for this element, or nullptr if this property has not been explicitly defined for this element. @lifetime The returned pointer is invalidated on any following call into RmlUi, please copy by value to store the property around.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | The name of the property to fetch the value for. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetLocalStyleProperties"] refid="class_rml_1_1_element_1a3d032b42792b4f31289f3945287f2ab9" -->

Returns the local style properties. Does not include properties applied from any locally set classes. 
#### Returns
The local properties for this element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::ResolveLength"] refid="class_rml_1_1_element_1a677f75faa8d7489458d708654e36e73b" -->

Resolves a length to its canonical unit ('px'). 
#### Returns
The resolved value in their canonical unit, or zero if it could not be resolved. 

<div class="warning">

Font-relative and context-relative units will be resolved against this element's computed values and its context. 

</div>


Parameters:

| Name | Type | Description |
|------|------|-------------|
| `value` | `NumericValue` | The numeric value. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::ResolveNumericValue"] refid="class_rml_1_1_element_1a2957b51024d33f891de98db725512837" -->

Resolves a numeric value with units of number, percentage, length, or angle to their canonical unit (unit-less, 'px', or 'rad'). Numbers and percentages are scaled by the base value and returned. 
#### Returns
The resolved value in their canonical unit, or zero if it could not be resolved.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `value` | `NumericValue` | The value to be resolved. |
| `base_value` | `float` | The value that is scaled by the number or percentage value, if applicable. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetContainingBlock"] refid="class_rml_1_1_element_1a9a021990dd2f36a9af4ac4dbfddd8dd7" -->
Returns the size of the containing block. Often percentages are scaled relative to this.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetPosition"] refid="class_rml_1_1_element_1a14fc12d5846e2395fc04ea12f9ca7161" -->
Returns 'position' property value from the element's computed values.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetFloat"] refid="class_rml_1_1_element_1aeffba447dd7c00192dd6d32292956dc8" -->
Returns 'float' property value from the element's computed values.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetDisplay"] refid="class_rml_1_1_element_1a626f105037a3be578a1afe67e74e796d" -->
Returns 'display' property value from the element's computed values.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetLineHeight"] refid="class_rml_1_1_element_1a28009c57259ac0be677d8b87752290f0" -->
Returns 'line-height' property value from the element's computed values.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::Project"] refid="class_rml_1_1_element_1a609d46415195c0602a123b051fc386f1" -->

Project a 2D point in pixel coordinates onto the element's plane. 
#### Returns
True on success, false if transformation matrix is singular.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::Animate"] refid="class_rml_1_1_element_1ab580f93ef4007af78911d17e3cbfd1db" -->

Start an animation of the given property on this element. If an animation of the same property name exists, it will be replaced. If start_value is null, the current property value on this element is used. 
#### Returns
True if a new animation was added.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::AddAnimationKey"] refid="class_rml_1_1_element_1a3472a3d03e7c93b4a8dba296aba3c31e" -->

Add a key to an animation, extending its duration. If no animation exists for the given property name, the call will be ignored. 
#### Returns
True if a new animation key was added.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::IterateLocalProperties"] refid="class_rml_1_1_element_1a47d217de82a6a6f1529d4d0eb7c9974e" -->

Iterator for the local (non-inherited) properties defined on this element. 
<div class="warning">

Modifying the element's properties or classes invalidates the iterator. 

</div>


#### Returns
Iterator to the first property defined on this element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `filter_inherited_by` | `[Element](#element) *` | If set, only iterates properties that are inherited by the given element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetPseudoClass"] refid="class_rml_1_1_element_1aa6e986c21d649dab25842fa75d305c94" -->

Sets or removes a pseudo-class on the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `pseudo_class` | `const String &` | The pseudo class to activate or deactivate. |
| `activate` | `bool` | True if the pseudo-class is to be activated, false to be deactivated. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::IsPseudoClassSet"] refid="class_rml_1_1_element_1ae7c85486ea3946c1fe421c0ab99ef4a8" -->

Checks if a specific pseudo-class has been set on the element. 
#### Returns
True if the pseudo-class is set on the element, false if not.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `pseudo_class` | `const String &` | The name of the pseudo-class to check for. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::ArePseudoClassesSet"] refid="class_rml_1_1_element_1a240a798360ed9fa53925f48d66ab5abf" -->

Checks if a complete set of pseudo-classes are set on the element. 
#### Returns
True if all the pseudo-classes are set, false if not.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `pseudo_classes` | `const StringList &` | The list of pseudo-classes to check for. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetActivePseudoClasses"] refid="class_rml_1_1_element_1a56a6195955cb3135da1edfd76a8c9622" -->

Gets a list of the current active pseudo-classes. 
#### Returns
The list of active pseudo-classes.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetAttribute"] refid="class_rml_1_1_element_1a855f8e2d610fa7d3834210c18fbb837a" -->

Sets an attribute on the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | Name of the attribute. |
| `value` | `const T &` | Value of the attribute. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetAttribute"] refid="class_rml_1_1_element_1a38c8390fcc05c1ef84754b25953e106b" -->

Gets the specified attribute. 
#### Returns
A variant representing the attribute, or nullptr if the attribute doesn't exist.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | Name of the attribute to retrieve. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetAttribute"] refid="class_rml_1_1_element_1a7b9a1804890c473f60d59b307a0911b6" -->
Gets the specified attribute.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetAttribute"] refid="class_rml_1_1_element_1ac90903cc3440399eda5e31487ae7aeb3" -->

Gets the specified attribute, or the default value.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | Name of the attribute to retrieve. |
| `default_value` | `const T &` | Value to return if the attribute doesn't exist. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::HasAttribute"] refid="class_rml_1_1_element_1afe0b5b4f7cb3927925186017aed9fc8d" -->

Checks if the element has a certain attribute. 
#### Returns
True if the element has the given attribute, false if not.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | The name of the attribute to check for. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::RemoveAttribute"] refid="class_rml_1_1_element_1ac518a2f355e5473a49c3111440d91940" -->

Removes the attribute from the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `name` | `const String &` | Name of the attribute. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetAttributes"] refid="class_rml_1_1_element_1a6a62d826cf827f1164582eb1d408224f" -->

Set a group of attributes.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `attributes` | `const ElementAttributes &` | Attributes to set. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetAttributes"] refid="class_rml_1_1_element_1a0ba6817b23159e265629f103e16cfe11" -->

Get the attributes of the element. 
#### Returns
The attributes

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetNumAttributes"] refid="class_rml_1_1_element_1a99b8ceb76f4de5e6dd1913649e6c1f39" -->

Returns the number of attributes on the element. 
#### Returns
The number of attributes on the element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetFocusLeafNode"] refid="class_rml_1_1_element_1aa54bc84cefbbe308bd6957265e27af24" -->

Gets the outermost focus element down the tree from this node. 
#### Returns
Outermost focus element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetContext"] refid="class_rml_1_1_element_1a76d70079c510c459e6ef5daad99f72f7" -->

Returns the element's context. 
#### Returns
The context this element's document exists within.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetRenderManager"] refid="class_rml_1_1_element_1a0aca52185b33cc9e7370e13437f21231" -->

Returns the element's render manager. 
#### Returns
The render manager responsible for this element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetTagName"] refid="class_rml_1_1_element_1ac8f4300a9572cb49d550c4f1fe6563e7" -->

Gets the name of the element. 
#### Returns
The name of the element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetId"] refid="class_rml_1_1_element_1ab2face98c09ed42e444f59bcd52ef4a4" -->

Gets the id of the element. 
#### Returns
The element's id.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetId"] refid="class_rml_1_1_element_1a519ccc40f3b67490b6e6063fb88dc4eb" -->

Sets the id of the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `id` | `const String &` | The new id of the element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetAbsoluteLeft"] refid="class_rml_1_1_element_1ad2e45307bb7c8f68bc54cb4c1929352d" -->

Gets the horizontal offset from the context's left edge to the element's left border edge. 
#### Returns
The horizontal offset of the element within its context, in pixels.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetAbsoluteTop"] refid="class_rml_1_1_element_1a0947f1335c449bd4099256c6256dba94" -->

Gets the vertical offset from the context's top edge to the element's top border edge. 
#### Returns
The vertical offset of the element within its context, in pixels.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetClientLeft"] refid="class_rml_1_1_element_1a973027f138bc39ca1b5cfe5e9caae1ab" -->

Gets the horizontal offset from the element's left border edge to the left edge of its client area. This is effectively equivalent to the left border width. 
#### Returns
The horizontal offset of the element's client area, in pixels.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetClientTop"] refid="class_rml_1_1_element_1ab5c478518e67ba5850e687007560b9a7" -->

Gets the vertical offset from the element's top border edge to the top edge of its client area. This is effectively equivalent to the top border width. 
#### Returns
The vertical offset of the element's client area, in pixels.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetClientWidth"] refid="class_rml_1_1_element_1ae0280369a040da874406631c8c1eb931" -->

Gets the width of the element's client area. This is the padding area subtracted by the vertical scrollbar width if present. 
#### Returns
The element's client width, in pixels.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetClientHeight"] refid="class_rml_1_1_element_1a7113bb1054d906a5ae424bffca2ac160" -->

Gets the height of the element's client area. This is the padding area subtracted by the horizontal scrollbar height if present. 
#### Returns
The element's client height, in pixels.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetOffsetParent"] refid="class_rml_1_1_element_1a87c71a4ef612ab813cd813c62729d07d" -->

Returns the element from which all offset calculations are currently computed. 
#### Returns
This element's offset parent.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetOffsetLeft"] refid="class_rml_1_1_element_1ac28d029ace9ab99f36e059d3e1dc0b15" -->

Gets the distance from this element's left border to its offset parent's left border. 
#### Returns
The horizontal distance (in pixels) from this element's offset parent to itself.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetOffsetTop"] refid="class_rml_1_1_element_1ab8235c84bfeffca3dea7c69bb551ea36" -->

Gets the distance from this element's top border to its offset parent's top border. 
#### Returns
The vertical distance (in pixels) from this element's offset parent to itself.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetOffsetWidth"] refid="class_rml_1_1_element_1ad03e7d8d4bc6c2cfb7e3e74cc4851e5e" -->

Gets the width of the element, including the content area, padding, borders and scrollbars, but not margins. 
#### Returns
The width of the rendered element, in pixels.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetOffsetHeight"] refid="class_rml_1_1_element_1a1eefd484f93947dde50f9e3f95ec397f" -->

Gets the height of the element, including the content area, padding, borders and scrollbars, but not margins. 
#### Returns
The height of the rendered element, in pixels.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetScrollLeft"] refid="class_rml_1_1_element_1a255294fc07c9016b89c671d3ea6a8dc8" -->

Gets the left scroll offset of the element. 
#### Returns
The element's left scroll offset.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetScrollLeft"] refid="class_rml_1_1_element_1a177c75275d385f4bf687d59d98ab2578" -->

Sets the left scroll offset of the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `scroll_left` | `float` | The element's new left scroll offset. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetScrollTop"] refid="class_rml_1_1_element_1a53fd5799570ac3692286655456f02d3a" -->

Gets the top scroll offset of the element. 
#### Returns
The element's top scroll offset.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetScrollTop"] refid="class_rml_1_1_element_1a961b34cdba5f79a2b2d4a6c3ce3715b3" -->

Sets the top scroll offset of the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `scroll_top` | `float` | The element's new top scroll offset. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetScrollWidth"] refid="class_rml_1_1_element_1ad8eeb78008d3cead3939cfb8a37d4fad" -->

Gets the width of the scrollable content of the element; it includes the element padding but not its margin. 
#### Returns
The width (in pixels) of the scrollable content of the element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetScrollHeight"] refid="class_rml_1_1_element_1a994ed5b60af0a902fafacfafb1b32835" -->

Gets the height of the scrollable content of the element; it includes the element padding but not its margin. 
#### Returns
The height (in pixels) of the scrollable content of the element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetStyle"] refid="class_rml_1_1_element_1a6baed79f8776d7a30cdd38d0ee847d4a" -->

Gets the object representing the declarations of an element's style attributes. 
#### Returns
The element's style.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetOwnerDocument"] refid="class_rml_1_1_element_1a1aa0433be52e6ee9ae5b9281da8ba682" -->

Gets the document this element belongs to. 
#### Returns
This element's document.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetParentNode"] refid="class_rml_1_1_element_1a005070ddd44fd3b485ae627ccac9663a" -->

Gets this element's parent node. 
#### Returns
This element's parent.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::Closest"] refid="class_rml_1_1_element_1abb686763575ec3ce82f5b176280dfa61" -->

Recursively search for the first ancestor of this node matching the given selector. 
#### Returns
The ancestor if found, or nullptr if no ancestor could be matched. @performance Prefer GetElementById/TagName/ClassName whenever possible.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `selectors` | `const String &` | The selector or comma-separated selectors to match against. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetNextSibling"] refid="class_rml_1_1_element_1a155f17c5221b8b5ba07e07193e70eaa1" -->

Gets the element immediately following this one in the tree. 
#### Returns
This element's next sibling element, or nullptr if there is no sibling element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetPreviousSibling"] refid="class_rml_1_1_element_1a6da7014825513c249f5272af3a4eaaf9" -->

Gets the element immediately preceding this one in the tree. 
#### Returns
This element's previous sibling element, or nullptr if there is no sibling element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetFirstChild"] refid="class_rml_1_1_element_1a29fa54b46e2e93fce570b855d89c16b2" -->

Returns the first child of this element. 
#### Returns
This element's first child, or nullptr if it contains no children.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetLastChild"] refid="class_rml_1_1_element_1a2e7ec361ed6bca150b50ab5fb95dcb75" -->

Gets the last child of this element. 
#### Returns
This element's last child, or nullptr if it contains no children.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetChild"] refid="class_rml_1_1_element_1a95622230ae259eaaaa273d5227474598" -->

Get the child element at the given index. 
#### Returns
The child element at the given index.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `index` | `int` | Index of child to get. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetNumChildren"] refid="class_rml_1_1_element_1a9f475f98b21cabdbd474aab76d964999" -->

Get the current number of children in this element 
#### Returns
The number of children.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `include_non_dom_elements` | `bool` | True if the caller wants to include the non-DOM children. Only set this to true if you know what you're doing! |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetInnerRML"] refid="class_rml_1_1_element_1aa02bd9b3f35cc3ef6c7a97b7856ee783" -->

Gets the markup and content of the element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `content` | `String &` | The content of the element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetInnerRML"] refid="class_rml_1_1_element_1a212949131dcd71aff0e7a23f7d855bde" -->

Gets the markup and content of the element. 
#### Returns
The content of the element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::SetInnerRML"] refid="class_rml_1_1_element_1a7b13ccdfefb3c7d295234aac2cd4e625" -->

Sets the markup and content of the element. All existing children will be replaced.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rml` | `const String &` | The new content of the element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::Focus"] refid="class_rml_1_1_element_1a14e266bac08e0562bb79f3a94527af90" -->

Gives focus to the current element. 
#### Returns
True if the change focus request was successful

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `focus_visible` | `bool` | True to indicate that the focus should be visually indicated by setting the ':focus-visible' pseudo class. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::Blur"] refid="class_rml_1_1_element_1ac5338f72f5d31b5f6a2850b2a72389c4" -->
Removes focus from this element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::Click"] refid="class_rml_1_1_element_1ad16cdef9e77efbe725a849480a5b6ffd" -->
Fakes a mouse click on this element.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::AddEventListener"] refid="class_rml_1_1_element_1ad7f87dbd616f189f7f8d3efdcb2161ac" -->

Adds an event listener to this element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `event` | `const String &` | Event to attach to. |
| `listener` | `EventListener *` | The listener object to be attached. |
| `in_capture_phase` | `bool` | True to attach in the capture phase, false in the bubble phase. @lifetime The added listener must stay alive until after the dispatched call from EventListener::OnDetach(). This occurs e.g. when the element is destroyed or when [RemoveEventListener()](#removeeventlistener-1) is called with the same parameters passed here. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::AddEventListener"] refid="class_rml_1_1_element_1a7ca7743064618728a3eac25dbdb9a173" -->

Adds an event listener to this element by id. @lifetime The added listener must stay alive until after the dispatched call from EventListener::OnDetach(). This occurs e.g. when the element is destroyed or when [RemoveEventListener()](#removeeventlistener-1) is called with the same parameters passed here.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::RemoveEventListener"] refid="class_rml_1_1_element_1a61cf71e5cca808a202383309aa2d5f03" -->

Removes an event listener from this element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `event` | `const String &` | Event to detach from. |
| `listener` | `EventListener *` | The listener object to be detached. |
| `in_capture_phase` | `bool` | True to detach from the capture phase, false from the bubble phase. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::RemoveEventListener"] refid="class_rml_1_1_element_1ae5e7898738d44471b00108f4e84404ba" -->
Removes an event listener from this element by id.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::DispatchEvent"] refid="class_rml_1_1_element_1acd8fa336e2c9acf062572be1f119f30e" -->

Sends an event to this element. 
#### Returns
True if the event was not consumed (ie, was prevented from propagating by an element), false if it was.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `type` | `const String &` | Event type in string form. |
| `parameters` | `const Dictionary &` | The event parameters. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::DispatchEvent"] refid="class_rml_1_1_element_1a8edbca966574f1ebd105d26f5d1ac12e" -->
Sends an event to this element, overriding the default behavior for the given event type.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::DispatchEvent"] refid="class_rml_1_1_element_1a06a7b1b9fe52dbf077f58368c0ac5e5e" -->
Sends an event to this element by event id.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::ScrollIntoView"] refid="class_rml_1_1_element_1aef732069ed6f403c17e433f832d4573c" -->

Scrolls the parent element's contents so that this element is visible.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `options` | `ScrollIntoViewOptions` | Scroll parameters that control the desired element alignment relative to the parent. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::ScrollIntoView"] refid="class_rml_1_1_element_1aae19164aa82db481d25320bd16b31549" -->

Scrolls the parent element's contents so that this element is visible.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `align_with_top` | `bool` | If true, the element will align itself to the top of the parent element's window. If false, the element will be aligned to the bottom of the parent element's window. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::ScrollTo"] refid="class_rml_1_1_element_1ade7d452051f87b66580485e293a6ef4b" -->

Sets the scroll offset of this element to the given coordinates. 
<div class="warning">

Smooth scrolling can only be applied to a single element at a time, any active smooth scrolls will be canceled. 

</div>


Parameters:

| Name | Type | Description |
|------|------|-------------|
| `offset` | `Vector2f` | The scroll destination coordinates. |
| `behavior` | `ScrollBehavior` | Smooth scrolling behavior. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::AppendChild"] refid="class_rml_1_1_element_1a6b67714d0cc5c77dba94cc924b28ee30" -->

Append a child to this element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `element` | `ElementPtr` | The element to append as a child. |
| `dom_element` | `bool` | True if the element is to be part of the DOM, false otherwise. Only set this to false if you know what you're doing! |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::InsertBefore"] refid="class_rml_1_1_element_1a22f68e09c5605968105d6cfbf56842ea" -->

Adds a child to this element directly before the adjacent element. The new element inherits the DOM/non-DOM status from the adjacent element.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `element` | `ElementPtr` | [Element](#element) to be inserted. |
| `adjacent_element` | `[Element](#element) *` | The reference element which the new element will be inserted before. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::ReplaceChild"] refid="class_rml_1_1_element_1a06786873807c75be4767c6985d4946c0" -->

Replaces the second node with the first node. 
#### Returns
A unique pointer to the replaced element if found, discard the result to immediately destroy.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `inserted_element` | `ElementPtr` | The element that will be inserted and replace the other element. |
| `replaced_element` | `[Element](#element) *` | The existing element that will be replaced. If this doesn't exist, inserted_element will be appended. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::RemoveChild"] refid="class_rml_1_1_element_1a7cb1f1a45f281c9bb4932ae9af0a286d" -->

Remove a child element from this element. 
#### Returns
A unique pointer to the element if found, discard the result to immediately destroy.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `element` | `[Element](#element) *` | The element to remove. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::HasChildNodes"] refid="class_rml_1_1_element_1a5c0fbcd7313f5745a0d96bcbe5fd40ef" -->

Returns whether or not this element has any DOM children. 
#### Returns
True if the element has at least one DOM child, false otherwise.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetElementById"] refid="class_rml_1_1_element_1a42c97ec8d9fcf1da312a4c5ac987a60f" -->

Get a child element by its ID. 
#### Returns
The child of this element with the given ID, or nullptr if no such child exists.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `id` | `const String &` | The ID of the child element. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetElementsByTagName"] refid="class_rml_1_1_element_1ac3a6e25ac68ced581325013aa10c8b18" -->

Get all descendant elements with the given tag.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `elements` | `ElementList &` | Resulting elements. |
| `tag` | `const String &` | Tag to search for. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::GetElementsByClassName"] refid="class_rml_1_1_element_1ac3bae3fee56cdda3573a78ea0de96e68" -->

Get all descendant elements with the given class set on them.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `elements` | `ElementList &` | Resulting elements. |
| `class_name` | `const String &` | Class name to search for. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::QuerySelector"] refid="class_rml_1_1_element_1a2196253937d7a8084703218f88579f93" -->

Returns the first descendent element matching the RCSS selector query. 
#### Returns
The first matching element during a depth-first traversal. @performance Prefer GetElementById/TagName/ClassName whenever possible.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `selector` | `const String &` | The selector or comma-separated selectors to match against. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::QuerySelectorAll"] refid="class_rml_1_1_element_1a2fe2641cc295ee616d5a12234b1d9a08" -->

Returns all descendent elements matching the RCSS selector query.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `elements` | `ElementList &` | The list of matching elements. |
| `selector` | `const String &` | The selector or comma-separated selectors to match against. @performance Prefer GetElementById/TagName/ClassName whenever possible. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::Matches"] refid="class_rml_1_1_element_1a81b6f268fa7ba064ec6a80063c06fff0" -->

Checks if the element matches the given RCSS selector query. 
#### Returns
True if the element matches the given RCSS selector query, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `selector` | `const String &` | The selector or comma-separated selectors to match against. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::Contains"] refid="class_rml_1_1_element_1aab540b4ad5266db92fc5343dad2a2b5c" -->

Checks if the provided element is a descendant of the current element. 
#### Returns
True if the provided element is a descendant of this element, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `element` | `[Element](#element) *` | The element to test with. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::RMLUI_RTTI_DeclareWithParent"] refid="class_rml_1_1_element_1ae064a1d32988acea268050875222b637" -->

Constructs a new RmlUi element. This should not be called directly; use the Factory instead.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Element::Clone"] refid="class_rml_1_1_element_1acf47354cb6e1333345aad15d8160fefe" -->
Clones this element, returning a new, unparented element.

<!-- /rsmlui:block -->

