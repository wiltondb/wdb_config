
use super::*;

pub trait PopupDialog<A: PopupArgs, R> {
    fn popup(args: A) -> PopupJoinHandle<R>;

    fn init(&mut self);

    fn result(&mut self) -> R;

    fn close(&mut self, _: nwg::EventData);

    fn on_resize(&mut self, _: nwg::EventData);
}
