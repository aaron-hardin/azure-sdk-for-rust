#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2022-01-31")]
pub mod package_2022_01_31;
#[cfg(all(feature = "package-2022-01-31", not(feature = "no-default-tag")))]
pub use package_2022_01_31::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-06-22")]
pub mod package_2021_06_22;
#[cfg(all(feature = "package-2021-06-22", not(feature = "no-default-tag")))]
pub use package_2021_06_22::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2020-01-13-preview")]
pub mod package_2020_01_13_preview;
#[cfg(all(feature = "package-2020-01-13-preview", not(feature = "no-default-tag")))]
pub use package_2020_01_13_preview::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2019-06")]
pub mod package_2019_06;
#[cfg(all(feature = "package-2019-06", not(feature = "no-default-tag")))]
pub use package_2019_06::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2018-06-preview")]
pub mod package_2018_06_preview;
#[cfg(all(feature = "package-2018-06-preview", not(feature = "no-default-tag")))]
pub use package_2018_06_preview::{models, operations, operations::Client, operations::ClientBuilder};
