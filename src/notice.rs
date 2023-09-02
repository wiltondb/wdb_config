
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;

// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow
// A window must be enabled before it can be activated. For example, if an application
// is displaying a modeless dialog box and has disabled its main window, the application
// must enable the main window before destroying the dialog box. Otherwise, another window
// will receive the keyboard focus and be activated.

pub type SyncNotice = SyncNoticeValue<()>;
pub type SyncNoticeSender = SyncNoticeValueSender<()>;

pub struct SyncNoticeValueSender<T: Sync + Send + Default> {
    sender: nwg::NoticeSender,
    tx: SyncSender<T>,
}

impl<T: Sync + Send + Default> SyncNoticeValueSender<T> {
    pub fn send(&self) {
        self.sender.notice();
        self.tx.send(Default::default()).expect("Notice send failure");
    }

    pub fn send_result(&self, t: T) {
        self.sender.notice();
        self.tx.send(t).expect("Notice send failure");
    }
}

#[derive(Default)]
pub struct SyncNoticeValue<T: Sync + Send + Default> {
    pub notice: nwg::Notice,
    tx: Option<SyncSender<T>>,
    rx: Option<Receiver<T>>,
}

impl<T: Sync + Send + Default> SyncNoticeValue<T> {

    pub fn sender(&self) -> SyncNoticeValueSender<T> {
        SyncNoticeValueSender {
            sender: self.notice.sender(),
            tx: self.tx.as_ref().expect("Notice not initialized").clone(),
        }
    }

    pub fn receive(&self) -> T {
        self.rx.as_ref().expect("Notice not initalized")
            .recv().expect("Notice receive failure")
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

    pub fn build<T: Sync + Send + Default>(self, out: &mut SyncNoticeValue<T>) -> Result<(), nwg::NwgError> {
        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(nwg::NwgError::no_parent("Notice"))
        }?;

        nwg::Notice::builder()
            .parent(&parent)
            .build(&mut out.notice)?;

        let (tx, rx) = sync_channel::<T>(0);
        out.tx = Some(tx);
        out.rx = Some(rx);

        Ok(())
    }

}

pub fn builder() -> SyncNoticeBuilder {
    SyncNoticeBuilder::new()
}


