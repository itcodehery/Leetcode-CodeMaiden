use std::cell::RefCell;
use std::rc::Rc;

pub struct WindowPane {
    pub id: u32,
    pub title: String,
    pub is_focused: bool,
}

pub struct WindowManager {
    pub panes: Vec<Rc<RefCell<WindowPane>>>,
    pub active_pane: Option<Rc<RefCell<WindowPane>>>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            panes: Vec::new(),
            active_pane: None,
        }
    }

    // TODO 1: Add a new pane to the layout
    pub fn add_pane(&mut self, id: u32, title: &str) {
        let new_pane = WindowPane {
            id,
            title: title.to_string(),
            is_focused: false,
        };

        // Wrap the raw struct in our smart pointers
        let wrapped_pane = Rc::new(RefCell::new(new_pane));
        self.panes.push(wrapped_pane);
    }

    // TODO 2: Focus a pane by its index
    pub fn focus_pane_at_index(&mut self, index: usize) {
        // 1. Unfocus the currently active pane, if there is one
        if let Some(current_active) = &self.active_pane {
            current_active.borrow_mut().is_focused = false;
        }

        // 2 & 3. Get the new pane from the layout and clone the Rc pointer
        let next_active = self.panes[index].clone();

        // 4. Mutate the new pane to be focused
        next_active.borrow_mut().is_focused = true;

        // Assign shared ownership to active_pane
        self.active_pane = Some(next_active);
    }

    // TODO 3: Rename the currently active pane
    pub fn rename_active_pane(&self, new_title: &str) {
        // Peel the Option to access the pointer
        if let Some(active) = &self.active_pane {
            // Borrow the RefCell mutably to change the title
            active.borrow_mut().title = new_title.to_string();
        }
    }
}
