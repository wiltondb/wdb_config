
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;

// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow
// A window must be enabled before it can be activated. For example, if an application
// is displaying a modeless dialog box and has disabled its main window, the application
// must enable the main window before destroying the dialog box. Otherwise, another window
// will receive the keyboard focus and be activated.

#[derive(Default)]
pub struct SyncNoticeSender {
    sender: Option<nwg::NoticeSender>,
    tx: Option<SyncSender<()>>,
}

impl SyncNoticeSender {
    pub fn send(&self) {
        self.sender.as_ref().expect("Sender not initialized").notice();
        self.tx.as_ref().expect("Sender not initialized").send(()).expect("Notice send failure");
    }
}

#[derive(Default)]
pub struct SyncNotice {
    pub notice: nwg::Notice,
    tx: Option<SyncSender<()>>,
    rx: Option<Receiver<()>>,
}

impl SyncNotice {

    pub fn sender(&self) -> SyncNoticeSender {
        SyncNoticeSender {
            sender: Some(self.notice.sender()),
            tx: Some(self.tx.as_ref().expect("Notice not initialized").clone())
        }
    }

    pub fn receive(&self) {
        self.rx.as_ref().expect("Notice not initalized")
            .recv().expect("Notice receive failure");
    }
}

pub struct SyncNoticeBuilder {
    parent: Option<nwg::ControlHandle>,
}

impl SyncNoticeBuilder {
    fn new() -> Self {
        Self {
            parent: None,
        }
    }

    pub fn parent<C: Into<nwg::ControlHandle>>(mut self, p: C) -> Self {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut SyncNotice) -> Result<(), nwg::NwgError> {
        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(nwg::NwgError::no_parent("Notice"))
        }?;

        nwg::Notice::builder()
            .parent(&parent)
            .build(&mut out.notice)?;

        let (tx, rx) = sync_channel::<()>(0);
        out.tx = Some(tx);
        out.rx = Some(rx);

        Ok(())
    }

}

pub fn builder() -> SyncNoticeBuilder {
    SyncNoticeBuilder::new()
}


