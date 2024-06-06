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

use num_traits::{self, Zero, Saturating, NumCast, Num, Float};
use std::{borrow::{Borrow, BorrowMut}, ops::{Mul, Div, Add, Sub, Index, IndexMut}};
use std::marker::PhantomData;
use std::mem;
use color_space::{TransferFunction, Srgb, LinearRgb, MatrixColorSpace, D65};
use angle::*;

use {Color, FloatColor};
use {Channel, FloatChannel};
use {Hsv, ToHsv};
use {Luma, ToLuma};
use xyz::{Xyz, ToXyz};
use alpha::{ToRgba, Rgba};
use std::fmt::{self, Debug};

use crate::{oklab::{OkLab, ToOkLab}, ToHsl, Hsl};


#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct Rgb<T = u8, S = Srgb> { pub r: T, pub g: T, pub b: T, standard: PhantomData<S> }

impl<T: Clone,S> Clone for Rgb<T, S>{
    fn clone(&self) -> Rgb<T, S>{
        Rgb{ r: self.r.clone(), g: self.g.clone(), b: self.b.clone(), standard: PhantomData }
    }
}

impl<T: Copy, S> Copy for Rgb<T, S>{}

impl<N: Clone + PartialEq + Num + NumCast, S> PartialEq for Rgb<N, S>{
	#[inline]
	fn eq(&self, other: &Rgb<N, S>) -> bool{
		self.r.eq(&other.r) && self.g.eq(&other.g) && self.b.eq(&other.b)
	}
}

impl<N: Clone + PartialEq + Eq + Num + NumCast, S> Eq for Rgb<N, S>{}

impl<T: Debug, S: Default + Debug> Debug for Rgb<T,S>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rgb")
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .field("standard", &S::default())
            .finish()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Rg<T = u8, S = Srgb> { pub r: T, pub g: T, pub standard: PhantomData<S> }

impl<T, S> Rg<T, S>{
    pub const fn new(r: T, g: T) -> Rg<T,S>{
        Rg{r, g, standard: PhantomData}
    }
}

impl<T: Clone,S> Clone for Rg<T, S>{
    fn clone(&self) -> Rg<T, S>{
        Rg{ r: self.r.clone(), g: self.g.clone(), standard: PhantomData }
    }
}

impl<T: Copy, S> Copy for Rg<T, S>{}

impl<N: Clone + PartialEq + Num + NumCast, S> PartialEq for Rg<N, S>{
	#[inline]
	fn eq(&self, other: &Rg<N, S>) -> bool{
		self.r.eq(&other.r) && self.g.eq(&other.g)
	}
}

impl<N: Clone + PartialEq + Eq + Num + NumCast, S> Eq for Rg<N, S>{}

impl<T: Debug, S: Default + Debug> Debug for Rg<T,S>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rg")
            .field("r", &self.r)
            .field("g", &self.g)
            .field("standard", &S::default())
            .finish()
    }
}

fn cast<T: num_traits::NumCast, U: num_traits::NumCast>(n: T) -> U {
    num_traits::cast(n).unwrap()
}

impl<T, S> Rgb<T, S> {
    #[inline]
    pub const fn new(r: T, g: T, b: T) -> Rgb<T, S> {
        Rgb { r: r, g: g, b: b, standard: PhantomData }
    }
}

impl<T: Channel, S: TransferFunction> Rgb<T, S> {
    pub fn from_hex(hex: u32) -> Rgb<T, S> {
        let r = hex >> 16 & 0xFF;
        let g = hex >> 8 & 0xFF;
        let b = hex & 0xFF;
        Rgb::<u8, S>::new(r as u8, g as u8, b as u8).to_rgb()
    }

    #[inline]
    pub fn rg(&self) -> Rg<T, S> {
        Rg{r: self.r, g: self.g, standard: PhantomData}
    }

    #[inline]
    pub fn rb(&self) -> Rg<T, S> {
        Rg{r: self.r, g: self.b, standard: PhantomData}
    }

    #[inline]
    pub fn gr(&self) -> Rg<T, S> {
        Rg{r: self.g, g: self.r, standard: PhantomData}
    }

    #[inline]
    pub fn gb(&self) -> Rg<T, S> {
        Rg{r: self.g, g: self.b, standard: PhantomData}
    }

    #[inline]
    pub fn br(&self) -> Rg<T, S> {
        Rg{r: self.b, g: self.r, standard: PhantomData}
    }

    #[inline]
    pub fn bg(&self) -> Rg<T, S> {
        Rg{r: self.b, g: self.g, standard: PhantomData}
    }

    #[inline]
    pub fn rgb(&self) -> Rgb<T, S> {
        Rgb{r: self.r, g: self.g, b: self.b, standard: PhantomData}
    }

    #[inline]
    pub fn rbg(&self) -> Rgb<T, S> {
        Rgb{r: self.r, g: self.b, b: self.g, standard: PhantomData}
    }

    #[inline]
    pub fn bgr(&self) -> Rgb<T, S> {
        Rgb{r: self.b, g: self.g, b: self.r, standard: PhantomData}
    }

