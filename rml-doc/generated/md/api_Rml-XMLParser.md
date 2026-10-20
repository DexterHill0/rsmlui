<!-- rsmlui:block["Rml::XMLParser"] refid="class_rml_1_1_x_m_l_parser" -->

RmlUi's XML parsing engine. The factory creates an instance of this class for each RML parse.

C++ include:

```cpp
#include <RmlUi/Core/XMLParser.h>
```

C++ inheritance: `Rml::BaseXMLParser`
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::GetDocumentHeader"] refid="class_rml_1_1_x_m_l_parser_1a8d12c89e090870c4c129027acd2412cf" -->

Returns the XML document's header. 
#### Returns
The document header.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::PushHandler"] refid="class_rml_1_1_x_m_l_parser_1af742b0cf228066b436a984e71d97faa1" -->

Pushes an element handler onto the parse stack for parsing child elements. 
#### Returns
True if an appropriate handler was found and pushed onto the stack, false if not.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `tag` | `const String &` | The tag the handler was registered with. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::PushDefaultHandler"] refid="class_rml_1_1_x_m_l_parser_1a565576647d1f6770a4088560f545588a" -->
Pushes the default element handler onto the parse stack.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::GetParseFrame"] refid="class_rml_1_1_x_m_l_parser_1a5fce732eb84e8845f48ded8a7977dcdd" -->
Access the current parse frame.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::GetSourceURL"] refid="class_rml_1_1_x_m_l_parser_1a1130e4f2cbd94c9591d8e7b3c5ef7391" -->
Returns the source URL of this parse.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::HandleData"] refid="class_rml_1_1_x_m_l_parser_1aa202fda36e9e5cb4b9437c832db0979e" -->
Called when the parser encounters data.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::RegisterPersistentCDATATag"] refid="class_rml_1_1_x_m_l_parser_1a523cf95158d5e30330044758787b749c" -->

Registers a tag were its contents should be treated as CDATA. Whereas BaseXMLParser RegisterCDataTag only registeres a tag for a parser instance, this function willl register a tag for all [XMLParser](#xmlparser) instances created after this call.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `_tag` | `const String &` | The tag for contents to be treated as CDATA |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::RegisterNodeHandler"] refid="class_rml_1_1_x_m_l_parser_1a3b616575c7ab9dbe310edefa8c65839c" -->

Registers a custom node handler to be used to a given tag. 
#### Returns
The registered XML node handler.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `tag` | `const String &` | The tag the custom parser will handle. |
| `handler` | `SharedPtr< [XMLNodeHandler](api_Rml-XMLNodeHandler.md#xmlnodehandler) >` | The custom handler. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::GetNodeHandler"] refid="class_rml_1_1_x_m_l_parser_1a01104c3dca9b944299b32ec91b795725" -->

Retrieve a registered node handler. 
#### Returns
The registered XML node handler or nullptr if it does not exist for the given tag.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `tag` | `const String &` | The tag the custom parser handles. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::ReleaseHandlers"] refid="class_rml_1_1_x_m_l_parser_1a8d954b55b6f03f4886fe982294aaf73a" -->
Releases all registered node handlers. This is called internally.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::HandleElementStart"] refid="class_rml_1_1_x_m_l_parser_1a18c4cec31bd7f8cc394bea6d30ff0340" -->
Called when the parser finds the beginning of an element tag.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::XMLParser::HandleElementEnd"] refid="class_rml_1_1_x_m_l_parser_1ade89c1a6f02ff16f3fe6b9e1ebde29e4" -->
Called when the parser finds the end of an element tag.

<!-- /rsmlui:block -->

