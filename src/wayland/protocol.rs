// Wayland protocol structures and message handling
// Based on the Wayland protocol specification

/// Wayland object ID
pub type ObjectId = u32;

/// Wayland protocol message types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    DisplaySync,
    DisplayGetRegistry,
    RegistryBind,
    CompositorCreateSurface,
    SurfaceAttach,
    SurfaceCommit,
    SurfaceDestroy,
}

/// Wayland protocol message
#[derive(Debug, Clone, Copy)]
pub struct Message {
    pub object_id: ObjectId,
    pub opcode: u16,
    pub message_type: MessageType,
}

impl Message {
    pub fn new(object_id: ObjectId, opcode: u16, message_type: MessageType) -> Self {
        Self {
            object_id,
            opcode,
            message_type,
        }
    }
}

/// Wayland global registry entry
#[derive(Debug, Clone, Copy)]
pub struct GlobalEntry {
    pub name: u32,
    pub interface: Interface,
    pub version: u32,
}

/// Wayland interface types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interface {
    Display,
    Registry,
    Compositor,
    Surface,
    Seat,
    Output,
}

impl Interface {
    pub fn name(&self) -> &'static str {
        match self {
            Interface::Display => "wl_display",
            Interface::Registry => "wl_registry",
            Interface::Compositor => "wl_compositor",
            Interface::Surface => "wl_surface",
            Interface::Seat => "wl_seat",
            Interface::Output => "wl_output",
        }
    }
}
