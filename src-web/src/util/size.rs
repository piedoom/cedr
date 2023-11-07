use std::fmt::Display;

#[derive(PartialEq)]
pub enum Size {
    Px(f32),
    Em(f32),
    Percent(f32),
    Auto,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Size::Px(s) => format!("{s}px"),
            Size::Em(s) => format!("{s}em"),
            Size::Percent(s) => format!("{s}%"),
            Size::Auto => "auto".into(),
        };
        write!(f, "{}", s)
    }
}
