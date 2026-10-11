<!-- rsmlui:block["Rml::Colour"] refid="class_rml_1_1_colour" -->

Templated class for a four-component RGBA colour.

C++ include:

```cpp
#include <RmlUi/Core/Colour.h>
```
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::Colour"] refid="class_rml_1_1_colour_1a54146996d171f328f01fb5bbe2e4ab8a" -->

Initialising constructor.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rgb` | `ColourType` | Initial red, green and blue value of the colour. |
| `alpha` | `ColourType` | Initial alpha value of the colour. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::Colour"] refid="class_rml_1_1_colour_1ac73a949b42ecff51c9cf2303bd2c1a03" -->

Initialising constructor.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `red` | `ColourType` | Initial red value of the colour. |
| `green` | `ColourType` | Initial green value of the colour. |
| `blue` | `ColourType` | Initial blue value of the colour. |
| `alpha` | `ColourType` | Initial alpha value of the colour. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator+"] refid="class_rml_1_1_colour_1a8e16f28405c30739c8c21760a0726cfe" -->

Returns the sum of this colour and another. This does not saturate the channels. 
#### Returns
The sum of the two colours.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `[Colour](#colour-1)` | The colour to add this to. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator-"] refid="class_rml_1_1_colour_1ade1af019dd09f9e51a4271ad225b261e" -->

Returns the result of subtracting another colour from this colour. 
#### Returns
The result of the subtraction.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `[Colour](#colour-1)` | The colour to subtract from this colour. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator*"] refid="class_rml_1_1_colour_1a8518993eaf81b360dbf4f30c9ee3bea5" -->

Returns the result of multiplying this colour component-wise by a scalar. 
#### Returns
The result of the scale.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `float` | The scalar value to multiply by. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator/"] refid="class_rml_1_1_colour_1ad9c1de209105a08896cbaa6a521b9405" -->

Returns the result of dividing this colour component-wise by a scalar. 
#### Returns
The result of the scale.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `float` | The scalar value to divide by. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator+="] refid="class_rml_1_1_colour_1a0402caf24c6341cca75a288bc7c2bd7a" -->

Adds another colour to this in-place. This does not saturate the channels.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `[Colour](#colour-1)` | The colour to add. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator-="] refid="class_rml_1_1_colour_1aab9bba10466a714457d38680e78734b0" -->

Subtracts another colour from this in-place.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `[Colour](#colour-1)` | The colour to subtract. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator*="] refid="class_rml_1_1_colour_1a24b830f55805e8f5e3423b26b157dbd7" -->

Scales this colour component-wise in-place.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `float` | The value to scale this colour's components by. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator/="] refid="class_rml_1_1_colour_1a110ecdd9e6eb45eecc46febc1f41f3f8" -->

Scales this colour component-wise in-place by the inverse of a value.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `float` | The value to divide this colour's components by. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator=="] refid="class_rml_1_1_colour_1a0476079a61d742dea0e37c108ffedc85" -->

Equality operator. 
#### Returns
True if the two colours are equal, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `[Colour](#colour-1)` | The colour to compare this against. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator!="] refid="class_rml_1_1_colour_1a39046a6f8365d6443ded336b25a678d9" -->

Inequality operator. 
#### Returns
True if the two colours are not equal, false otherwise.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `rhs` | `[Colour](#colour-1)` | The colour to compare this against. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator[]"] refid="class_rml_1_1_colour_1a192a11dd03e57475c98c402b250ec929" -->

Returns a reference to the colour component at the given index.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `index` | `size_t` | The index of the colour component (0 = red, 1 = green, 2 = blue, 3 = alpha). |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator[]"] refid="class_rml_1_1_colour_1a85d31525d6b6d777031a6ea0bb36b9a9" -->

Returns a reference to the colour component at the given index.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `index` | `size_t` | The index of the colour component (0 = red, 1 = green, 2 = blue, 3 = alpha). |
<!-- /rsmlui:block -->

