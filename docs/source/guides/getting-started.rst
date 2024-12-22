.. _getting-started:

Getting Started
===============

Setting up a project for the Nitro/Twilight requires some additional setup compared to a standard
Rust executable.

Requirements
------------

- A copy of GNU Binutils for the ``arm-none-eabi`` platform.

 - For Arch, this can be installed with ``pacman -S arm-none-eabi-binutils``.
 - For Gentoo, this can be acquired with `crossdev <https://wiki.gentoo.org/wiki/Crossdev>`_.
 - On other Linux distributions, the package name will likely be something along the lines of
   ``binutils-arm-none-eabi``. Do not use the devkitpro binutils.

- Rust nightly must be installed, because ``build-std`` is *still* unstable.
- The ``rust-src`` component must be installed for the usage of ``build-std``.

Choosing a target
-----------------

SquidTWL supports two different targets: the original Nintendo DS (codenamed and referred to as 
Nitro/NTR) and the Nintendo DSi (codenamed and referred to as Twilight/TWL). The TWL is a strict
superset of the NTR's hardware, with upgrades such as four times more main memory, cameras, access
to a proper storage location with the SD card/built-in flash storage, and improved wifi support.

The NTR is better for learning how to program in a more constrained embedded environment with
its limited capabilities. It is unlikely that any modern-day homebrew will run on a real Nitro,
with the 3DS and DSi being the systems of choice due to being easier to hack.

Creating a project
------------------

Create your project with Cargo, like so:

.. code-block:: shell

    $ cargo init --bin --edition 2024 .

Copy the `linker configurations`_ for your chosen target to your project directory, and create 
a ``build.rs`` file:

.. code-block:: rust

    // No, I can't propagate this down to you, because the Cargo devs removed this feature with
    // the stated justification of "fuck you". 
    // See: https://github.com/rust-lang/cargo/issues/9554
    pub fn main() {
        println!("cargo:rustc-link-arg=-Tnitro.ld")
    }

Create a new ``.cargo/config.toml`` file with the ``build-std`` configuration:

.. code-block:: toml

    [build]
    target = "armv5te-none-eabi"

    [unstable]
    build-std = ["core", "alloc"]

    [target.armv5te-none-eabi]
    rustflags = [
        "-Clinker=arm-none-eabi-ld",
    ]

Create a ``main.rs``:

.. code-block:: rust

    #![no_std]
    #![no_main]

    #[unsafe(no_mangle)]
    extern "C" fn main() {
        // ...
    }

You can then build your ELF file with ``cargo build`` and pack it into a rom.

.. _linker configurations: https://github.com/SquidTWL/SquidTWL/tree/mizuki/linker
