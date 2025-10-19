// Wayland surface management

use super::protocol::ObjectId;

/// Default surface dimensions
const DEFAULT_SURFACE_WIDTH: u32 = 800;
const DEFAULT_SURFACE_HEIGHT: u32 = 600;

/// Starting ID for surface objects
const SURFACE_ID_START: ObjectId = 1000;

/// Wayland surface - represents a rectangular area that can be rendered
#[derive(Debug, Clone, Copy)]
pub struct Surface {
    pub id: ObjectId,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
    pub buffer_attached: bool,
}

impl Surface {
    pub fn new(id: ObjectId) -> Self {
        Self {
            id,
            x: 0,
            y: 0,
            width: DEFAULT_SURFACE_WIDTH,
            height: DEFAULT_SURFACE_HEIGHT,
            visible: false,
            buffer_attached: false,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn attach_buffer(&mut self) {
        self.buffer_attached = true;
    }

    pub fn commit(&mut self) {
        self.visible = true;
    }

    pub fn destroy(&mut self) {
        self.visible = false;
        self.buffer_attached = false;
    }
}

/// Surface manager - manages all surfaces
pub struct SurfaceManager {
    surfaces: [Option<Surface>; 32],
    next_id: ObjectId,
}

impl SurfaceManager {
    pub const fn empty() -> Self {
        Self {
            surfaces: [None; 32],
            next_id: SURFACE_ID_START,
        }
    }

    pub fn init(&mut self) {
        // Initialize surfaces array element by element to avoid potential memcpy issues
        for i in 0..self.surfaces.len() {
            self.surfaces[i] = None;
        }
        self.next_id = SURFACE_ID_START;
    }

    pub fn create_surface(&mut self) -> Option<ObjectId> {
        for slot in &mut self.surfaces {
            if slot.is_none() {
                let id = self.next_id;
                self.next_id += 1;
                *slot = Some(Surface::new(id));
                return Some(id);
            }
        }
        None
    }

    pub fn get_surface(&self, id: ObjectId) -> Option<&Surface> {
        self.surfaces.iter()
            .filter_map(|s| s.as_ref())
            .find(|s| s.id == id)
    }

    pub fn get_surface_mut(&mut self, id: ObjectId) -> Option<&mut Surface> {
        self.surfaces.iter_mut()
            .filter_map(|s| s.as_mut())
            .find(|s| s.id == id)
    }

    pub fn destroy_surface(&mut self, id: ObjectId) -> bool {
        for slot in &mut self.surfaces {
            if let Some(surface) = slot {
                if surface.id == id {
                    *slot = None;
                    return true;
                }
            }
        }
        false
    }

    pub fn count_surfaces(&self) -> usize {
        self.surfaces.iter().filter(|s| s.is_some()).count()
    }
}