    #[inline]
    pub fn brg(&self) -> Rgb<T, S> {
        Rgb{r: self.b, g: self.r, b: self.g, standard: PhantomData}
    }

    #[inline]
    pub fn grb(&self) -> Rgb<T, S> {
        Rgb{r: self.g, g: self.r, b: self.b, standard: PhantomData}
    }

    #[inline]
    pub fn gbr(&self) -> Rgb<T, S> {
        Rgb{r: self.g, g: self.b, b: self.r, standard: PhantomData}
    }
}

impl<T: Channel, S: TransferFunction> Rgb<T, S> {
    pub fn to_standard<S2: TransferFunction>(&self) -> Rgb<T, S2>{
        if std::any::TypeId::of::<S>() != std::any::TypeId::of::<S2>(){
            let r = S2::from_linear(S::to_linear(self.r.to_nearest_precision_float()));
            let g = S2::from_linear(S::to_linear(self.g.to_nearest_precision_float()));
            let b = S2::from_linear(S::to_linear(self.b.to_nearest_precision_float()));
            Rgb::new(r.to_channel(), g.to_channel(), b.to_channel())
        }else{
            Rgb::new(self.r, self.g, self.b)
        }
    }

    pub fn to_linear(&self) -> Rgb<T, LinearRgb>{
        let r = S::to_linear(self.r.to_nearest_precision_float());
        let g = S::to_linear(self.g.to_nearest_precision_float());
        let b = S::to_linear(self.b.to_nearest_precision_float());
        Rgb::new(r.to_channel(), g.to_channel(), b.to_channel())
    }
}

#[macro_export]
macro_rules! rgb{
    ( $r: expr, $g: expr, $b: expr ) => {
        $crate::Rgb::<_, $crate::color_space::Srgb>::new( $r, $g, $b )
    };
    ( $rg: expr, $b: expr ) => {
        $crate::Rgb::<_, $crate::color_space::Srgb>::new( $rg.r, $rg.g, $b )
    };
    ( $r: expr, $gb: expr ) => {
        $crate::Rgb::<_, $crate::color_space::Srgb>::new( $r, $gb.r, $gb.g )
    };
    ( $num: expr ) => {
        $crate::Rgb::<_, $crate::color_space::Srgb>::new( $num, $num, $num )
    };
}

#[macro_export]
macro_rules! rgb_linear{
    ( $r: expr, $g: expr, $b: expr ) => {
        $crate::Rgb::<_, $crate::color_space::LinearRgb>::new( $r, $g, $b )
    };
    ( $rg: expr, $b: expr ) => {
        $crate::Rgb::<_, $crate::color_space::LinearRgb>::new( $rg.r, $rg.g, $b )
    };
    ( $r: expr, $gb: expr ) => {
        $crate::Rgb::<_, $crate::color_space::LinearRgb>::new( $r, $gb.r, $gb.g )
    };
    ( $num: expr ) => {
        $crate::Rgb::<_, $crate::color_space::LinearRgb>::new( $num, $num, $num )
    };
}

