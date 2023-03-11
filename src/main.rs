
use druid::text::format::ParseFormatter;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use std::thread;
use druid::widget::{Button, Flex, Label, TextBox, ValueTextBox};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, };


#[tokio::main]
pub async fn second() {
    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);
        }
    });

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    client.join("ultia".to_owned()).unwrap();

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}


fn main() -> Result<(), PlatformError> {
    // let _handle = thread::spawn(||{
    //     second()
    // });
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    let res = AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data);

    res
}


fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);
    let formatter = ParseFormatter::new();
    let _box = TextBox::new().with_formatter(formatter);
    Flex::column().with_child(label).with_child(button).with_child(_box)
}
