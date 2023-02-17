# Dynamic and Static Libs

!
=

Let’s try to use Rust from C.

Library
=======
```rust,ignore
    #[derive(Debug)]
    #[repr(C)]
    pub struct Point {
        x: i32,
        y: i32
    }

    #[no_mangle]
    pub extern "C" fn new_point(x: i32, y: i32) -> *mut Point {
        let p = Box::new(Point { x: x, y: y });
        Box::into_raw(p)
    }

    #[no_mangle]
    pub extern "C" fn destroy_point(p: *mut Point) {
        unsafe { Box::from_raw(p) };
    }

    #[no_mangle]
    pub extern "C" fn inspect_point(p: *mut Point) {
        unsafe {
            let point: Box<Point> = Box::from_raw(p);
            point.inspect();
        };
    }
```
C-Header (excerpt)
==================
```c
    #include <stdint.h>
    #include <stdbool.h>

    typedef struct Point {
        int32_t x;
        int32_t y;
    } Point;

    Point* new_point(int32_t x, int32_t y);

    void destroy_point(Point* p);

    void inspect(Point* p);
```
Cargo
=====
```toml
    [lib]
    crate-type = ["cdylib", "staticlib"]
```
`cargo build` will now build a static lib instead of an rlib.
`cdylib`s are a special kind of dylib that also removes all
Rust-specific metadata.

Usage
=====
```c
    #include "../include/point.h"

    int main (int argc, char const *argv[])
    {
            Point* p = new_point(1,1);
            inspect_point(p);
            p->x = 2;
            inspect_point(p);
            destroy_point(p);
    }
```

```console
    gcc -L target/debug/ -I include -lc -lm -lSystem -lcore test/test.c -o point
```

Execution
=========
```console
    $ ./point
    Point { x: 1, y: 1 }
    Point { x: 2, y: 1 }
    point(98132,0x7fffb30293c0) malloc: *** error for object 0x7fa635c02980: pointer being freed was not allocated
    *** set a breakpoint in malloc_error_break to debug
    Abort trap: 6
```

Woops!
======

Take good care of ownership!
```rust,ignore
    #[no_mangle]
    pub extern "C" fn inspect_point(p: *mut Point) {
        unsafe {
            let point: Box<Point> = Box::from_raw(p);
            point.inspect();
            std::mem::forget(point)
        };
    }

    #[no_mangle]
    pub extern "C" fn inspect_point(p: &Point) {
        p.inspect();
    }
```

Helpers
=======

-   Cheddar - generates C-Headers from Rust-Libs. TODO: isn't this `cbindgen` now?
-   `bindgen` - generates Rust bindings from C headers