impl<T: Channel, S> Color<T> for Rgb<T, S> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    fn clamp_s(self, lo: T, hi: T) -> Rgb<T, S> {
        Rgb::new(self.r.clamp(lo, hi),
                 self.g.clamp(lo, hi),
                 self.b.clamp(lo, hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    fn clamp_c(self, lo: Rgb<T, S>, hi: Rgb<T, S>) -> Rgb<T, S> {
        Rgb::new(self.r.clamp(lo.r, hi.r),
                 self.g.clamp(lo.g, hi.g),
                 self.b.clamp(lo.b, hi.b))
    }

    /// Inverts the color.
    #[inline]
    fn inverse(self) -> Rgb<T, S> {
        Rgb::new(self.r.invert_channel(),
                 self.g.invert_channel(),
                 self.b.invert_channel())
    }

    #[inline]
    fn mix(self, other: Self, value: T) -> Self {
        Rgb::new(self.r.mix(other.r, value),
             self.g.mix(other.g, value),
             self.b.mix(other.b, value))
    }
}

impl<T: FloatChannel, S> FloatColor<T> for Rgb<T, S> {
    /// Clamps the components of the color to the range `(0,1)`.
    #[inline]
    fn saturate(self) -> Rgb<T, S> {
        Rgb::new(self.r.saturate(),
                 self.g.saturate(),
                 self.b.saturate())
    }
}

pub trait ToRgb {
    type Standard: TransferFunction;
    fn to_rgb<U:Channel>(&self) -> Rgb<U, Self::Standard>;
}

impl ToRgb for u32 {
    type Standard = Srgb;
    #[inline]
    fn to_rgb<U:Channel>(&self) -> Rgb<U, Srgb> {
        let r: u8 = cast((*self >> 16) & 0xff);
        let g: u8 = cast((*self >> 8) & 0xff);
        let b: u8 = cast((*self >> 0) & 0xff);
        let r: U = Channel::from(r);
        let g: U = Channel::from(g);
        let b: U = Channel::from(b);
        rgb!(r, g, b)
    }
}

impl<T: Channel, S: TransferFunction> ToRgb for Rgb<T,S> {
    type Standard = S;
    #[inline]
    fn to_rgb<U:Channel>(&self) -> Rgb<U,S> {
        Rgb::new(self.r.to_channel(),
                 self.g.to_channel(),
                 self.b.to_channel())
    }
}

impl<T: Channel, S: TransferFunction> ToLuma for Rgb<T, S> {
    type Standard = S;
    fn to_luma<U: Channel>(&self) -> Luma<U, S> {
        Luma::new(Channel::from(
            self.r.to_nearest_precision_float() * cast::<f32, <T as Channel>::NearestFloat>(0.2126)
                + self.g.to_nearest_precision_float() * cast::<f32, <T as Channel>::NearestFloat>(0.7152)
                + self.b.to_nearest_precision_float() * cast::<f32, <T as Channel>::NearestFloat>(0.0722)
        ))
    }
}

impl<T: Channel, S: TransferFunction> ToRgba for Rgb<T, S> {
    type Standard = S;

    #[inline]
    fn to_rgba<U: Channel>(&self) -> Rgba<U, S>{
        Rgba{c: self.to_rgb(), a: 1.0f32.to_channel()}
    }
}

impl<T:Channel, S: TransferFunction> ToHsv for Rgb<T, S> {
    type Standard = S;
    #[inline]
    fn to_hsv<U:Channel + NumCast + Num>(&self) -> Hsv<U, S> {
        // Algorithm taken from the Wikipedia article on HSL and Hsv:
        // http://en.wikipedia.org/wiki/HSL_and_Hsv#From_Hsv

        let rgb_u = self.to_rgb::<U>();

        let mx = cast(cast::<U,f64>(rgb_u.r).max(cast(rgb_u.g)).max(cast(rgb_u.b)));
        let mn = cast(cast::<U,f64>(rgb_u.r).min(cast(rgb_u.g)).min(cast(rgb_u.b)));
        let chr = mx - mn;

        if chr != Zero::zero() {
            let h =
                if      rgb_u.r == mx       { ((rgb_u.g - rgb_u.b) / chr) % cast(6u8) }
                else if rgb_u.g == mx       { ((rgb_u.b - rgb_u.r) / chr) + cast(2u8) }
                else    /* rgb_u.b == mx */ { ((rgb_u.r - rgb_u.g) / chr) + cast(4u8) }
            * cast(60u8);

            let s = chr / mx;

            Hsv::new(Deg(h), s, mx)

        } else {
            Hsv::new(Zero::zero(), Zero::zero(),mx)
        }
    }
}

impl<T:Channel, S: TransferFunction> ToHsl for Rgb<T, S> {
    type Standard = S;
    #[inline]
    fn to_hsl<U:Channel + NumCast + Num>(&self) -> Hsl<U, S> {
        let rgb_f64 = self.to_rgb::<f64>();

        let mx = rgb_f64.r.channel_max(rgb_f64.g).channel_max(rgb_f64.b);
        let mn = rgb_f64.r.channel_min(rgb_f64.g).channel_min(rgb_f64.b);
        let d = mx - mn;
        let l = (mx + mn) * 0.5;
        if d != Zero::zero() {
            let s = if l < 0.5 { d / (mx + mn) } else { d / (2. - mx - mn ) } ;
            let h =
                if      rgb_f64.r == mx       { (rgb_f64.g - rgb_f64.b) / d }
                else if rgb_f64.g == mx       { ((rgb_f64.b - rgb_f64.r) / d) + 2. }
                else    /* rgb_u.b == mx */ { ((rgb_f64.r - rgb_f64.g) / d) + 4. }
            * 60.;

            Hsl::new(angle::cast(Deg(h)).unwrap(), Channel::from(s), Channel::from(l))
        } else {
            Hsl::new(Zero::zero(), Zero::zero(), Channel::from(l))
        }
    }
}

impl<T: Channel, S: MatrixColorSpace + TransferFunction> ToXyz for Rgb<T, S> {
    type WhitePoint = D65;
    fn to_xyz<U: Channel + Float>(&self) -> Xyz<U, D65> {
        let rgb = self.to_rgb().to_linear();
        let xyz = S::to_xyz_matrix() * rgb.into();
        Xyz::new(xyz[0], xyz[1], xyz[2])
    }
}

impl <T: Channel + Float + NumCast, S: MatrixColorSpace + TransferFunction> ToOkLab for Rgb<T, S> {
    fn to_oklab<U: Channel>(&self) -> OkLab<U> {
        let c: Rgb<T,_> = self.to_rgb().to_linear();
        let l = cast::<f64, T>(0.4122214708) * c.r + cast::<f64, T>(0.5363325363) * c.g + cast::<f64, T>(0.0514459929) * c.b;
        let m = cast::<f64, T>(0.2119034982) * c.r + cast::<f64, T>(0.6806995451) * c.g + cast::<f64, T>(0.1073969566) * c.b;
        let s = cast::<f64, T>(0.0883024619) * c.r + cast::<f64, T>(0.2817188376) * c.g + cast::<f64, T>(0.6299787005) * c.b;

        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();

        OkLab {
            l: (cast::<f64, T>(0.2104542553)*l_ + cast::<f64, T>(0.7936177850)*m_ - cast::<f64, T>(0.0040720468)*s_).to_channel(),
            a: (cast::<f64, T>(1.9779984951)*l_ - cast::<f64, T>(2.4285922050)*m_ + cast::<f64, T>(0.4505937099)*s_).to_channel(),
            b: (cast::<f64, T>(0.0259040371)*l_ + cast::<f64, T>(0.7827717662)*m_ - cast::<f64, T>(0.8086757660)*s_).to_channel(),
        }
    }
}

impl<T: Channel, S> Mul for Rgb<T, S> {
    type Output = Rgb<T, S>;

    #[inline]
    fn mul(self, rhs: Rgb<T, S>) -> Rgb<T, S> {
        Rgb::new(self.r.normalized_mul(rhs.r),
                 self.g.normalized_mul(rhs.g),
                 self.b.normalized_mul(rhs.b))
    }
}

impl<T: Channel + Mul<T,Output=T>, S> Mul<T> for Rgb<T, S> {
    type Output = Rgb<T, S>;

    #[inline]
    fn mul(self, rhs: T) -> Rgb<T, S> {
        Rgb::new(self.r * rhs,
                 self.g * rhs,
                 self.b * rhs)
    }
}


impl<T: Channel, S> Div for Rgb<T, S> {
    type Output = Rgb<T, S>;

    #[inline]
    fn div(self, rhs: Rgb<T, S>) -> Rgb<T, S> {
        Rgb::new(self.r.normalized_div(rhs.r),
                 self.g.normalized_div(rhs.g),
                 self.b.normalized_div(rhs.b))
    }
}

impl<T: Channel + Div<T,Output=T>, S> Div<T> for Rgb<T, S> {
    type Output = Rgb<T, S>;

    #[inline]
    fn div(self, rhs: T) -> Rgb<T, S> {
        Rgb::new(self.r / rhs,
                 self.g / rhs,
                 self.b / rhs)
    }
}

impl<T: Channel + Add<T,Output=T>, S> Add for Rgb<T, S> {
    type Output = Rgb<T, S>;

    #[inline]
    fn add(self, rhs: Rgb<T, S>) -> Rgb<T, S> {
        Rgb::new(self.r + rhs.r,
                 self.g + rhs.g,
                 self.b + rhs.b)
    }
}

impl<T: Channel + Sub<T,Output=T>, S> Sub for Rgb<T, S> {
    type Output = Rgb<T, S>;

    #[inline]
    fn sub(self, rhs: Rgb<T, S>) -> Rgb<T, S> {
        Rgb::new(self.r - rhs.r,
                 self.g - rhs.g,
                 self.b - rhs.b)
    }
}

impl<T: Channel + Saturating, S> Saturating for Rgb<T, S> {
    fn saturating_add(self, v: Rgb<T, S>) -> Rgb<T, S> {
        Rgb::new(self.r.saturating_add(v.r),
            self.g.saturating_add(v.g),
            self.b.saturating_add(v.b))
    }

    fn saturating_sub(self, v: Rgb<T, S>) -> Rgb<T, S> {
        Rgb::new(self.r.saturating_sub(v.r),
            self.g.saturating_sub(v.g),
            self.b.saturating_sub(v.b))
    }
}

impl<T, S> Index<usize> for Rgb<T, S> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        self.as_ref().index(index)
    }
}

