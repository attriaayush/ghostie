use crate::tui::list::StatefulList;

pub struct TerminalApp<T> {
    pub items: StatefulList<T>,
}

impl<T> TerminalApp<T> {
    pub fn create_list(items: Vec<T>) -> TerminalApp<T> {
        TerminalApp {
            items: StatefulList::with_items(items),
        }
    }
}
