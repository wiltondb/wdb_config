
use winapi::shared::windef::HWND;
use winapi::um::winuser::SetWindowPos;
use winapi::um::winuser::SWP_NOMOVE;
use winapi::um::winuser::SWP_NOSIZE;

#[derive(Default)]
pub struct TabOrderBuilder {
    control_handles: Vec<nwg::ControlHandle>,
}

impl TabOrderBuilder {
    pub(super) fn new() -> Self {
        Default::default()
    }

    pub fn control<C: Into<nwg::ControlHandle>>(mut self, control: C) -> Self {
        self.control_handles.push(control.into());
        self
    }

    pub fn build(self) {
        let handles: Vec<HWND> = self.control_handles
            .iter()
            .filter_map(|h: &nwg::ControlHandle| h.hwnd())
            .collect();
        for i in 0..handles.len() - 1 {
            unsafe {
                SetWindowPos(
                    handles[i + 1],
                    handles[i],
                    0, 0, 0, 0,
                    SWP_NOSIZE | SWP_NOMOVE);
            }
        }
    }
}