impl<T, S> IndexMut<usize> for Rgb<T, S> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        self.as_mut().index_mut(index)
    }
}

impl<T, S> AsRef<[T;3]> for Rgb<T, S> {
    fn as_ref(&self) -> &[T;3] {
        unsafe{ mem::transmute(self) }
    }
}

impl<T, S> AsMut<[T;3]> for Rgb<T, S> {
    fn as_mut(&mut self) -> &mut [T;3] {
        unsafe{ mem::transmute(self) }
    }
}

impl<T, S> Borrow<[T;3]> for Rgb<T, S> {
    fn borrow(&self) -> &[T;3] {
        unsafe{ mem::transmute(self)}
    }
}

impl<T, S> BorrowMut<[T;3]> for Rgb<T, S> {
    fn borrow_mut(&mut self) -> &mut [T;3] {
        unsafe{ mem::transmute(self)}
    }
}

/// SVG 1.0 color constants: http://www.w3.org/TR/SVG/types.html#ColorKeywords
pub mod consts {
    use Rgb;
    use color_space::Srgb;

    pub static ALICEBLUE:               Rgb<u8, Srgb> = Rgb::new(0xF0, 0xF8, 0xFF);
    pub static ANTIQUEWHITE:            Rgb<u8, Srgb> = Rgb::new(0xFA, 0xEB, 0xD7);
    pub static AQUA:                    Rgb<u8, Srgb> = Rgb::new(0x00, 0xFF, 0xFF);
    pub static AQUAMARINE:              Rgb<u8, Srgb> = Rgb::new(0x7F, 0xFF, 0xD4);
    pub static AZURE:                   Rgb<u8, Srgb> = Rgb::new(0xF0, 0xFF, 0xFF);
    pub static BEIGE:                   Rgb<u8, Srgb> = Rgb::new(0xF5, 0xF5, 0xDC);
    pub static BISQUE:                  Rgb<u8, Srgb> = Rgb::new(0xFF, 0xE4, 0xC4);
    pub static BLACK:                   Rgb<u8, Srgb> = Rgb::new(0x00, 0x00, 0x00);
    pub static BLANCHEDALMOND:          Rgb<u8, Srgb> = Rgb::new(0xFF, 0xEB, 0xCD);
    pub static BLUE:                    Rgb<u8, Srgb> = Rgb::new(0x00, 0x00, 0xFF);
    pub static BLUEVIOLET:              Rgb<u8, Srgb> = Rgb::new(0x8A, 0x2B, 0xE2);
    pub static BROWN:                   Rgb<u8, Srgb> = Rgb::new(0xA5, 0x2A, 0x2A);
    pub static BURLYWOOD:               Rgb<u8, Srgb> = Rgb::new(0xDE, 0xB8, 0x87);
    pub static CADETBLUE:               Rgb<u8, Srgb> = Rgb::new(0x5F, 0x9E, 0xA0);
    pub static CHARTREUSE:              Rgb<u8, Srgb> = Rgb::new(0x7F, 0xFF, 0x00);
    pub static CHOCOLATE:               Rgb<u8, Srgb> = Rgb::new(0xD2, 0x69, 0x1E);
    pub static CORAL:                   Rgb<u8, Srgb> = Rgb::new(0xFF, 0x7F, 0x50);
    pub static CORNFLOWERBLUE:          Rgb<u8, Srgb> = Rgb::new(0x64, 0x95, 0xED);
    pub static CORNSILK:                Rgb<u8, Srgb> = Rgb::new(0xFF, 0xF8, 0xDC);
    pub static CRIMSON:                 Rgb<u8, Srgb> = Rgb::new(0xDC, 0x14, 0x3C);
    pub static CYAN:                    Rgb<u8, Srgb> = Rgb::new(0x00, 0xFF, 0xFF);
    pub static DARKBLUE:                Rgb<u8, Srgb> = Rgb::new(0x00, 0x00, 0x8B);
    pub static DARKCYAN:                Rgb<u8, Srgb> = Rgb::new(0x00, 0x8B, 0x8B);
    pub static DARKGOLDENROD:           Rgb<u8, Srgb> = Rgb::new(0xB8, 0x86, 0x0B);
    pub static DARKGRAY:                Rgb<u8, Srgb> = Rgb::new(0xA9, 0xA9, 0xA9);
    pub static DARKGREEN:               Rgb<u8, Srgb> = Rgb::new(0x00, 0x64, 0x00);
    pub static DARKKHAKI:               Rgb<u8, Srgb> = Rgb::new(0xBD, 0xB7, 0x6B);
    pub static DARKMAGENTA:             Rgb<u8, Srgb> = Rgb::new(0x8B, 0x00, 0x8B);
    pub static DARKOLIVEGREEN:          Rgb<u8, Srgb> = Rgb::new(0x55, 0x6B, 0x2F);
    pub static DARKORANGE:              Rgb<u8, Srgb> = Rgb::new(0xFF, 0x8C, 0x00);
    pub static DARKORCHID:              Rgb<u8, Srgb> = Rgb::new(0x99, 0x32, 0xCC);
    pub static DARKRED:                 Rgb<u8, Srgb> = Rgb::new(0x8B, 0x00, 0x00);
    pub static DARKSALMON:              Rgb<u8, Srgb> = Rgb::new(0xE9, 0x96, 0x7A);
    pub static DARKSEAGREEN:            Rgb<u8, Srgb> = Rgb::new(0x8F, 0xBC, 0x8F);
    pub static DARKSLATEBLUE:           Rgb<u8, Srgb> = Rgb::new(0x48, 0x3D, 0x8B);
    pub static DARKSLATEGRAY:           Rgb<u8, Srgb> = Rgb::new(0x2F, 0x4F, 0x4F);
    pub static DARKTURQUOISE:           Rgb<u8, Srgb> = Rgb::new(0x00, 0xCE, 0xD1);
    pub static DARKVIOLET:              Rgb<u8, Srgb> = Rgb::new(0x94, 0x00, 0xD3);
    pub static DEEPPINK:                Rgb<u8, Srgb> = Rgb::new(0xFF, 0x14, 0x93);
    pub static DEEPSKYBLUE:             Rgb<u8, Srgb> = Rgb::new(0x00, 0xBF, 0xFF);
    pub static DIMGRAY:                 Rgb<u8, Srgb> = Rgb::new(0x69, 0x69, 0x69);
    pub static DODGERBLUE:              Rgb<u8, Srgb> = Rgb::new(0x1E, 0x90, 0xFF);
    pub static FIREBRICK:               Rgb<u8, Srgb> = Rgb::new(0xB2, 0x22, 0x22);
    pub static FLORALWHITE:             Rgb<u8, Srgb> = Rgb::new(0xFF, 0xFA, 0xF0);
    pub static FORESTGREEN:             Rgb<u8, Srgb> = Rgb::new(0x22, 0x8B, 0x22);
    pub static FUCHSIA:                 Rgb<u8, Srgb> = Rgb::new(0xFF, 0x00, 0xFF);
    pub static GAINSBORO:               Rgb<u8, Srgb> = Rgb::new(0xDC, 0xDC, 0xDC);
    pub static GHOSTWHITE:              Rgb<u8, Srgb> = Rgb::new(0xF8, 0xF8, 0xFF);
    pub static GOLD:                    Rgb<u8, Srgb> = Rgb::new(0xFF, 0xD7, 0x00);
    pub static GOLDENROD:               Rgb<u8, Srgb> = Rgb::new(0xDA, 0xA5, 0x20);
    pub static GRAY:                    Rgb<u8, Srgb> = Rgb::new(0x80, 0x80, 0x80);
    pub static GREEN:                   Rgb<u8, Srgb> = Rgb::new(0x00, 0x80, 0x00);
    pub static GREENYELLOW:             Rgb<u8, Srgb> = Rgb::new(0xAD, 0xFF, 0x2F);
    pub static HONEYDEW:                Rgb<u8, Srgb> = Rgb::new(0xF0, 0xFF, 0xF0);
    pub static HOTPINK:                 Rgb<u8, Srgb> = Rgb::new(0xFF, 0x69, 0xB4);
    pub static INDIANRED:               Rgb<u8, Srgb> = Rgb::new(0xCD, 0x5C, 0x5C);
    pub static INDIGO:                  Rgb<u8, Srgb> = Rgb::new(0x4B, 0x00, 0x82);
    pub static IVORY:                   Rgb<u8, Srgb> = Rgb::new(0xFF, 0xFF, 0xF0);
    pub static KHAKI:                   Rgb<u8, Srgb> = Rgb::new(0xF0, 0xE6, 0x8C);
    pub static LAVENDER:                Rgb<u8, Srgb> = Rgb::new(0xE6, 0xE6, 0xFA);
    pub static LAVENDERBLUSH:           Rgb<u8, Srgb> = Rgb::new(0xFF, 0xF0, 0xF5);
    pub static LAWNGREEN:               Rgb<u8, Srgb> = Rgb::new(0x7C, 0xFC, 0x00);
    pub static LEMONCHIFFON:            Rgb<u8, Srgb> = Rgb::new(0xFF, 0xFA, 0xCD);
    pub static LIGHTBLUE:               Rgb<u8, Srgb> = Rgb::new(0xAD, 0xD8, 0xE6);
    pub static LIGHTCORAL:              Rgb<u8, Srgb> = Rgb::new(0xF0, 0x80, 0x80);
    pub static LIGHTCYAN:               Rgb<u8, Srgb> = Rgb::new(0xE0, 0xFF, 0xFF);
    pub static LIGHTGOLDENRODYELLOW:    Rgb<u8, Srgb> = Rgb::new(0xFA, 0xFA, 0xD2);
    pub static LIGHTGREEN:              Rgb<u8, Srgb> = Rgb::new(0x90, 0xEE, 0x90);
    pub static LIGHTGREY:               Rgb<u8, Srgb> = Rgb::new(0xD3, 0xD3, 0xD3);
    pub static LIGHTPINK:               Rgb<u8, Srgb> = Rgb::new(0xFF, 0xB6, 0xC1);
    pub static LIGHTSALMON:             Rgb<u8, Srgb> = Rgb::new(0xFF, 0xA0, 0x7A);
    pub static LIGHTSEAGREEN:           Rgb<u8, Srgb> = Rgb::new(0x20, 0xB2, 0xAA);
    pub static LIGHTSKYBLUE:            Rgb<u8, Srgb> = Rgb::new(0x87, 0xCE, 0xFA);
    pub static LIGHTSLATEGRAY:          Rgb<u8, Srgb> = Rgb::new(0x77, 0x88, 0x99);
    pub static LIGHTSTEELBLUE:          Rgb<u8, Srgb> = Rgb::new(0xB0, 0xC4, 0xDE);
    pub static LIGHTYELLOW:             Rgb<u8, Srgb> = Rgb::new(0xFF, 0xFF, 0xE0);
    pub static LIME:                    Rgb<u8, Srgb> = Rgb::new(0x00, 0xFF, 0x00);
    pub static LIMEGREEN:               Rgb<u8, Srgb> = Rgb::new(0x32, 0xCD, 0x32);
    pub static LINEN:                   Rgb<u8, Srgb> = Rgb::new(0xFA, 0xF0, 0xE6);
    pub static MAGENTA:                 Rgb<u8, Srgb> = Rgb::new(0xFF, 0x00, 0xFF);
    pub static MAROON:                  Rgb<u8, Srgb> = Rgb::new(0x80, 0x00, 0x00);
    pub static MEDIUMAQUAMARINE:        Rgb<u8, Srgb> = Rgb::new(0x66, 0xCD, 0xAA);
    pub static MEDIUMBLUE:              Rgb<u8, Srgb> = Rgb::new(0x00, 0x00, 0xCD);
    pub static MEDIUMORCHID:            Rgb<u8, Srgb> = Rgb::new(0xBA, 0x55, 0xD3);
    pub static MEDIUMPURPLE:            Rgb<u8, Srgb> = Rgb::new(0x93, 0x70, 0xDB);
    pub static MEDIUMSEAGREEN:          Rgb<u8, Srgb> = Rgb::new(0x3C, 0xB3, 0x71);
    pub static MEDIUMSLATEBLUE:         Rgb<u8, Srgb> = Rgb::new(0x7B, 0x68, 0xEE);
    pub static MEDIUMSPRINGGREEN:       Rgb<u8, Srgb> = Rgb::new(0x00, 0xFA, 0x9A);
    pub static MEDIUMTURQUOISE:         Rgb<u8, Srgb> = Rgb::new(0x48, 0xD1, 0xCC);
    pub static MEDIUMVIOLETRED:         Rgb<u8, Srgb> = Rgb::new(0xC7, 0x15, 0x85);
    pub static MIDNIGHTBLUE:            Rgb<u8, Srgb> = Rgb::new(0x19, 0x19, 0x70);
    pub static MINTCREAM:               Rgb<u8, Srgb> = Rgb::new(0xF5, 0xFF, 0xFA);
    pub static MISTYROSE:               Rgb<u8, Srgb> = Rgb::new(0xFF, 0xE4, 0xE1);
    pub static MOCCASIN:                Rgb<u8, Srgb> = Rgb::new(0xFF, 0xE4, 0xB5);
    pub static NAVAJOWHITE:             Rgb<u8, Srgb> = Rgb::new(0xFF, 0xDE, 0xAD);
    pub static NAVY:                    Rgb<u8, Srgb> = Rgb::new(0x00, 0x00, 0x80);
    pub static OLDLACE:                 Rgb<u8, Srgb> = Rgb::new(0xFD, 0xF5, 0xE6);
    pub static OLIVE:                   Rgb<u8, Srgb> = Rgb::new(0x80, 0x80, 0x00);
    pub static OLIVEDRAB:               Rgb<u8, Srgb> = Rgb::new(0x6B, 0x8E, 0x23);
    pub static ORANGE:                  Rgb<u8, Srgb> = Rgb::new(0xFF, 0xA5, 0x00);
    pub static ORANGERED:               Rgb<u8, Srgb> = Rgb::new(0xFF, 0x45, 0x00);
    pub static ORCHID:                  Rgb<u8, Srgb> = Rgb::new(0xDA, 0x70, 0xD6);
    pub static PALEGOLDENROD:           Rgb<u8, Srgb> = Rgb::new(0xEE, 0xE8, 0xAA);
    pub static PALEGREEN:               Rgb<u8, Srgb> = Rgb::new(0x98, 0xFB, 0x98);
    pub static PALEVIOLETRED:           Rgb<u8, Srgb> = Rgb::new(0xDB, 0x70, 0x93);
    pub static PAPAYAWHIP:              Rgb<u8, Srgb> = Rgb::new(0xFF, 0xEF, 0xD5);
    pub static PEACHPUFF:               Rgb<u8, Srgb> = Rgb::new(0xFF, 0xDA, 0xB9);
    pub static PERU:                    Rgb<u8, Srgb> = Rgb::new(0xCD, 0x85, 0x3F);
    pub static PINK:                    Rgb<u8, Srgb> = Rgb::new(0xFF, 0xC0, 0xCB);
    pub static PLUM:                    Rgb<u8, Srgb> = Rgb::new(0xDD, 0xA0, 0xDD);
    pub static POWDERBLUE:              Rgb<u8, Srgb> = Rgb::new(0xB0, 0xE0, 0xE6);
    pub static PURPLE:                  Rgb<u8, Srgb> = Rgb::new(0x80, 0x00, 0x80);
    pub static RED:                     Rgb<u8, Srgb> = Rgb::new(0xFF, 0x00, 0x00);
    pub static ROSYBROWN:               Rgb<u8, Srgb> = Rgb::new(0xBC, 0x8F, 0x8F);
    pub static ROYALBLUE:               Rgb<u8, Srgb> = Rgb::new(0x41, 0x69, 0xE1);
    pub static SADDLEBROWN:             Rgb<u8, Srgb> = Rgb::new(0x8B, 0x45, 0x13);
    pub static SALMON:                  Rgb<u8, Srgb> = Rgb::new(0xFA, 0x80, 0x72);
    pub static SANDYBROWN:              Rgb<u8, Srgb> = Rgb::new(0xFA, 0xA4, 0x60);
    pub static SEAGREEN:                Rgb<u8, Srgb> = Rgb::new(0x2E, 0x8B, 0x57);
    pub static SEASHELL:                Rgb<u8, Srgb> = Rgb::new(0xFF, 0xF5, 0xEE);
    pub static SIENNA:                  Rgb<u8, Srgb> = Rgb::new(0xA0, 0x52, 0x2D);
    pub static SILVER:                  Rgb<u8, Srgb> = Rgb::new(0xC0, 0xC0, 0xC0);
    pub static SKYBLUE:                 Rgb<u8, Srgb> = Rgb::new(0x87, 0xCE, 0xEB);
    pub static SLATEBLUE:               Rgb<u8, Srgb> = Rgb::new(0x6A, 0x5A, 0xCD);
    pub static SLATEGRAY:               Rgb<u8, Srgb> = Rgb::new(0x70, 0x80, 0x90);
    pub static SNOW:                    Rgb<u8, Srgb> = Rgb::new(0xFF, 0xFA, 0xFA);
    pub static SPRINGGREEN:             Rgb<u8, Srgb> = Rgb::new(0x00, 0xFF, 0x7F);
    pub static STEELBLUE:               Rgb<u8, Srgb> = Rgb::new(0x46, 0x82, 0xB4);
    pub static TAN:                     Rgb<u8, Srgb> = Rgb::new(0xD2, 0xB4, 0x8C);
    pub static TEAL:                    Rgb<u8, Srgb> = Rgb::new(0x00, 0x80, 0x80);
    pub static THISTLE:                 Rgb<u8, Srgb> = Rgb::new(0xD8, 0xBF, 0xD8);
    pub static TOMATO:                  Rgb<u8, Srgb> = Rgb::new(0xFF, 0x63, 0x47);
    pub static TURQUOISE:               Rgb<u8, Srgb> = Rgb::new(0x40, 0xE0, 0xD0);
    pub static VIOLET:                  Rgb<u8, Srgb> = Rgb::new(0xEE, 0x82, 0xEE);
    pub static WHEAT:                   Rgb<u8, Srgb> = Rgb::new(0xF5, 0xDE, 0xB3);
    pub static WHITE:                   Rgb<u8, Srgb> = Rgb::new(0xFF, 0xFF, 0xFF);
    pub static WHITESMOKE:              Rgb<u8, Srgb> = Rgb::new(0xF5, 0xF5, 0xF5);
    pub static YELLOW:                  Rgb<u8, Srgb> = Rgb::new(0xFF, 0xFF, 0x00);
    pub static YELLOWGREEN:             Rgb<u8, Srgb> = Rgb::new(0x9A, 0xCD, 0x32);
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, S> bytemuck::Pod for Rgb<T, S>
where T: Copy + 'static, S: TransferFunction {}

#[cfg(feature = "bytemuck")]
unsafe impl<T, S> bytemuck::Zeroable for Rgb<T, S>
where S: TransferFunction {}

#[cfg(test)]
mod tests {
    use {Hsv, ToHsv};
    use {Rgb, ToRgb};
    use FloatColor;
    use angle::*;
    use num_traits::Saturating;

