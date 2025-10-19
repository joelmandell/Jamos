// Tiling window manager for virtual desktops
use super::screen::Screen;
use crate::drivers::uart::Uart;

const MAX_PANES: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileLayout {
    Single,      // One full pane
    Vertical,    // Two panes side by side
    Horizontal,  // Two panes stacked
    Quad,        // Four panes in 2x2 grid
}

#[derive(Clone, Copy)]
pub struct Pane {
    screen: Screen,
    is_active: bool,
    pane_id: usize,
}

impl Pane {
    pub const fn empty() -> Self {
        Pane {
            screen: Screen::empty(),
            is_active: false,
            pane_id: 0,
        }
    }

    pub fn new(uart: Uart, pane_id: usize) -> Self {
        Pane {
            screen: Screen::new(uart),
            is_active: true,
            pane_id,
        }
    }

    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }
}

pub struct TilingManager {
    panes: [Pane; MAX_PANES],
    active_pane: usize,
    layout: TileLayout,
    pane_count: usize,
}

impl TilingManager {
    pub const fn empty() -> Self {
        TilingManager {
            panes: [Pane::empty(); MAX_PANES],
            active_pane: 0,
            layout: TileLayout::Single,
            pane_count: 0,
        }
    }

    pub fn init(&mut self, _uart: Uart) {
        // For now, just initialize the counts without setting up panes
        // to avoid initialization hang issues
        self.pane_count = 1;
        self.active_pane = 0;
        self.layout = TileLayout::Single;
    }

    pub fn split_vertical(&mut self, uart: Uart) -> bool {
        if self.pane_count >= MAX_PANES {
            return false;
        }

        match self.layout {
            TileLayout::Single => {
                self.panes[1] = Pane::new(uart, 1);
                self.pane_count = 2;
                self.layout = TileLayout::Vertical;
                true
            }
            _ => false, // Already split
        }
    }

    pub fn split_horizontal(&mut self, uart: Uart) -> bool {
        if self.pane_count >= MAX_PANES {
            return false;
        }

        match self.layout {
            TileLayout::Single => {
                self.panes[1] = Pane::new(uart, 1);
                self.pane_count = 2;
                self.layout = TileLayout::Horizontal;
                true
            }
            _ => false, // Already split
        }
    }

    pub fn next_pane(&mut self) -> bool {
        if self.pane_count <= 1 {
            return false;
        }

        self.active_pane = (self.active_pane + 1) % self.pane_count;
        true
    }

    pub fn prev_pane(&mut self) -> bool {
        if self.pane_count <= 1 {
            return false;
        }

        if self.active_pane == 0 {
            self.active_pane = self.pane_count - 1;
        } else {
            self.active_pane -= 1;
        }
        true
    }

    pub fn current_pane_mut(&mut self) -> Option<&mut Pane> {
        if self.active_pane < self.pane_count && self.panes[self.active_pane].is_active {
            Some(&mut self.panes[self.active_pane])
        } else {
            None
        }
    }

    pub fn get_layout(&self) -> TileLayout {
        self.layout
    }

    pub fn get_active_pane(&self) -> usize {
        self.active_pane
    }

    pub fn get_pane_count(&self) -> usize {
        self.pane_count
    }
}
