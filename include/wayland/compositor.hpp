#ifndef WAYLAND_COMPOSITOR_HPP
#define WAYLAND_COMPOSITOR_HPP

#include "terminal/screen.hpp"

class WaylandCompositor {
private:
    bool running;
    
public:
    WaylandCompositor() : running(false) {}
    
    void init() {}
    void start(Screen* screen);
    void stop(Screen* screen);
    void status(Screen* screen);
};

#endif // WAYLAND_COMPOSITOR_HPP
