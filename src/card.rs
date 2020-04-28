use druid::{
    widget::{Flex, Label, WidgetExt},
    Data, Lens, Widget,
};

#[derive(Clone, Debug, Data, Lens)]
pub struct Card {
    pub title: String,
    pub text: String,
}

impl Card {
    pub fn new(title: impl Into<String>, text: impl Into<String>) -> Card {
        Card {
            title: title.into(),
            text: text.into(),
        }
    }

    pub fn widget() -> impl Widget<Card> {
        let title = Label::new(|s: &String, _env: &_| s.to_string()).lens(Card::title);
        let text = Label::new(|s: &String, _env: &_| s.to_string()).lens(Card::text);

        Flex::column().with_child(title).with_child(text)
    }
}
