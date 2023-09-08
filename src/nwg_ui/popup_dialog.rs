
use std::thread::JoinHandle;

use super::*;

pub trait PopupDialog<A: PopupArgs, R> {
    fn popup(args: A) -> JoinHandle<R>;

    fn init(&mut self);

    fn result(&mut self) -> R;

    fn close(&mut self);
}
