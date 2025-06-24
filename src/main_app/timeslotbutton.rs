use crate::resource::icon::Icon;

pub enum SelectButtonState {
    Selected,
    Unselected,
}

struct SelectSlotButton {
    selected: [Icon; 2],
    unselected: [Icon; 2],
    state: SelectButtonState,
}
