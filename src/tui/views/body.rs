use crate::tui::controller::navigation::Navigation;

use super::BorderedWidget;
use left::Left;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, BorderType, Widget},
};
use right::Right;

mod left;
mod right;

pub struct Body<'a> {
    pub eth_price: &'a Option<String>,
    pub navigation: &'a Navigation<'a>,
}

impl Widget for Body<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let horizontal_layout =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]);
        let [left_area, right_area] = horizontal_layout.areas(area);
        Left {
            page: self.navigation.current_page(),
            text_input: self.navigation.text_input.clone(),
            _marker: std::marker::PhantomData,
        }
        .render_with_block(
            left_area,
            buf,
            Block::bordered().border_type(BorderType::Plain),
        );
        Right {
            eth_price: self.eth_price,
        }
        .render_with_block(
            right_area,
            buf,
            Block::bordered().border_type(BorderType::Plain),
        );
    }
}
