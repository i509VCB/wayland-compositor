use ash::vk;
use smithay::{
    backend::{allocator, renderer::gles2::ffi::types::GLuint},
    reexports::wayland_server::protocol::wl_shm,
};

macro_rules! format_tables {
    (
        $(
            $fourcc_wl: ident {
                $(opaque: $opaque: expr,)?
                alpha: $alpha: expr,
                $(gl: $gl: ident,)?
                $(
                    // The meta fragment specifier exists because the in memory representation of
                    // `A8B8G8R8_SRGB_PACK32` depends on the host endianness.
                    $(#[$vk_meta: meta])*
                    vk: $vk: ident,
                )?
            }
        ),* $(,)?
    ) => {
        /// Returns an equivalent Vulkan format from the specified fourcc code.
        ///
        /// The second field of the returned tuple describes whether Vulkan needs to swizzle the alpha
        /// component.
        pub const fn fourcc_to_vk(
            fourcc: smithay::backend::allocator::Fourcc,
        ) -> Option<(ash::vk::Format, bool)> {
            match fourcc {
                $($(
                    $(#[$vk_meta])*
                    smithay::backend::allocator::Fourcc::$fourcc_wl => Some((ash::vk::Format::$vk, $alpha)),
                )*)*

                _ => None
            }
        }

        /// Returns an equivalent Vulkan format from the specified wl_shm code.
        ///
        /// The second field of the returned tuple describes whether Vulkan needs to swizzle the alpha
        /// component.
        pub const fn wl_shm_to_vk(
            wl: smithay::reexports::wayland_server::protocol::wl_shm::Format,
        ) -> Option<(ash::vk::Format, bool)> {
            match wl {
                $($(
                    $(#[$vk_meta])*
                    smithay::reexports::wayland_server::protocol::wl_shm::Format::$fourcc_wl
                        => Some((ash::vk::Format::$vk, $alpha)),
                )*)*

                _ => None
            }
        }

        // TODO: vk to fourcc and wl_shm
    };
}

format_tables! {
    // Formats mandated by wl_shm

    // Using the first entry as a reference, this is how the syntax works:
    //
    // The first thing we declare is fourcc code. The fourcc code should appear before opening the braces.
    Argb8888 {
        // Next we need to provide data as to whether the color format has an alpha channel.
        //
        // This is a required value. Some renderers do not have specific no-alpha formats but support
        // indicating which color channels should be used.
        //
        // For example, Vulkan does not have specific formats to indicate there is a padding byte where the
        // alpha channel would exist in another format. Vulkan however allows specifying which color
        // components to use in an image view via the VkComponentSwizzle enum, allowing the alpha channel to
        // be disabled.
        alpha: true,
        // Now conversions to other formats may be specified.
        //
        // You may specify how to convert a fourcc code to an OpenGL or Vulkan format.
        //
        // These fields are optional, omitting them indicates there is no compatible format mapping.

        // For Vulkan, we can only use SRGB formats or else we need to convert the format.
        vk: B8G8R8A8_SRGB,
    },

    Xrgb8888 {
        alpha: false,
        vk: B8G8R8A8_SRGB,
    },

    // Non-mandatory formats

    Abgr8888 {
        alpha: true,
        vk: R8G8B8A8_SRGB,
    },

    Xbgr8888 {
        alpha: false,
        vk: R8G8B8A8_SRGB,
    },

    // The PACK32 formats are equivalent to a u32 instead of a [u8; 4].
    //
    // This means these formats will depend on the host endianness.
    // On little endian, this means we have a valid format mapping. On big endian, the format is is
    // represented in memory the exact same as ABGR8888, which we already have a mapping for. 
    Rgba8888 {
        alpha: true,
        #[cfg(target_endian = "little")]
        vk: A8B8G8R8_SRGB_PACK32,
    },

    Rgbx8888 {
        alpha: false,
        #[cfg(target_endian = "little")]
        vk: A8B8G8R8_SRGB_PACK32,
    },

    Bgr888 {
        alpha: false,
        vk: R8G8B8_SRGB,
    },

    Rgb888 {
        alpha: false,
        vk: B8G8R8_SRGB,
    },

    R8 {
        alpha: false,
        vk: R8_SRGB,
    },

    Gr88 {
        alpha: false,
        vk: R8G8_SRGB,
    },

    // § 3.9.3. 16-Bit Floating-Point Numbers
    //
    // > 16-bit floating point numbers are defined in the “16-bit floating point numbers” section of the
    // > Khronos Data Format Specification.
    //
    // The khronos data format defines a 16-bit floating point number as a half precision IEEE 754-2008 float
    // (binary16).
    //
    // Since the DRM Fourcc formats that are floating point are also IEEE-754, Vulkan can represent some
    // floating point Drm Fourcc formats.

    Abgr16161616f {
        alpha: true,
        vk: R16G16B16A16_SFLOAT,
    },

    Xbgr16161616f {
        alpha: false,
        vk: R16G16B16A16_SFLOAT,
    }
}
