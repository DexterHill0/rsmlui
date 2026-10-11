<!-- rsmlui:block["Rml::Log"] refid="class_rml_1_1_log" -->

RmlUi logging API.

C++ include:

```cpp
#include <RmlUi/Core/Log.h>
```
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Log::Message"] refid="class_rml_1_1_log_1a0ef6be822f43a71a3aa02171536f8e4d" -->

[Log](#log) the specified message via the registered log interface

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `type` | `Type` | Type of message. |
| `format` | `const char *` | The message, with sprintf-style parameters. |
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Log::ParseError"] refid="class_rml_1_1_log_1a8a5576b8c632c406b4496d6ac4b2d12f" -->

[Log](#log) a parse error on the specified file and line number.

Parameters:

| Name | Type | Description |
|------|------|-------------|
| `filename` | `const String &` | Name of the file with the parse error. |
| `line_number` | `int` | Line the error occurred on. |
| `format` | `const char *` | The error message, with sprintf-style parameters. |
<!-- /rsmlui:block -->

