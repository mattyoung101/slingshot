/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// required to make certain we link against libstdc++
extern crate link_cplusplus;

// Include the generated slingshot-cpp bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod slingshot_ergo;
pub mod diagnostics;
pub mod completion;
pub mod indexing;
