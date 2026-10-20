<!-- rsmlui:block["Rml::XMLNodeHandler"] refid="class_rml_1_1_x_m_l_node_handler" -->

A handler gets ElementStart, ElementEnd and ElementData called by the [XMLParser](api_Rml-XMLParser.md#xmlparser).

C++ include:

```cpp
#include <RmlUi/Core/XMLNodeHandler.h>
```

C++ inheritance: `Rml::NonCopyMoveable`
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLNodeHandler::ElementStart"] refid="class_rml_1_1_x_m_l_node_handler_1a7102b38f9861a11985b279ad204e018a" -->

Called when a new element tag is opened. 
#### Returns
The new element, may be nullptr if no element was created.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `parser` | `[XMLParser](api_Rml-XMLParser.md#xmlparser) *` | The parser executing the parse. |
| `name` | `const String &` | The XML tag name. |
| `attributes` | `const XMLAttributes &` | The tag attributes. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLNodeHandler::ElementEnd"] refid="class_rml_1_1_x_m_l_node_handler_1aff71ab173d2664117775605d71e6baf7" -->

Called when an element is closed.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `parser` | `[XMLParser](api_Rml-XMLParser.md#xmlparser) *` | The parser executing the parse. |
| `name` | `const String &` | The XML tag name. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLNodeHandler::ElementData"] refid="class_rml_1_1_x_m_l_node_handler_1a7ff8153d8f1b0d8b3854ab2c5df40ba9" -->

Called for element data.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `parser` | `[XMLParser](api_Rml-XMLParser.md#xmlparser) *` | The parser executing the parse. |
| `data` | `const String &` | The element data. |
<!-- /rsmlui:block -->

