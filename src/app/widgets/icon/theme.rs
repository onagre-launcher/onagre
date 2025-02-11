use iced::widget::svg;

impl svg::Catalog for crate::Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {
        
    }

    fn style(&self, class: &Self::Class<'_>, status: svg::Status) -> svg::Style {
        svg::Style { color: None }
    }
}
