# libslingshot bindings
Since getting slingshot-cpp (libslingshot) and the Rust part of this project to work together was such a 
complete and utter fucking shitstorm, I'm going to document/rant about how this works to
hopefully save future grief and in case anything goes wrong.

Also, since this is my first time with Rust, I've most likely done this all wrong and over-complicated it,
so suggestions welcome.

## Chapter 1: C++ implementation, what is libslingshot?
The slingshot-cpp subdirectory exists to provide a library, libslingshot. The one and only purpose of
libslingshot is to interface with the Slang SystemVerilog parser, which is written from C++.

libslingshot and slingshot-cpp exist so that the rest of the project can be written in Rust.

## Chapter 2: Fighting C++: Writing a C API
The first state of warfare begins with C++. In order to get C++ and Rust tied nicely together, you basically
need to expose a C API so you get the stable C ABI. There's no way around it. Even crates like cxx.rs do
that as well, but they generate the C API code for you behind the scenes. Sure, you "can" use rust-bindgen
with C++, but there is literally no point as none of the types it emits can be used anywhere in Rust at all.

In a somewhat typical fashion, cxx.rs doesn't include a build config for one of C++'s most widely used build 
tools, CMake. There are unofficial configs, but most seem over-complicated and focus on either embedding Rust
_in_ C++ (which we don't want), or figuring out new features like cross-language LTO (which we also don't
really care about).

So, what I ended up doing was manually writing the C interface and using rust-bindgen. Not the end of the
world, but now we have to AddressSanitize, and especially LeakSanitize, slingshot-cpp. A memory leak in a
long running language server process would be catastrophic.

## Chapter 3: Fighting Rust: Generating bindings
With the C API written, we now turn our head to getting rust-bindgen to generate the Rust side of the bindings.
Oh man. Enter the world of build.rs. As much as I want to whine and say this is a bad idea, and feels gross,
I can't think of a better way of doing it myself.

Debugging build.rs scripts is nigh impossible. They only talk to Cargo, so the only way to get messages _out_
of these build scripts is to ask Cargo to print a warning, which is really gross. This is really problematic
because we are trying to tell rust-bindgen where the header files live, and Cargo where to link the libslingshot
static library. In typical fashion, the linker error messages are useless and cryptic, so a lot of printfing
and navigating around Rust's target directory was in order.

I was eventually able to get it working with this guide: https://rendered-obsolete.github.io/2018/09/30/rust-ffi-ci.html

Another good one to finish off on is it appears that, to my knowledge, rust-bindgen will _only_ look in the
_root directory of your project_ for the header file you want to generate bindings for. Nowhere else. It does
not understand subdirectories. Root directory only. So, this is why `wrapper.h` is in the root directory.

## Chapter 4: Boss battle: Fighting the linker
This is somewhat of a misnomer being called Chapter 4, because truth be told I had been fighting the linker
since day 1. However, in that case it was mostly my fault, as I was using new and experimental stuff like
Mold and LTO when I shouldn't have been. After removing all of that, the linker worked better. But not
better enough.

- need to shared link static library that's fun
- LTO -> doesn't work, I forgot to disable, who knows why
- ASan -> also doesn't work, who knows why
- fat static library -> link libslingshot and libsvlang to libchungus (libslingshotfat) 
    - linker errors otherwise, `target_link_libraries(slingshot PUBLIC slang::slang)` will **NOT** link slang
    in libslingshot.a
- rust-cmake doesn't work lmao slang version number is wrong
