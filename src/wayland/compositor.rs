// Wayland compositor implementation

use super::protocol::{GlobalEntry, Interface, Message, MessageType};
use super::surface::SurfaceManager;
use super::CompositorState;
use crate::terminal::Screen;
use crate::utils::print_number;

/// Client connection to the compositor
#[derive(Debug, Clone, Copy)]
pub struct Client {
    pub id: u32,
    pub connected: bool,
}

impl Client {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            connected: true,
        }
    }
}

/// Wayland compositor - manages clients, surfaces, and protocol handling
pub struct WaylandCompositor {
    state: CompositorState,
    surface_manager: SurfaceManager,
    clients: [Option<Client>; 8],
    next_client_id: u32,
    globals: [Option<GlobalEntry>; 16],
    global_count: usize,
}

impl WaylandCompositor {
    pub const fn empty() -> Self {
        Self {
            state: CompositorState::Stopped,
            surface_manager: SurfaceManager::empty(),
            clients: [None; 8],
            next_client_id: 1,
            globals: [None; 16],
            global_count: 0,
        }
    }

    pub fn init(&mut self) {
        use crate::drivers::uart::Uart;
        let uart = Uart::new();
        
        // NOTE: These uart.putc calls output "\r" (carriage return) to work around
        // a compiler optimization bug that causes system hangs. The "\r" overwrites
        // itself so no visible output is produced. Do not remove these calls.
        uart.putc(b'\r');
        self.state = CompositorState::Stopped;
        uart.putc(b'\r');
        self.surface_manager.init();
        uart.putc(b'\r');
        // Initialize clients array element by element to avoid potential memcpy issues
        let mut i = 0;
        while i < self.clients.len() {
            self.clients[i] = None;
            i += 1;
        }
        uart.putc(b'\r');
        self.next_client_id = 1;
        uart.putc(b'\r');
        // Note: global_count is already 0 from empty(), no need to set it again
        // Setting it causes a hang due to compiler optimization issues
        
        // Register global interfaces
        self.register_global(Interface::Compositor, 4);
        uart.putc(b'\r');
        self.register_global(Interface::Seat, 7);
        uart.putc(b'\r');
        self.register_global(Interface::Output, 3);
        uart.putc(b'\r');
    }

    pub fn start(&mut self, screen: &mut Screen) {
        if self.state == CompositorState::Running {
            screen.puts("Wayland compositor is already running.\n");
            return;
        }

        self.state = CompositorState::Running;
        screen.puts("=== Wayland Compositor Started ===\n");
        screen.puts("Compositor state: Running\n");
        screen.puts("Listening for client connections...\n");
        screen.puts("\nGlobal interfaces registered:\n");
        
        for i in 0..self.global_count {
            if let Some(global) = self.globals[i] {
                screen.puts("  - ");
                screen.puts(global.interface.name());
                screen.puts(" (version ");
                print_number(screen, global.version as usize);
                screen.puts(")\n");
            }
        }
        
        screen.puts("\nUse 'wayland status' to check compositor status\n");
        screen.puts("Use 'wayland stop' to stop the compositor\n");
    }

    pub fn stop(&mut self, screen: &mut Screen) {
        if self.state == CompositorState::Stopped {
            screen.puts("Wayland compositor is not running.\n");
            return;
        }

        // Disconnect all clients
        for client in &mut self.clients {
            if let Some(c) = client {
                c.connected = false;
            }
        }

        self.state = CompositorState::Stopped;
        screen.puts("Wayland compositor stopped.\n");
    }

    pub fn status(&self, screen: &mut Screen) {
        screen.puts("=== Wayland Compositor Status ===\n");
        
        match self.state {
            CompositorState::Running => screen.puts("State: Running\n"),
            CompositorState::Stopped => screen.puts("State: Stopped\n"),
        }
        
        screen.puts("Connected clients: ");
        let client_count = self.count_clients();
        print_number(screen, client_count);
        screen.puts("\n");
        
        screen.puts("Active surfaces: ");
        let surface_count = self.surface_manager.count_surfaces();
        print_number(screen, surface_count);
        screen.puts("\n");
        
        screen.puts("Registered globals: ");
        print_number(screen, self.global_count);
        screen.puts("\n");
    }

    pub fn is_running(&self) -> bool {
        self.state == CompositorState::Running
    }

    // Client management
    pub fn connect_client(&mut self) -> Option<u32> {
        for slot in &mut self.clients {
            if slot.is_none() {
                let id = self.next_client_id;
                self.next_client_id += 1;
                *slot = Some(Client::new(id));
                return Some(id);
            }
        }
        None
    }

    pub fn disconnect_client(&mut self, client_id: u32) {
        for slot in &mut self.clients {
            if let Some(client) = slot {
                if client.id == client_id {
                    *slot = None;
                    return;
                }
            }
        }
    }

    fn count_clients(&self) -> usize {
        self.clients.iter().filter(|c| c.is_some()).count()
    }

    // Global registry
    fn register_global(&mut self, interface: Interface, version: u32) {
        if self.global_count < self.globals.len() {
            self.globals[self.global_count] = Some(GlobalEntry {
                name: self.global_count as u32,
                interface,
                version,
            });
            self.global_count += 1;
        }
    }

    // Protocol message handling
    pub fn handle_message(&mut self, msg: Message, screen: &mut Screen) {
        match msg.message_type {
            MessageType::DisplaySync => {
                screen.puts("[Wayland] Display sync\n");
            }
            MessageType::DisplayGetRegistry => {
                screen.puts("[Wayland] Get registry\n");
            }
            MessageType::RegistryBind => {
                screen.puts("[Wayland] Bind interface\n");
            }
            MessageType::CompositorCreateSurface => {
                if let Some(surface_id) = self.surface_manager.create_surface() {
                    screen.puts("[Wayland] Created surface ID: ");
                    print_number(screen, surface_id as usize);
                    screen.puts("\n");
                }
            }
            MessageType::SurfaceAttach => {
                if let Some(surface) = self.surface_manager.get_surface_mut(msg.object_id) {
                    surface.attach_buffer();
                    screen.puts("[Wayland] Buffer attached to surface\n");
                }
            }
            MessageType::SurfaceCommit => {
                if let Some(surface) = self.surface_manager.get_surface_mut(msg.object_id) {
                    surface.commit();
                    screen.puts("[Wayland] Surface committed\n");
                }
            }
            MessageType::SurfaceDestroy => {
                if self.surface_manager.destroy_surface(msg.object_id) {
                    screen.puts("[Wayland] Surface destroyed\n");
                }
            }
        }
    }

}
