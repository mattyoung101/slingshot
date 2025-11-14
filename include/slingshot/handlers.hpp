// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "lsp/messages.h"

namespace slingshot::handlers {

lsp::requests::Initialize::Result initialise(const lsp::requests::Initialize::Params &&params);

lsp::requests::Shutdown::Result shutdown(const lsp::requests::Shutdown &&params);

void exit(const lsp::notifications::Exit &&params);

} // namespace slingshot::handlers
