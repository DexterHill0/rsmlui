#pragma once
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/RenderInterface.h>
#include <RmlUi/Core/Span.h>
#include <RmlUi_Backend.h>

#include <cstddef>

#include "RmlUi/Core/Input.h"
#include "RmlUi/Core/SystemInterface.h"
#include "RmlUi_Backend.h"

namespace rsmlui {
using Context = Rml::Context;
using RenderInterface = Rml::RenderInterface;
using SystemInterface = Rml::SystemInterface;
using Vertex = Rml::Vertex;
using Vector2f = Rml::Vector2f;
using Vector2i = Rml::Vector2i;
using Rectanglei = Rml::Rectanglei;
using ClipMaskOperation = Rml::ClipMaskOperation;
using BlendMode = Rml::BlendMode;
using ElementDocument = Rml::ElementDocument;
using KeyIdentifier = Rml::Input::KeyIdentifier;
using KeyDownCallback = KeyDownCallback;
template<typename T>
using Span = Rml::Span<T>;
} // namespace rsmlui