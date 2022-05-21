use crate::app::style::app::AppContainerStyles;
use crate::app::style::rows::RowStyles;
use crate::app::style::scrollable::RowContainerStyle;
use crate::app::style::search::SearchContainerStyles;
use crate::Theme;

// Propagate style from parent to children
// Unless implemented this should never be called
pub trait Inherit {
    fn propagate_background(&mut self) {
        unreachable!()
    }

    fn propagate_font_size(&mut self) {
        unreachable!()
    }

    fn propagate_icon_size(&mut self) {
        unreachable!()
    }

    fn propagate_color(&mut self) {
        unreachable!()
    }
}

impl Inherit for Theme {
    fn propagate_background(&mut self) {
        self.app_container.background = self.background;
        self.app_container.propagate_background();
    }

    fn propagate_font_size(&mut self) {
        let font_size = self.font_size;
        self.app_container.search.input.size = font_size;
        if let Some(hint) = &mut self.app_container.search.plugin_hint {
            hint.font_size = font_size
        }
    }

    fn propagate_icon_size(&mut self) {
        self.app_container.rows.row.icon.size = self.icon_size;
        self.app_container.rows.row_selected.icon.size = self.icon_size;
    }

    fn propagate_color(&mut self) {
        self.app_container.color = self.color;
        self.app_container.propagate_color();
    }
}

impl Inherit for AppContainerStyles {
    fn propagate_background(&mut self) {
        self.rows.background = self.background;
        self.rows.propagate_background();
        self.search.background = self.background;
        self.search.propagate_background();
        self.scrollable.background = self.background;
    }

    fn propagate_color(&mut self) {
        self.rows.color = self.color;
        self.rows.propagate_color();
        self.search.color = self.color;
        self.search.propagate_color();
        self.scrollable.scroller_color = self.color;
    }
}

impl Inherit for RowContainerStyle {
    fn propagate_background(&mut self) {
        self.row.background = self.background;
        self.row.propagate_background();

        self.row_selected.background = self.background;
        self.row_selected.propagate_background();
    }

    fn propagate_color(&mut self) {
        self.row.color = self.color;
        self.row.propagate_color();

        self.row_selected.color = self.color;
        self.row_selected.propagate_color();
    }
}

impl Inherit for RowStyles {
    fn propagate_background(&mut self) {
        self.title.background = self.background;
        self.description.background = self.background;
        self.icon.background = self.background;
    }

    fn propagate_color(&mut self) {
        self.title.color = self.color;
        self.description.color = self.color;
        self.icon.color = self.color;
    }
}

impl Inherit for SearchContainerStyles {
    fn propagate_background(&mut self) {
        let background = self.background;
        self.input.background = background;
        if let Some(hint) = &mut self.plugin_hint {
            hint.background = background;
        }
    }

    fn propagate_color(&mut self) {
        let color = self.color;
        self.input.value_color = color;
        if let Some(hint) = &mut self.plugin_hint {
            hint.color = color;
        }
    }
}
