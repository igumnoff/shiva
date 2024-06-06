// Copyright 2013 The color-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num_traits::{self, NumCast, Num};
use angle::*;

use {Color, FloatColor};
use {Channel, FloatChannel};
use {Rgb, ToRgb};
use alpha::{ToRgba, Rgba};
use color_space::{Srgb, TransferFunction};
use std::marker::PhantomData;

#[inline]
fn cast<T: num_traits::NumCast, U: num_traits::NumCast>(n: T) -> U {
    num_traits::cast(n).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hsv<T = f32, S = Srgb> { pub h: Deg<T>, pub s: T, pub v: T, pub standard: PhantomData<S> }

impl<T: Clone,S> Clone for Hsv<T, S>{
    fn clone(&self) -> Hsv<T, S>{
        Hsv{ h: self.h.clone(), s: self.s.clone(), v: self.v.clone(), standard: PhantomData }
    }
}

impl<T: Copy, S> Copy for Hsv<T, S>{}

impl<N: Clone + PartialEq + Num + NumCast, S> PartialEq for Hsv<N, S>{
	#[inline]
	fn eq(&self, other: &Hsv<N, S>) -> bool{
		self.h.clone().wrap().eq(&other.h.clone().wrap()) && self.s.eq(&other.s) && self.v.eq(&other.v)
	}
}

impl<N: Clone + PartialEq + Eq + Num + NumCast, S> Eq for Hsv<N, S>{}

impl<T, S> Hsv<T, S> {
    pub const fn new(h: Deg<T>, s: T, v: T) -> Hsv<T, S> {
        Hsv { h: h, s: s, v: v, standard: PhantomData }
    }
}

impl<T: Channel + NumCast + Num, S: TransferFunction> Color<T> for Hsv<T, S> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    fn clamp_s(self, lo: T, hi: T) -> Hsv<T, S> {
        Hsv::new(self.h, // Should the hue component be clamped?
                 self.s.clamp(lo, hi),
                 self.v.clamp(lo, hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    fn clamp_c(self, lo: Hsv<T, S>, hi: Hsv<T, S>) -> Hsv<T, S> {
        Hsv::new(self.h,
                 self.s.clamp(lo.s, hi.s),
                 self.v.clamp(lo.v, hi.v))
    }

    /// Inverts the color.
    #[inline]
    fn inverse(self) -> Hsv<T, S> {
        Hsv::new((self.h + Deg(cast(180))).wrap(),
                 self.s.invert_channel(),
                 self.v.invert_channel())
    }

    #[inline]
    fn mix(self, other: Self, value: T) -> Self {
        self.to_rgb().mix(other.to_rgb(),value).to_hsv() // TODO: can we mix the hsv directly?
    }
}

impl<T: FloatChannel> FloatColor<T> for Hsv<T> {
    /// Normalizes the components of the color. Modulo `360` is applied to the
    /// `h` component, and `s` and `v` are clamped to the range `(0,1)`.
    #[inline]
    fn saturate(self) -> Hsv<T> {
        Hsv::new(self.h.wrap(),
                 self.s.saturate(),
                 self.v.saturate())
    }
}

pub trait ToHsv {
    type Standard: TransferFunction;
    fn to_hsv<U:Channel + NumCast + Num>(&self) -> Hsv<U, Self::Standard>;
}

impl ToHsv for u32 {
    type Standard = Srgb;
    #[inline]
    fn to_hsv<U:Channel>(&self) -> Hsv<U, Srgb> {
        panic!("Not yet implemented")
    }
}

impl ToHsv for u64 {
    type Standard = Srgb;
    #[inline]
    fn to_hsv<U:Channel + NumCast + Num>(&self) -> Hsv<U, Srgb> {
        panic!("Not yet implemented")
    }
}

impl<T:Channel + NumCast + Num, S: TransferFunction> ToHsv for Hsv<T, S> {
    type Standard = S;
    #[inline]
    fn to_hsv<U:Channel + NumCast + Num>(&self) -> Hsv<U,S> {
        Hsv::new(Deg(cast(self.h.value())),
                 self.s.to_channel(),
                 self.v.to_channel())
    }
}

impl<T: Clone + FloatChannel, S: TransferFunction> ToRgba for Hsv<T, S> {
    type Standard = S;
    #[inline]
    fn to_rgba<U: Channel>(&self) -> Rgba<U, S>{
        Rgba{c: self.to_rgb(), a: 1.0f32.to_channel()}
    }
}

impl<T:Clone + Channel + NumCast + Num, S: TransferFunction> ToRgb for Hsv<T, S> {
    type Standard = S;
    fn to_rgb<U:Channel>(&self) -> Rgb<U, S> {
        if self.v.is_zero() {
            Rgb::new(<U as Channel>::zero(), <U as Channel>::zero(), <U as Channel>::zero())
        } else if self.s.is_zero() {
            let gray = Channel::from(self.v);
            Rgb::new(gray, gray, gray)
        } else {
            let max_f: f64 = cast(T::CHANNEL_MAX);
            let hue: f64 = cast(self.h.wrap().value());
            let hue_six: f64 = hue / 360f64 * 6f64;
            let hue_six_cat: usize = cast(hue_six);
            let hue_six_rem: T = cast(hue_six.fract() * max_f);
            let pv = Channel::from((T::CHANNEL_MAX - self.s).normalized_mul(self.v));
            let qv = Channel::from((T::CHANNEL_MAX - self.s.normalized_mul(hue_six_rem)).normalized_mul(self.v));
            let tv = Channel::from((T::CHANNEL_MAX - self.s.normalized_mul(T::CHANNEL_MAX - hue_six_rem)).normalized_mul(self.v));
            let b: U = Channel::from(self.v);
            match hue_six_cat {
                0 | 6 => Rgb::new(b,tv,pv),
                1 =>     Rgb::new(qv, b, pv),
                2 =>     Rgb::new(pv, b, tv),
                3 =>     Rgb::new(pv, qv, b),
                4 =>     Rgb::new(tv, pv, b),
                5 =>     Rgb::new(b, pv, qv),
                _ => panic!("Unreachable code")
            }
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, S> bytemuck::Pod for Hsv<T, S>
where T: Copy + 'static, S: TransferFunction {}

#[cfg(feature = "bytemuck")]
unsafe impl<T, S> bytemuck::Zeroable for Hsv<T, S>
where S: TransferFunction {}

#[cfg(test)]
mod tests {
    use {Hsv, ToHsv};
    use {Rgb, ToRgb};
    use angle::*;

    #[test]
    fn test_hsv_to_hsv() {
        assert_eq!(Hsv::<f64>::new(Deg(0.0), 0.0, 1.0).to_hsv::<f32>(),   Hsv::<f32>::new(Deg(0.0), 0.0, 1.0));
        assert_eq!(Hsv::<f64>::new(Deg(0.0), 1.0, 0.6).to_hsv::<f32>(),   Hsv::<f32>::new(Deg(0.0), 1.0, 0.6));
        assert_eq!(Hsv::<f64>::new(Deg(120.0), 1.0, 0.6).to_hsv::<f32>(), Hsv::<f32>::new(Deg(120.0), 1.0, 0.6));
        assert_eq!(Hsv::<f64>::new(Deg(240.0), 1.0, 0.6).to_hsv::<f32>(), Hsv::<f32>::new(Deg(240.0), 1.0, 0.6));
    }

    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(Hsv::<f32>::new(Deg(0.0), 0.0, 1.0).to_rgb::<u8>(),   Rgb::<u8>::new(0xFF, 0xFF, 0xFF));
        assert_eq!(Hsv::<f32>::new(Deg(0.0), 1.0, 0.6).to_rgb::<u8>(),   Rgb::<u8>::new(0x99, 0x00, 0x00));
        assert_eq!(Hsv::<f32>::new(Deg(120.0), 1.0, 0.6).to_rgb::<u8>(), Rgb::<u8>::new(0x00, 0x99, 0x00));
        assert_eq!(Hsv::<f32>::new(Deg(240.0), 1.0, 0.6).to_rgb::<u8>(), Rgb::<u8>::new(0x00, 0x00, 0x99));
        assert_eq!(Hsv::<u16>::new(Deg(0), 0, 65535).to_rgb::<u8>(),     Rgb::<u8>::new(0xFF, 0xFF, 0xFF));
        assert_eq!(Hsv::<u16>::new(Deg(0), 65535, 39321).to_rgb::<u8>(),   Rgb::<u8>::new(0x99, 0x00, 0x00));
        assert_eq!(Hsv::<u16>::new(Deg(120), 65535, 39321).to_rgb::<u8>(), Rgb::<u8>::new(0x00, 0x99, 0x00));
        assert_eq!(Hsv::<u16>::new(Deg(240), 65535, 39321).to_rgb::<u8>(), Rgb::<u8>::new(0x00, 0x00, 0x99));
    }
}
