use iced::Padding;

#[derive(Debug, PartialEq, Clone)]
pub struct OnagrePadding {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl OnagrePadding {
    pub const ZERO: OnagrePadding = OnagrePadding {
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
    };

    pub fn to_iced_padding(&self) -> Padding {
        Padding {
            top: self.top as f32,
            right: self.right as f32,
            bottom: self.bottom as f32,
            left: self.left as f32,
        }
    }
}

impl From<Padding> for OnagrePadding {
    fn from(value: Padding) -> Self {
        Self {
            top: value.top as u16,
            right: value.right as u16,
            bottom: value.bottom as u16,
            left: value.left as u16,
        }
    }
}

impl From<u16> for OnagrePadding {
    fn from(value: u16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}
