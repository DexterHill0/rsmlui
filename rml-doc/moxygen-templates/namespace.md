<!-- rsmlui:block["{{name}}"] refid="{{refid}}" -->
{{#if (eq kind "group")}}
{{summary}}
{{else}}
{{briefdescription}}

{{detaileddescription}}
{{/if}}
<!-- /rsmlui:block -->

{{#each filtered.sections}}
{{#each members}}
{{#if (memberSummary this)}}
<!-- rsmlui:block["{{../../name}}::{{name}}"] refid="{{refid}}" -->
{{briefdescription}}

{{detaileddescription}}

{{#unless briefdescription}}
{{#unless detaileddescription}}
{{memberSummary this}}
{{/unless}}
{{/unless}}
{{#if returnTypeShort}}

Return type: `{{returnTypeShort}}`
{{/if}}
{{#if signature}}

C++ signature:

```cpp
{{signature}}
```
{{/if}}
{{#if (hasDocumentedParams params)}}

Parameters:

| Name | Type | Description |
|------|------|-------------|
{{#each (documentedParams params)}}| `{{name}}` | `{{type}}` | {{description}} |
{{/each}}
{{/if}}
{{#if enumvalue}}

Values:

| Name | Description |
|------|-------------|
{{#each enumvalue}}| `{{name}}` | {{summary}} |
{{/each}}
{{/if}}
<!-- /rsmlui:block -->

{{/if}}
{{/each}}
{{/each}}
