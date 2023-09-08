
use std::thread::JoinHandle;

use super::*;

pub trait PopupDialog<A: PopupArgs, R> {
    fn popup(args: A) -> JoinHandle<R>;

    fn init(&self);

    fn result(&self) -> R;

    fn close(&self);
}
