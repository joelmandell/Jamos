#include "wayland/compositor.hpp"

void WaylandCompositor::start(Screen* screen) {
    if (running) {
        screen->puts("Wayland compositor is already running.\n");
    } else {
        running = true;
        screen->puts("Wayland compositor started.\n");
    }
}

void WaylandCompositor::stop(Screen* screen) {
    if (!running) {
        screen->puts("Wayland compositor is not running.\n");
    } else {
        running = false;
        screen->puts("Wayland compositor stopped.\n");
    }
}

void WaylandCompositor::status(Screen* screen) {
    screen->puts("Wayland Compositor Status: ");
    if (running) {
        screen->puts("Running\n");
    } else {
        screen->puts("Stopped\n");
    }
}
