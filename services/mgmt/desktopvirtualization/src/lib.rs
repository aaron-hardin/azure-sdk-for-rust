#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-preview-2022-02")]
pub mod package_preview_2022_02;
#[cfg(all(feature = "package-preview-2022-02", not(feature = "no-default-tag")))]
pub use package_preview_2022_02::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-preview-2021-09")]
pub mod package_preview_2021_09;
#[cfg(all(feature = "package-preview-2021-09", not(feature = "no-default-tag")))]
pub use package_preview_2021_09::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-07")]
pub mod package_2021_07;
#[cfg(all(feature = "package-2021-07", not(feature = "no-default-tag")))]
pub use package_2021_07::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-04-01-preview")]
pub mod package_2021_04_01_preview;
#[cfg(all(feature = "package-2021-04-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_04_01_preview::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-03-09-preview")]
pub mod package_2021_03_09_preview;
#[cfg(all(feature = "package-2021-03-09-preview", not(feature = "no-default-tag")))]
pub use package_2021_03_09_preview::{models, operations, operations::Client, operations::ClientBuilder};
