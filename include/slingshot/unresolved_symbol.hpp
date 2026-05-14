// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2026 Mel Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

#pragma once
#include <ankerl/unordered_dense.h>
#include <filesystem>
#include <optional>

// This has to be in an ENTIRELY FUCKING SEPARATE FILE because of how dogshit of a language C++ is right
// because the fucking template specialisation has to be in every fuuuuucking file that ever wants to use it.
// We can't put that shit in slingshot.hpp because otherwise we get dependency cycles. And this is why
// actually serious languages concocted after the year 1980 use a fucking reasonable, sensible, include
// system. Before you say, NO, fucking C++ modules is somehow an even bigger piece of shit than #include. Fuck
// you. Fuck this language. Peace.

namespace slingshot {

class UnresolvedSymbol {
public:
    /// LHS, this side provides the symbol
    std::optional<std::filesystem::path> lhs;
    /// RHS, this side requires the symbol
    std::optional<std::filesystem::path> rhs;
    std::string symbol;
    /// Is this symbol *maybe* required?
    bool maybe;

    bool operator==(const UnresolvedSymbol &s) const {
        return lhs == s.lhs && rhs == s.rhs && symbol == s.symbol && maybe == s.maybe;
    }
};

} // namespace slingshot

template <>
struct ankerl::unordered_dense::hash<slingshot::UnresolvedSymbol> {
    using is_avalanching = void;

    [[nodiscard]] auto operator()(slingshot::UnresolvedSymbol const &x) const noexcept -> uint64_t {
        uint64_t hash = 0xBEEF;

        auto lhs = ankerl::unordered_dense::hash<std::optional<std::filesystem::path>> { }(x.lhs);
        auto rhs = ankerl::unordered_dense::hash<std::optional<std::filesystem::path>> { }(x.rhs);
        auto symbol = ankerl::unordered_dense::hash<std::string> { }(x.symbol);
        auto maybe = ankerl::unordered_dense::hash<bool> { }(x.maybe);

        hash = detail::wyhash::mix(hash, lhs);
        hash = detail::wyhash::mix(hash, rhs);
        hash = detail::wyhash::mix(hash, symbol);
        hash = detail::wyhash::mix(hash, maybe);

        return hash;
    }
};
