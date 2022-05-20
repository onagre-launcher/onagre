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
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            left: self.left,
        }
    }
}

impl From<Padding> for OnagrePadding {
    fn from(value: Padding) -> Self {
        Self {
            top: value.top,
            right: value.right,
            bottom: value.bottom,
            left: value.left,
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
