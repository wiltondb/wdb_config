
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use nwg::stretch::style::AlignSelf;
use nwg::stretch::style::FlexDirection;

use crate::*;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use about_dialog::AboutDialog;
use nwg::Window;

#[derive(Default)]
pub struct AboutDialogUi {
    events: events::Events<AboutDialog>,

    font_normal: nwg::Font,

    window: nwg::Window,
    label: nwg::Label,
    close_button: nwg::Button,

    root_layout: nwg::FlexboxLayout,
}

impl DialogUi for AboutDialogUi {
    fn window(&self) -> &Window {
        &self.window
    }

    fn build_controls(&mut self) -> Result<(), nwg::NwgError> {
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .normal()
                .build())
            .build(&mut self.font_normal)?;

        nwg::Window::builder()
            .size((320, 200))
            .center(true)
            .title("About")
            .build(&mut self.window)?;
        events::builder()
            .control(&self.window)
            .event(nwg::Event::OnWindowClose)
            .handler(AboutDialog::close)
            .build(&mut self.events)?;

        nwg::Label::builder()
            .text("Very long label label label label label label label label \r\n will eventually go here")
            .h_align(nwg::HTextAlign::Center)
            .v_align(nwg::VTextAlign::Top)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.label)?;

        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;
        events::builder()
            .control(&self.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AboutDialog::close)
            .build(&mut self.events)?;

        Ok(())
    }

    fn build_layout(&mut self) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Column)

            .child(&self.label)
            .child_size(ui::size_builder()
                .width_auto()
                .height_pt(50)
                .build())
            .child_flex_grow(1.0)

            .child(&self.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_align_self(AlignSelf::FlexEnd)

            .build(&mut self.root_layout)?;

        Ok(())
    }
    
}

pub struct AboutDialogNwg {
    inner: Rc<AboutDialog>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<AboutDialogNwg> for AboutDialog {
    fn build_ui(mut data: AboutDialog) -> Result<AboutDialogNwg, nwg::NwgError> {
        data.build_popup_ui()?;

        let wrapper = AboutDialogNwg {
            inner:  Rc::new(data),
            default_handler: Default::default(),
        };

        let data_ref = Rc::downgrade(&wrapper.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_data) = data_ref.upgrade() {
                for eh in evt_data.ui().events.iter() {
                    if handle == eh.control_handle && evt == eh.event {
                        (eh.handler)(&evt_data);
                        break;
                    }
                }
            }
        };

        *wrapper.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&wrapper.ui().window.handle, handle_events));

        return Ok(wrapper);
    }
}

impl Drop for AboutDialogNwg {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for AboutDialogNwg {
    type Target = AboutDialog;

    fn deref(&self) -> &AboutDialog {
        &self.inner
    }
}
