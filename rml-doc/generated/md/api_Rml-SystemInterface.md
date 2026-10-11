<!-- rsmlui:block["Rml::SystemInterface"] refid="class_rml_1_1_system_interface" -->

RmlUi's system interface provides an interface for time, translation, logging, and other system utilities.

The default logging implementation outputs to the Windows Debug Console on Windows, and Standard Error on other platforms.

C++ include:

```cpp
#include <RmlUi/Core/SystemInterface.h>
```

C++ inheritance: `Rml::NonCopyMoveable`
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SystemInterface::GetElapsedTime"] refid="class_rml_1_1_system_interface_1ad8bf3b41df43634039a67d1a60dfb8bc" -->

Get the number of seconds elapsed since the start of the application. 
#### Returns
Elapsed time, in seconds.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SystemInterface::TranslateString"] refid="class_rml_1_1_system_interface_1ab0b1e560e5ee57c74554dc95a6d68c17" -->

Translate the input string into the translated string. 
#### Returns
Number of translations that occured.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `translated` | `String &` | Translated string ready for display. |
| `input` | `const String &` | String as received from XML. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SystemInterface::JoinPath"] refid="class_rml_1_1_system_interface_1a0866b454afd5a51689a59aa0ca006593" -->

Joins the path of an RML or RCSS file with the path of a resource specified within the file.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `translated_path` | `String &` | The joined path. |
| `document_path` | `const String &` | The path of the source document (including the file name). |
| `path` | `const String &` | The path of the resource specified in the document. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SystemInterface::LogMessage"] refid="class_rml_1_1_system_interface_1a0aa450786bb2e38d01e1035e04099774" -->

[Log](api_Rml-Log.md#log) the specified message. 
#### Returns
True to continue execution, false to break into the debugger.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `type` | `Log::Type` | Type of log message, ERROR, WARNING, etc. |
| `message` | `const String &` | Message to log. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SystemInterface::SetMouseCursor"] refid="class_rml_1_1_system_interface_1a976e8be014c478d05c865489b5d9c7f6" -->

Set mouse cursor.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `cursor_name` | `const String &` | Cursor name to activate. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SystemInterface::SetClipboardText"] refid="class_rml_1_1_system_interface_1afd09cc5a142fce7ca80df75dd16a6f92" -->

Set clipboard text.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `text` | `const String &` | Text to apply to clipboard. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SystemInterface::GetClipboardText"] refid="class_rml_1_1_system_interface_1a835de3745f7b5e311cf3115df1208a5f" -->

Get clipboard text.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `text` | `String &` | Retrieved text from clipboard. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SystemInterface::ActivateKeyboard"] refid="class_rml_1_1_system_interface_1a7a6cda2d4e26c551e6b577c315a0602c" -->

Activate keyboard (for touchscreen devices).

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `caret_position` | `Rml::Vector2f` | Position of the caret in absolute window coordinates. |
| `line_height` | `float` | Height of the current line being edited. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::SystemInterface::DeactivateKeyboard"] refid="class_rml_1_1_system_interface_1a5d5fd57ab33f4ade2b71b5510ad9225c" -->
Deactivate keyboard (for touchscreen devices).

<!-- /rsmlui:block -->

