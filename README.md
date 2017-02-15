A port of Robert Penner's easing equations to rust

Usage:

```rs
extern crate pennereq;

use pennereq::*;

fn main(){
    quad::ease_in(t, from, distance, duration);
}
```

The easing functions can work with f32 or f64 resolution depending on the passed parameters
