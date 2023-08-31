
use super::*;
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
        .text("No")
        .parent(&data.window)
        .build(&mut data.choice_no)?;

    Ok(())
}
