<!-- rsmlui:block["Rml::Variant"] refid="class_rml_1_1_variant" -->

[Variant](#variant) is a container that can store a selection of basic types. The variant will store the value in the native form corresponding to the version of Set that was called.

Get is templated to convert from the stored form to the requested form by using a TypeConverter.

C++ include:

```cpp
#include <RmlUi/Core/Variant.h>
```
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Variant::Get"] refid="class_rml_1_1_variant_1a404bcc4196ba93c881615180502c4f59" -->

Templatised data accessor. TypeConverters will be used to attempt to convert from the internal representation to the requested representation. 
#### Parameters
* `default_value` The value returned if the conversion failed. 

#### Returns
Data in the requested type.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Variant::GetInto"] refid="class_rml_1_1_variant_1aad8ff6dadf153eacda5edcb333930480" -->

Templatised data accessor. TypeConverters will be used to attempt to convert from the internal representation to the requested representation. 
#### Parameters
* `value` Data in the requested type. 

#### Returns
True if the value was converted and returned, false if no data was stored in the variant. 

:::note
Can be used with enum types, will convert from stored integral value. 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Variant::GetReference"] refid="class_rml_1_1_variant_1a69205cf53cb4a9f39148547d738e0476" -->

Returns a reference to the variant's underlying type. 
:::warning
: Undefined behavior if T does not represent the underlying type of the variant. 

:::

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Variant::Type"] refid="class_rml_1_1_variant_1a262164cff91e2731d0a460d095920015" -->
[Type](#type-1) of data stored in the variant. We use size_t as base to avoid 'padding due to alignment specifier' warning.

Values:

| Name | Description |
|------|-------------|
| `NONE` |  |
| `BOOL` |  |
| `BYTE` |  |
| `CHAR` |  |
| `FLOAT` |  |
| `DOUBLE` |  |
| `INT` |  |
| `INT64` |  |
| `UINT` |  |
| `UINT64` |  |
| `STRING` |  |
| `VECTOR2` |  |
| `VECTOR3` |  |
| `VECTOR4` |  |
| `COLOURF` |  |
| `COLOURB` |  |
| `SCRIPTINTERFACE` |  |
| `TRANSFORMPTR` |  |
| `TRANSITIONLIST` |  |
| `ANIMATIONLIST` |  |
| `DECORATORSPTR` |  |
| `FILTERSPTR` |  |
| `FONTEFFECTSPTR` |  |
| `COLORSTOPLIST` |  |
| `BOXSHADOWLIST` |  |
| `VOIDPTR` |  |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Variant::Set"] refid="class_rml_1_1_variant_1a99b9687c88d187eb5a84af09d10e67f2" -->

Copy another variant's data to this variant. 
:::warning
Does not clear existing data. 

:::

<!-- /rsmlui:block -->