    #[test]
    fn test_rgb_to_rgb() {
        assert_eq!(Rgb::<u8>::new(0xA0, 0xA0, 0xA0).to_rgb::<u8>(), Rgb::<u8>::new(0xA0, 0xA0, 0xA0));
        assert_eq!(Rgb::<u8>::new(0xA0, 0xA0, 0xA0).to_rgb::<u16>(), Rgb::<u16>::new(0xA0A0, 0xA0A0, 0xA0A0));
    }

    #[test]
    fn test_rgb_to_hsv() {
        assert_eq!(Rgb::<u8>::new(0xFF, 0xFF, 0xFF).to_hsv::<f32>(), Hsv::<f32>::new(Deg(0.0), 0.0, 1.0));
        assert_eq!(Rgb::<u8>::new(0x99, 0x00, 0x00).to_hsv::<f32>(), Hsv::<f32>::new(Deg(0.0), 1.0, 0.6));
        assert_eq!(Rgb::<u8>::new(0x00, 0x99, 0x00).to_hsv::<f32>(), Hsv::<f32>::new(Deg(120.0), 1.0, 0.6));
        assert_eq!(Rgb::<u8>::new(0x00, 0x00, 0x99).to_hsv::<f32>(), Hsv::<f32>::new(Deg(240.0), 1.0, 0.6));
    }

    #[test]
    fn test_rgb_ops(){
        assert_eq!( rgb!(20u8, 20, 20) + rgb!(20, 20, 20), rgb!(40, 40, 40) );
        assert_eq!( rgb!(254u8, 254, 254).saturating_add( rgb!(20, 20, 20) ), rgb!(255, 255, 255) );
        assert_eq!( rgb!(20u8, 20, 20).saturating_sub( rgb!(50, 50, 50) ), rgb!(0, 0, 0) );
        assert_eq!( rgb!(127u8, 127, 127) * rgb!(255, 255, 255), rgb!(127, 127, 127) );
        assert_eq!( rgb!(127u8, 127, 127) / rgb!(255, 255, 255), rgb!(127, 127, 127) );
        assert_eq!( rgb!(1.0f32, 1.0, 1.0) * 2.0, rgb!(2.0, 2.0, 2.0));
        assert_eq!( (rgb!(1.0f32, 1.0, 1.0) * 2.0).saturate(), rgb!(1.0, 1.0, 1.0));
    }
}
