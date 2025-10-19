// Common utility functions

use crate::terminal::Screen;

/// Print a number to the screen by converting to decimal digits
pub fn print_number(screen: &mut Screen, n: usize) {
    let mut buf = [0u8; 20];
    let mut num = n;
    let mut len = 0;
    
    if num == 0 {
        buf[0] = b'0';
        len = 1;
    } else {
        while num > 0 {
            buf[len] = b'0' + (num % 10) as u8;
            num /= 10;
            len += 1;
        }
    }
    
    // Print in reverse
    for i in (0..len).rev() {
        screen.putc(buf[i]);
    }
}
