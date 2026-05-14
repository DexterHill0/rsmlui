<!-- rsmlui:block["Rml::Colour"] refid="class_rml_1_1_colour" -->

Templated class for a four-component RGBA colour.

C++ include:

```cpp
#include <RmlUi/Core/Colour.h>
```
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::Colour"] refid="class_rml_1_1_colour_1a54146996d171f328f01fb5bbe2e4ab8a" -->

Initialising constructor. 
#### Parameters
* `rgb` Initial red, green and blue value of the colour. 

* `alpha` Initial alpha value of the colour.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::Colour"] refid="class_rml_1_1_colour_1ac73a949b42ecff51c9cf2303bd2c1a03" -->

Initialising constructor. 
#### Parameters
* `red` Initial red value of the colour. 

* `green` Initial green value of the colour. 

* `blue` Initial blue value of the colour. 

* `alpha` Initial alpha value of the colour.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator+"] refid="class_rml_1_1_colour_1a8e16f28405c30739c8c21760a0726cfe" -->

Returns the sum of this colour and another. This does not saturate the channels. 
#### Parameters
* `rhs` The colour to add this to. 

#### Returns
The sum of the two colours.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator-"] refid="class_rml_1_1_colour_1ade1af019dd09f9e51a4271ad225b261e" -->

Returns the result of subtracting another colour from this colour. 
#### Parameters
* `rhs` The colour to subtract from this colour. 

#### Returns
The result of the subtraction.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator*"] refid="class_rml_1_1_colour_1a8518993eaf81b360dbf4f30c9ee3bea5" -->

Returns the result of multiplying this colour component-wise by a scalar. 
#### Parameters
* `rhs` The scalar value to multiply by. 

#### Returns
The result of the scale.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator/"] refid="class_rml_1_1_colour_1ad9c1de209105a08896cbaa6a521b9405" -->

Returns the result of dividing this colour component-wise by a scalar. 
#### Parameters
* `rhs` The scalar value to divide by. 

#### Returns
The result of the scale.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator+="] refid="class_rml_1_1_colour_1a0402caf24c6341cca75a288bc7c2bd7a" -->

Adds another colour to this in-place. This does not saturate the channels. 
#### Parameters
* `rhs` The colour to add.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator-="] refid="class_rml_1_1_colour_1aab9bba10466a714457d38680e78734b0" -->

Subtracts another colour from this in-place. 
#### Parameters
* `rhs` The colour to subtract.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator*="] refid="class_rml_1_1_colour_1a24b830f55805e8f5e3423b26b157dbd7" -->

Scales this colour component-wise in-place. 
#### Parameters
* `rhs` The value to scale this colour's components by.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator/="] refid="class_rml_1_1_colour_1a110ecdd9e6eb45eecc46febc1f41f3f8" -->

Scales this colour component-wise in-place by the inverse of a value. 
#### Parameters
* `rhs` The value to divide this colour's components by.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator=="] refid="class_rml_1_1_colour_1a0476079a61d742dea0e37c108ffedc85" -->

Equality operator. 
#### Parameters
* `rhs` The colour to compare this against. 

#### Returns
True if the two colours are equal, false otherwise.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator!="] refid="class_rml_1_1_colour_1a39046a6f8365d6443ded336b25a678d9" -->

Inequality operator. 
#### Parameters
* `rhs` The colour to compare this against. 

#### Returns
True if the two colours are not equal, false otherwise.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator const ColourType *"] refid="class_rml_1_1_colour_1afab52df2ed0e929363fec93560431c82" -->

Auto-cast operator. 
#### Returns
A pointer to the first value.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Colour::operator ColourType *"] refid="class_rml_1_1_colour_1a66e2f41bee478b897dbee7b19b04a107" -->

Constant auto-cast operator. 
#### Returns
A constant pointer to the first value.

<!-- /rsmlui:block -->

