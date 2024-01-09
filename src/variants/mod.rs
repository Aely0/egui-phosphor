#[cfg(feature = "bold")]
pub mod bold;
#[cfg(feature = "fill")]
pub mod fill;
#[cfg(feature = "light")]
pub mod light;
#[cfg(feature = "regular")]
pub mod regular;
#[cfg(feature = "thin")]
pub mod thin;

#[cfg(not(any(
    feature = "thin",
    feature = "light",
    feature = "regular",
    feature = "bold",
    feature = "fill",
)))]
compile_error!(
    "At least one font variant must be selected as a crate feature. When in doubt, use default features."
);
