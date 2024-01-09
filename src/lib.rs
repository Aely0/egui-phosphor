pub mod variants;
pub use variants::*;

use std::collections::HashMap;

#[cfg(feature = "thin")]
const THIN: &'static [u8] = include_bytes!("../res/Phosphor-Thin.ttf");
#[cfg(feature = "light")]
const LIGHT: &'static [u8] = include_bytes!("../res/Phosphor-Light.ttf");
#[cfg(feature = "regular")]
const REGULAR: &'static [u8] = include_bytes!("../res/Phosphor.ttf");
#[cfg(feature = "bold")]
const BOLD: &'static [u8] = include_bytes!("../res/Phosphor-Bold.ttf");
#[cfg(feature = "fill")]
const FILL: &'static [u8] = include_bytes!("../res/Phosphor-Fill.ttf");

pub fn enabled_variants() -> HashMap<String, &'static [u8]> {
    let mut variants = HashMap::new();

    #[cfg(feature = "thin")]
    variants.insert("phosphor-thin".into(), THIN);
    #[cfg(feature = "light")]
    variants.insert("phosphor-light".into(), LIGHT);
    #[cfg(feature = "regular")]
    variants.insert("phosphor-regular".into(), REGULAR);
    #[cfg(feature = "bold")]
    variants.insert("phosphor-bold".into(), BOLD);
    #[cfg(feature = "fill")]
    variants.insert("phosphor-fill".into(), FILL);

    variants
}

#[macro_export]
macro_rules! insert_font {
    ($name:expr, $variant:expr, $fonts:expr) => {
        let mut font_data = egui::FontData::from_static($variant);
        font_data.tweak.y_offset_factor = 0.1;

        $fonts.font_data.insert($name, font_data);

        if let Some(font_keys) = $fonts.families.get_mut(&egui::FontFamily::Proportional) {
            font_keys.push($name);
        }

        $fonts.families.insert(
            egui::FontFamily::Name($name.into()),
            vec!["Ubuntu-Light".into(), $name],
        );
    };
}

#[macro_export]
macro_rules! add_all_enabled {
    ($ctx:expr) => {
        let mut fonts = egui::FontDefinitions::default();

        for (name, variant) in egui_phosphor::enabled_variants() {
            egui_phosphor::insert_font!(name.clone(), variant, fonts);
        }

        $ctx.set_fonts(fonts);
    };
}

#[macro_export]
macro_rules! add_variants {
    ($ctx:expr, [$($variant:ident),+]) => {
        let mut fonts = egui::FontDefinitions::default();
        let variants = egui_phosphor::enabled_variants();

        $(
            let name = format!("phosphor-{}", stringify!($variant));

            if let Some(variant) = variants.get(&name) {
                egui_phosphor::insert_font!(name.clone(), variant, fonts);
            } else {
                panic!("Attempting to use font that is not enabled ({})", name);
            }
        )+

        $ctx.set_fonts(fonts);
    };
}
