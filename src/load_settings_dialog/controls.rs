
use crate::*;
use nwg_ui as ui;

#[derive(Default)]
pub(super) struct LoadSettingsDialogControls {
    pub(super) font_normal: nwg::Font,

    pub(super) window: nwg::Window,
    pub(super) progress_bar: nwg::ProgressBar,
    pub(super) label: nwg::Label,
    pub(super) details_box: nwg::TextBox,
    pub(super) copy_clipboard_button: nwg::Button,
    pub(super) close_button: nwg::Button,

    pub(super) load_notice: ui::SyncNotice,
}

impl ui::Controls for LoadSettingsDialogControls {
    fn build(&mut self) -> Result<(), nwg::NwgError> {
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .normal()
                .build())
            .build(&mut self.font_normal)?;

        nwg::Window::builder()
            .size((320, 200))
            .center(true)
            .title("Check")
            .build(&mut self.window)?;

        nwg::ProgressBar::builder()
            .flags(nwg::ProgressBarFlags::VISIBLE | nwg::ProgressBarFlags::MARQUEE)
            .marquee(true)
            .marquee_update(30)
            .range(0..1)
            .parent(&self.window)
            .build(&mut self.progress_bar)?;

        nwg::Label::builder()
            .text("Loading ...")
            .flags(nwg::LabelFlags::VISIBLE | nwg::LabelFlags::ELIPSIS)
            .font(Some(&self.font_normal))
            .v_align(nwg::VTextAlign::Top)
            .parent(&self.window)
            .build(&mut self.label)?;

        nwg::TextBox::builder()
            .text("")
            .font(Some(&self.font_normal))
            .readonly(true)
            .parent(&self.window)
            .build(&mut self.details_box)?;

        nwg::Button::builder()
            .text("Copy to clipboard")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.copy_clipboard_button)?;

        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;

        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.load_notice)?;

        Ok(())
    }

    fn window(&self) -> &nwg::Window {
        &self.window
    }
}
