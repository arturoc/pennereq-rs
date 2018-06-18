extern crate num_traits;

pub mod quad{
    use num_traits::{Float,one};

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d;
        c * t * t + b
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d;
        -c * t * (t - From::from(2.)) + b
    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let one = one::<T>();
        let two = From::from(2.);
        let t = t / (d / two);
        if t < one{
            c / two * t * t + b
        }else{
            let t = t - one;
            -c / two * (t * (t - two) - one) + b
        }
    }
}

pub mod cubic{
    use num_traits::{Float,one};

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d;
        c * t * t * t + b
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d - one();
        c * (t * t * t + one()) + b
    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let one = one::<T>();
        let two = From::from(2.);
        let t = t / (d / two);
        if t < one{
            c / two * t * t * t + b
        }else{
            let t = t - two;
            c / two * (t * t  * t + two) + b
        }
    }
}

pub mod quart{
    use num_traits::{Float,one};

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d;
        c * t * t * t * t + b
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d - one();
        -c * (t * t * t * t - one()) + b
    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let one = one::<T>();
        let two = From::from(2.);
        let t = t / (d / two);
        if t < one{
            c / two * t * t * t * t + b
        }else{
            let t = t - two;
            -c / two * (t * t  * t * t - two) + b
        }
    }
}

pub mod quint{
    use num_traits::{Float,one};

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d;
        c * t * t * t * t * t + b
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d - one();
        c * (t * t * t * t * t + one()) + b
    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let one = one::<T>();
        let two = From::from(2.);
        let t = t / (d / two);
        if t < one{
            c / two * t * t * t * t * t + b
        }else{
            let t = t - two;
            c / two * (t * t  * t * t * t + two) + b
        }
    }
}

pub mod sine{
    use num_traits::{Float,one};
    use std::f32::consts::PI;

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let pi: T = From::from(PI);
        let two: T = From::from(2.);
        -c * (t / d * pi / two).cos() + c + b
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let pi: T = From::from(PI);
        let two: T = From::from(2.);
        c * (t / d * pi / two).sin() + b
    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let pi: T = From::from(PI);
        let two: T = From::from(2.);
        -c / two * ((pi * t / d).cos() - one()) + b
    }
}

pub mod expo{
    use num_traits::{Float,one,zero};

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let two: T = From::from(2.);
        let ten: T = From::from(10.);
        if t == zero() {
            b
        }else{
            c * two.powf(ten * (t / d - one())) + b
        }
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let two: T = From::from(2.);
        let ten: T = From::from(10.);
        if t == d{
            b + c
        }else{
            c * (-two.powf(-ten * t / d) + one()) + b
        }
    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let two: T = From::from(2.);
        let ten: T = From::from(10.);
        if t == zero() { b }
        else if t == d { b + c }
        else{
            let t = t / (d / two);
            if t < one(){
                c / two * two.powf(ten * (t - one())) + b
            }else{
                c / two * (-two.powf(-ten * (t - one())) + two) + b
            }
        }
    }
}

pub mod circ{
    use num_traits::{Float,one};

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d;
        -c * ((one::<T>() - t * t).sqrt() - one()) + b
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d - one();
        c * (one::<T>() - t * t).sqrt() + b
    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let one = one::<T>();
        let two = From::from(2.);
        let t = t / (d / two);
        if t < one{
            -c / two * ((one - t * t).sqrt() - one) + b
        }else{
            let t = t - two;
            c / two * ((one - t * t).sqrt() + one) + b
        }
    }
}

pub mod elastic {
    use num_traits::{Float,one,zero};
    use std::f32::consts::PI;

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        if t == zero() { return b };

        let pi: T = From::from(PI);
        let two: T = From::from(2.);
        let two_pi: T = two * pi;
        let ten: T = From::from(10.);
        let a: T = c;
        let t = t / d;

        if t == one() {
            b + c
        } else {
            let p = d * From::from(0.3);
            let s = if a < c.abs() {
                p / From::from(4.)
            } else {
                p / two_pi * (c / a).asin()
            };

            let t = t - one();
            -(a * two.powf(ten * t)) * ((t * d - s) * two_pi / p).sin() + b
        }
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        if t == zero() { return b };

