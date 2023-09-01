
use crate::*;
use about_dialog::AboutDialog;

pub fn build(data: &mut AboutDialog) -> Result<(), nwg::NwgError> {

    nwg::Window::builder()
        .size((320, 200))
        .center(true)
        .title("About")
        .build(&mut data.window)?;
    events::builder()
        .control(&data.window)
        .event(nwg::Event::OnWindowClose)
        .handler(AboutDialog::close)
        .build(&mut data.events)?;

    nwg::Button::builder()
        .text("Yes")
        .parent(&data.window)
        .build(&mut data.choice_yes)?;

    nwg::Button::builder()
        .text("Connect")
        .parent(&data.window)
        .build(&mut data.connect_button)?;
    events::builder()
        .control(&data.connect_button)
        .event(nwg::Event::OnButtonClick)
        .handler(AboutDialog::connect)
        .build(&mut data.events)?;

    Ok(())
}
