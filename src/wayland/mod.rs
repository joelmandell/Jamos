// Wayland compositor module for Jamos
// Implements a minimal Wayland compositor that can be started from the terminal

mod compositor;
mod protocol;
mod surface;

pub use compositor::WaylandCompositor;
pub use surface::Surface;

// Wayland compositor state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompositorState {
    Stopped,
    Running,
}
