<!-- rsmlui:block["Rml::Plugin"] refid="class_rml_1_1_plugin" -->

Generic Interface for plugins to RmlUi.

C++ include:

```cpp
#include <RmlUi/Core/Plugin.h>
```
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::GetEventClasses"] refid="class_rml_1_1_plugin_1abbc7ed034020bc50bc2d7c84d2a1187b" -->
Called when the plugin is registered to determine which of the above event types the plugin is interested in.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnInitialise"] refid="class_rml_1_1_plugin_1abb6096fae4cad5e8981d7662290c1234" -->

Called when RmlUi is initialised, or immediately when the plugin registers itself if RmlUi has already been initialised.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnShutdown"] refid="class_rml_1_1_plugin_1af07d01328649b783b847f9b73b391aa4" -->
Called when RmlUi shuts down.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnContextCreate"] refid="class_rml_1_1_plugin_1a4aaa0c192a8782bba7fc85fdb5db519b" -->
Called when a new context is created.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnContextDestroy"] refid="class_rml_1_1_plugin_1adb4c648e189f6085cd1fe297d686ff25" -->
Called when a context is destroyed.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnDocumentOpen"] refid="class_rml_1_1_plugin_1af33c4f1a148c652760768a620a65d415" -->
Called when a document load request occurs, before the document's file is opened.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnDocumentLoad"] refid="class_rml_1_1_plugin_1a227942e8dea8a0c096c26ac595ae47e4" -->

Called when a document is successfully loaded from file or instanced, initialised and added to its context. This is called before the document's 'load' event.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnDocumentUnload"] refid="class_rml_1_1_plugin_1a908dcc30dc06512141155476befa8c51" -->

Called when a document is unloaded from its context. This is called after the document's 'unload' event.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnElementCreate"] refid="class_rml_1_1_plugin_1af0ac25c1c53c2080c0ce83dc3c5bd50a" -->
Called when a new element is created.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnElementDestroy"] refid="class_rml_1_1_plugin_1aaee69422c555be551613b2be08127f80" -->
Called when an element is destroyed.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnDataModelCreate"] refid="class_rml_1_1_plugin_1a6e2b944f460829c5845be5c1bfdbdcfb" -->
Called when a new data model is created on a context.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::Plugin::OnDataModelDestroy"] refid="class_rml_1_1_plugin_1acead09c669bba08b0a5651f0718ffcd2" -->

Called when a data model is about to be destroyed, either explicitly through [Context::RemoveDataModel](api_Rml-Context.md#removedatamodel) or implicitly when the owning context is destroyed. At this point the data model is still accessible through [Context::GetDataModel](api_Rml-Context.md#getdatamodel), but will be unusable as soon as this callback returns.

<!-- /rsmlui:block -->

