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
pub struct Hsl<T = f32, S = Srgb> { pub h: Deg<T>, pub s: T, pub l: T, pub standard: PhantomData<S> }

impl<T: Clone,S> Clone for Hsl<T, S>{
    fn clone(&self) -> Hsl<T, S>{
        Hsl{ h: self.h.clone(), s: self.s.clone(), l: self.l.clone(), standard: PhantomData }
    }
}

impl<T: Copy, S> Copy for Hsl<T, S>{}

impl<N: Clone + PartialEq + Num + NumCast, S> PartialEq for Hsl<N, S>{
	#[inline]
	fn eq(&self, other: &Hsl<N, S>) -> bool{
		self.h.clone().wrap().eq(&other.h.clone().wrap()) && self.s.eq(&other.s) && self.l.eq(&other.l)
	}
}

impl<N: Clone + PartialEq + Eq + Num + NumCast, S> Eq for Hsl<N, S>{}

impl<T, S> Hsl<T, S> {
    pub const fn new(h: Deg<T>, s: T, l: T) -> Hsl<T, S> {
        Hsl { h: h, s: s, l: l, standard: PhantomData }
    }
}

impl<T: Channel + NumCast + Num, S: TransferFunction> Color<T> for Hsl<T, S> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    fn clamp_s(self, lo: T, hi: T) -> Hsl<T, S> {
        Hsl::new(self.h, // Should the hue component be clamped?
                 self.s.clamp(lo, hi),
                 self.l.clamp(lo, hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    fn clamp_c(self, lo: Hsl<T, S>, hi: Hsl<T, S>) -> Hsl<T, S> {
        Hsl::new(self.h,
                 self.s.clamp(lo.s, hi.s),
                 self.l.clamp(lo.l, hi.l))
    }

    /// Inverts the color.
    #[inline]
    fn inverse(self) -> Hsl<T, S> {
        Hsl::new((self.h + Deg(cast(180))).wrap(),
                 self.s.invert_channel(),
                 self.l.invert_channel())
    }

    #[inline]
    fn mix(self, other: Self, value: T) -> Self {
        self.to_rgb().mix(other.to_rgb(),value).to_hsl() // TODO: can we mix the hsl directly?
    }
}

impl<T: FloatChannel> FloatColor<T> for Hsl<T> {
    /// Normalizes the components of the color. Modulo `360` is applied to the
    /// `h` component, and `s` and `l` are clamped to the range `(0,1)`.
    #[inline]
    fn saturate(self) -> Hsl<T> {
        Hsl::new(self.h.wrap(),
                 self.s.saturate(),
                 self.l.saturate())
    }
}

pub trait ToHsl {
    type Standard: TransferFunction;
    fn to_hsl<U:Channel + NumCast + Num>(&self) -> Hsl<U, Self::Standard>;
}

impl ToHsl for u32 {
    type Standard = Srgb;
    #[inline]
    fn to_hsl<U:Channel>(&self) -> Hsl<U, Srgb> {
        panic!("Not yet implemented")
    }
}

impl ToHsl for u64 {
    type Standard = Srgb;
    #[inline]
    fn to_hsl<U:Channel + NumCast + Num>(&self) -> Hsl<U, Srgb> {
        panic!("Not yet implemented")
    }
}

impl<T:Channel + NumCast + Num, S: TransferFunction> ToHsl for Hsl<T, S> {
    type Standard = S;
    #[inline]
    fn to_hsl<U:Channel + NumCast + Num>(&self) -> Hsl<U,S> {
        Hsl::new(Deg(cast(self.h.value())),
                 self.s.to_channel(),
                 self.l.to_channel())
    }
}

impl<T: Clone + FloatChannel, S: TransferFunction> ToRgba for Hsl<T, S> {
    type Standard = S;
    #[inline]
    fn to_rgba<U: Channel>(&self) -> Rgba<U, S>{
        Rgba{c: self.to_rgb(), a: 1.0f32.to_channel()}
    }
}

impl<T:Clone + Channel + NumCast + Num, S: TransferFunction> ToRgb for Hsl<T, S> {
    type Standard = S;
    fn to_rgb<U:Channel>(&self) -> Rgb<U, S> {
        if self.l.is_zero() {
            Rgb::new(<U as Channel>::zero(), <U as Channel>::zero(), <U as Channel>::zero())
        } else if self.s.is_zero() {
            let gray = Channel::from(self.l);
            Rgb::new(gray, gray, gray)
        } else {
            let a: f32 = Channel::from(self.s.normalized_mul(self.l.channel_min(T::CHANNEL_MAX - self.l)));
            let f = |n| {
                let hue: f32 = cast(self.h.wrap().value());
                let hue_six: f32 = hue / 30f32;
                let k: f32 = (n + hue_six) % 12.;
                let l: f32 = Channel::from(self.l);
                <U as Channel>::from(l - a * (k - 3.).min(9. - k).min(1.).max(-1.))
            };
            rgb!(f(0.), f(8.), f(4.)).to_standard()
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, S> bytemuck::Pod for Hsl<T, S>
where T: Copy + 'static, S: TransferFunction {}

#[cfg(feature = "bytemuck")]
unsafe impl<T, S> bytemuck::Zeroable for Hsl<T, S>
where S: TransferFunction {}

#[cfg(test)]
mod tests {
    use {Hsl, ToHsl};
    use {Rgb, ToRgb};
    use angle::*;

    #[test]
    fn test_hsl_to_hsl() {
        assert_eq!(Hsl::<f64>::new(Deg(0.0), 0.0, 1.0).to_hsl::<f32>(),   Hsl::<f32>::new(Deg(0.0), 0.0, 1.0));
        assert_eq!(Hsl::<f64>::new(Deg(0.0), 1.0, 0.6).to_hsl::<f32>(),   Hsl::<f32>::new(Deg(0.0), 1.0, 0.6));
        assert_eq!(Hsl::<f64>::new(Deg(120.0), 1.0, 0.6).to_hsl::<f32>(), Hsl::<f32>::new(Deg(120.0), 1.0, 0.6));
        assert_eq!(Hsl::<f64>::new(Deg(240.0), 1.0, 0.6).to_hsl::<f32>(), Hsl::<f32>::new(Deg(240.0), 1.0, 0.6));
    }

    #[test]
    fn test_hsl_to_rgb() {
        assert_eq!(Hsl::<f32>::new(Deg(0.0), 0.0, 1.0).to_rgb::<u8>(),   Rgb::<u8>::new(0xFF, 0xFF, 0xFF));
        assert_eq!(Hsl::<f32>::new(Deg(0.0), 1.0, 0.6).to_rgb::<u8>(),   Rgb::<u8>::new(0xFF, 0x33, 0x33));
        assert_eq!(Hsl::<f32>::new(Deg(120.0), 1.0, 0.6).to_rgb::<u8>(), Rgb::<u8>::new(0x33, 0xff, 0x33));
        assert_eq!(Hsl::<f32>::new(Deg(240.0), 1.0, 0.6).to_rgb::<u8>(), Rgb::<u8>::new(0x33, 0x33, 0xff));
        assert_eq!(Hsl::<u16>::new(Deg(0), 0, 65535).to_rgb::<u8>(),     Rgb::<u8>::new(0xFF, 0xFF, 0xFF));
        assert_eq!(Hsl::<u16>::new(Deg(0), 65535, 39321).to_rgb::<u8>(),   Rgb::<u8>::new(0xff, 0x33, 0x33));
        assert_eq!(Hsl::<u16>::new(Deg(120), 65535, 39321).to_rgb::<u8>(), Rgb::<u8>::new(0x33, 0xff, 0x33));
        assert_eq!(Hsl::<u16>::new(Deg(240), 65535, 39321).to_rgb::<u8>(), Rgb::<u8>::new(0x33, 0x33, 0xff));
    }

    #[test]
    fn test_rgb_to_hsl() {
        assert_eq!(Rgb::<u8>::new(0xFF, 0xFF, 0xFF).to_hsl(), Hsl::<f32>::new(Deg(0.0), 0.0, 1.0));
        assert_eq!(Rgb::<u8>::new(0xFF, 0x33, 0x33).to_hsl(), Hsl::<f32>::new(Deg(0.0), 1.0, 0.6));
        assert_eq!(Rgb::<u8>::new(0x33, 0xff, 0x33).to_hsl(), Hsl::<f32>::new(Deg(120.0), 1.0, 0.6));
        assert_eq!(Rgb::<u8>::new(0x33, 0x33, 0xff).to_hsl(), Hsl::<f32>::new(Deg(240.0), 1.0, 0.6));
        assert_eq!(Rgb::<u8>::new(0xFF, 0xFF, 0xFF).to_hsl(), Hsl::<u16>::new(Deg(0), 0, 65535));
        assert_eq!(Rgb::<u8>::new(0xff, 0x33, 0x33).to_hsl(), Hsl::<u16>::new(Deg(0), 65535, 39321));
        assert_eq!(Rgb::<u8>::new(0x33, 0xff, 0x33).to_hsl(), Hsl::<u16>::new(Deg(120), 65535, 39321));
        assert_eq!(Rgb::<u8>::new(0x33, 0x33, 0xff).to_hsl(), Hsl::<u16>::new(Deg(240), 65535, 39321));
    }
}
