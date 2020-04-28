mod card;
use card::Card;
fn main() {
    let data = Card::new("Title", "here's some nice text for you");

    let window =
        druid::WindowDesc::new(Card::widget).title(druid::LocalizedString::new("Card Widget"));

    druid::AppLauncher::with_window(window)
        .launch(data)
        .unwrap();
}