        let a: T = c;
        let two: T = From::from(2.);
        let ten: T = From::from(10.);
        let pi: T = From::from(PI);
        let two_pi: T = two * pi;
        let t = t / d;

        if t == one() {
            b + c
        } else {
            let p = d * From::from(0.3);
            let s = if a < c.abs() {
                p / From::from(4.)
            } else {
                p / two_pi * (c / a).asin()
            };

            a * two.powf(-ten * t) * ((t * d - s) * two_pi / p).sin() + c + b
        }

    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        if t == zero() { return b };

        let a: T = c;
        let two: T = From::from(2.);
        let ten: T = From::from(10.);
        let pi: T = From::from(PI);
        let two_pi: T = two * pi;
        let half: T = From::from(0.5);
        let t = t / (d / two);

        if t == two {
            b + c
        } else {
            let p = d * From::from(0.3) * From::from(1.5);
            let s = if a < c.abs() {
                p / From::from(4.)
            } else {
                p / two_pi * (c / a).asin()
            };
            if t < one() {
                let t = t - one();
                -half * ( a * two.powf(ten * t)) * ((t * d - s) * two_pi / p).sin() + b
            }else{
                let t = t - one();
                a * two.powf(-ten * t) * ((t * d - s) * two_pi / p).sin() * half + c + b
            }
        }
    }
}

pub mod back{
    use num_traits::{Float,one};

    static DEFAULT_S: f32 = 1.70158;

    #[inline]
    fn default_s<T: From<f32>>() -> T {
        From::from(DEFAULT_S)
    }

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        ease_in_s(t, b, c, d, default_s())
    }

    #[inline]
    pub fn ease_in_s<T:Float>(t: T, b: T, c: T, d: T, s: T) -> T {
        let t = t / d;
        c * t * t * ((s + one()) * t - s) + b
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        ease_out_s(t, b, c, d, default_s())
    }

    #[inline]
    pub fn ease_out_s<T:Float>(t: T, b: T, c: T, d: T, s: T) -> T {
        let t = t / d - one();
        c * (t * t * ((s + one()) * t + s) + one()) + b
    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        ease_in_out_s(t, b, c, d, default_s())
    }

    #[inline]
    pub fn ease_in_out_s<T:Float + From<f32>>(t: T, b: T, c: T, d: T, s: T) -> T {
        let s = s * From::from(1.525);
        let two = From::from(2.);
        let t = t / ( d / two );
        if t < one(){
            c / two * (t * t * ((s + one()) * t - s )) + b
        }else{
            let t = t - two;
            c / two * (t * t * ((s + one()) * t + s) + two) + b
        }
    }
}

pub mod bounce{
    use num_traits::{Float,one,zero};

    #[inline]
    pub fn ease_in<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        c - ease_out(d-t, zero(), c, d) + b
    }

    #[inline]
    pub fn ease_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        let t = t / d;
        let one: T = one();
        let two: T = From::from(2.);
        let _1_5: T = From::from(1.5);
        let _2_5: T = From::from(2.5);
        let _2_25: T = From::from(2.25);
        let _2_625: T = From::from(2.625);
        let _2_75: T = From::from(2.75);
        let _7_5625: T = From::from(7.5625);
        if t < one / _2_75 {
            c * (_7_5625 * t * t) +b
        }else if t < two / _2_75 {
            let t = t - _1_5 / _2_75;
            c * (_7_5625 * t * t + From::from(0.75)) + b
        }else if t < _2_5 / _2_75{
            let t = t - _2_25 / _2_75;
            c * (_7_5625 * t * t + From::from(0.9375)) + b
        }else{
            let t = t - _2_625 / _2_75;
            c * (_7_5625 * t * t + From::from(0.984375)) + b
        }
    }

    #[inline]
    pub fn ease_in_out<T:Float + From<f32>>(t: T, b: T, c: T, d: T) -> T {
        if t < d / From::from(2.){
            ease_in(t * From::from(2.), zero(), c, d) * From::from(0.5) + b
        }else{
            ease_out(t * From::from(2.) - d, zero(), c, d)  * From::from(0.5) + c  * From::from(0.5) + b
        }
    }
}
