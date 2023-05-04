#![allow(dead_code)] // temporary

use iced::Font;

pub const BLACK: Font = Font::External {
    name: "Inter Black",
    bytes: include_bytes!("../assets/fonts/Inter-Black.ttf")
};

pub const BOLD: Font = Font::External {
    name: "Inter Bold",
    bytes: include_bytes!("../assets/fonts/Inter-Bold.ttf")
};

pub const EXTRA_BOLD: Font = Font::External {
    name: "Inter Extra Bold",
    bytes: include_bytes!("../assets/fonts/Inter-ExtraBold.ttf")
};

pub const EXTRA_LIGHT: Font = Font::External {
    name: "Inter Extra Light",
    bytes: include_bytes!("../assets/fonts/Inter-ExtraLight.ttf")
};

pub const LIGHT: Font = Font::External {
    name: "Inter Light",
    bytes: include_bytes!("../assets/fonts/Inter-Light.ttf")
};

pub const MEDIUM: Font = Font::External {
    name: "Inter Medium",
    bytes: include_bytes!("../assets/fonts/Inter-Medium.ttf")
};

pub const REGULAR: Font = Font::External {
    name: "Inter Regular",
    bytes: include_bytes!("../assets/fonts/Inter-Regular.ttf")
};

pub const SEMI_BOLD: Font = Font::External {
    name: "Inter Semi Bold",
    bytes: include_bytes!("../assets/fonts/Inter-SemiBold.ttf")
};

pub const THIN: Font = Font::External {
    name: "Inter Thin",
    bytes: include_bytes!("../assets/fonts/Inter-Thin.ttf")
};
