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

use std::{borrow::{Borrow, BorrowMut}, ops::{Mul, Div, Add, Sub, Index, IndexMut}};
use color_space::TransferFunction;
use num_traits::Saturating;
use std::mem;
use {Color, Channel};
use {Rgb, Rg, ToRgb, Hsv, YCbCr};
use color_space::{Srgb, LinearRgb};
use luma::{Luma, ToLuma};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct AlphaColor<T, C> { pub c: C, pub a: T }

pub type Rgba<T = u8, S = Srgb> = AlphaColor<T, Rgb<T,S>>;
pub type LumaA<T = f32, S = Srgb> = AlphaColor<T, Luma<T,S>>;
pub type Hsva<T = f32, S = Srgb> = AlphaColor<T, Hsv<T,S>>;
pub type YCbCra<T> = AlphaColor<T, YCbCr<T>>;

impl<T, C> AlphaColor<T, C>{
    pub const fn new(c: C, a: T) -> AlphaColor<T,C>{
        AlphaColor{c, a}
    }
}

impl<T: Channel, C: Color<T>> Color<T> for AlphaColor<T, C> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    fn clamp_s(self, lo: T, hi: T) -> AlphaColor<T, C> {
        AlphaColor {
            c: self.c.clamp_s(lo, hi),
            a: self.a.clamp(lo, hi),
        }
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    fn clamp_c(self, lo: AlphaColor<T, C>, hi: AlphaColor<T, C>) -> AlphaColor<T, C> {
        AlphaColor {
            c: self.c.clamp_c(lo.c, hi.c),
            a: self.a.clamp(lo.a, hi.a),
        }
    }

    /// Inverts the color.
    #[inline]
    fn inverse(self) -> AlphaColor<T, C> {
        AlphaColor {
            c: self.c.inverse(),
            a: self.a.invert_channel(),
        }
    }

    #[inline]
    fn mix(self, other: Self, value: T) -> Self {
        AlphaColor {
            c: self.c.mix(other.c, value),
            a: self.a.mix(other.a, value)
        }
    }
}

#[macro_export]
macro_rules! rgba{
    ( $r: expr, $g: expr, $b: expr, $a: expr ) => ({
        use $crate::{Rgba, Rgb};
        Rgba{ c: Rgb::<_,$crate::color_space::Srgb>::new($r, $g, $b), a: $a }
    });
    ( $to_rgb: expr, $a: expr ) => ({
        use $crate::{Rgba,ToRgb};
        Rgba{ c: $to_rgb.to_rgb(), a: $a }
    });
}

#[macro_export]
macro_rules! rgba_linear{
    ( $r: expr, $g: expr, $b: expr, $a: expr ) => ({
        use $crate::{Rgba, Rgb};
        Rgba{ c: Rgb::<_,$crate::color_space::LinearRgb>::new($r, $g, $b), a: $a }
    });
    ( $to_rgb: expr, $a: expr ) => ({
        use $crate::{Rgba,ToRgb};
        Rgba{ c: $to_rgb.to_rgb().to_linear(), a: $a }
    });
}

impl<T:Channel, S: TransferFunction> Rgba<T, S> {
    pub fn from_hex(hex: u32) -> Rgba<T, S> {
        let r = hex >> 24 & 0xFF;
        let g = hex >> 16 & 0xFF;
        let b = hex >> 8 & 0xFF;
        let a = hex & 0xFF;
        Rgba{c: Rgb::new(r as u8, g as u8, b as u8), a: a as u8}.to_rgba()
    }

    #[inline]
    pub fn rg(&self) -> Rg<T, S> {
        self.c.rg()
    }

    #[inline]
    pub fn rb(&self) -> Rg<T, S> {
        self.c.rb()
    }

    #[inline]
    pub fn gr(&self) -> Rg<T, S> {
        self.c.gr()
    }

    #[inline]
    pub fn gb(&self) -> Rg<T, S> {
        self.c.gb()
    }

    #[inline]
    pub fn br(&self) -> Rg<T, S> {
        self.c.br()
    }

    #[inline]
    pub fn bg(&self) -> Rg<T, S> {
        self.c.bg()
    }

    #[inline]
    pub fn ar(&self) -> Rg<T, S> {
        Rg::new(self.a, self.c.r)
    }

    #[inline]
    pub fn ag(&self) -> Rg<T, S> {
        Rg::new(self.a, self.c.g)
    }

    #[inline]
    pub fn ab(&self) -> Rg<T, S> {
        Rg::new(self.a, self.c.b)
    }

    #[inline]
    pub fn ra(&self) -> Rg<T, S> {
        Rg::new(self.c.r, self.a)
    }

    #[inline]
    pub fn ga(&self) -> Rg<T, S> {
        Rg::new(self.c.g, self.a)
    }

    #[inline]
    pub fn ba(&self) -> Rg<T, S> {
        Rg::new(self.c.b, self.a)
    }

    #[inline]
    pub fn rgb(&self) -> Rgb<T, S> {
        self.c.rgb()
    }

    #[inline]
    pub fn rbg(&self) -> Rgb<T, S> {
        self.c.rbg()
    }

    #[inline]
    pub fn bgr(&self) -> Rgb<T, S> {
        self.c.bgr()
    }

    #[inline]
    pub fn brg(&self) -> Rgb<T, S> {
        self.c.brg()
    }

    #[inline]
    pub fn grb(&self) -> Rgb<T, S> {
        self.c.grb()
    }

    #[inline]
    pub fn gbr(&self) -> Rgb<T, S> {
        self.c.gbr()
    }

    #[inline]
    pub fn rga(&self) -> Rgb<T, S> {
        Rgb::new(self.c.r,self.c.g,self.a)
    }

    #[inline]
    pub fn rba(&self) -> Rgb<T, S> {
        Rgb::new(self.c.r,self.c.b,self.a)
    }

    #[inline]
    pub fn bra(&self) -> Rgb<T, S> {
        Rgb::new(self.c.b,self.c.r,self.a)
    }

    #[inline]
    pub fn bga(&self) -> Rgb<T, S> {
        Rgb::new(self.c.b,self.c.g,self.a)
    }

    #[inline]
    pub fn gra(&self) -> Rgb<T, S> {
        Rgb::new(self.c.g,self.c.r,self.a)
    }

    #[inline]
    pub fn gba(&self) -> Rgb<T, S> {
        Rgb::new(self.c.g,self.c.b,self.a)
    }

    #[inline]
    pub fn arg(&self) -> Rgb<T, S> {
        Rgb::new(self.a,self.c.r,self.c.g)
    }

    #[inline]
    pub fn arb(&self) -> Rgb<T, S> {
        Rgb::new(self.a,self.c.r,self.c.b)
    }

    #[inline]
    pub fn agr(&self) -> Rgb<T, S> {
        Rgb::new(self.a,self.c.g,self.c.r)
    }

    #[inline]
    pub fn agb(&self) -> Rgb<T, S> {
        Rgb::new(self.a,self.c.g,self.c.b)
    }

    #[inline]
    pub fn abr(&self) -> Rgb<T, S> {
        Rgb::new(self.a,self.c.b,self.c.r)
    }

    #[inline]
    pub fn abg(&self) -> Rgb<T, S> {
        Rgb::new(self.a,self.c.b,self.c.g)
    }

    #[inline]
    pub fn rag(&self) -> Rgb<T, S> {
        Rgb::new(self.c.r,self.a,self.c.g)
    }

    #[inline]
    pub fn rab(&self) -> Rgb<T, S> {
        Rgb::new(self.c.r,self.a,self.c.b)
    }

    #[inline]
    pub fn gar(&self) -> Rgb<T, S> {
        Rgb::new(self.c.g,self.a,self.c.r)
    }

    #[inline]
    pub fn gab(&self) -> Rgb<T, S> {
        Rgb::new(self.c.g,self.a,self.c.b)
    }

    #[inline]
    pub fn bar(&self) -> Rgb<T, S> {
        Rgb::new(self.c.b,self.a,self.c.r)
    }

    #[inline]
    pub fn bag(&self) -> Rgb<T, S> {
        Rgb::new(self.c.b,self.a,self.c.g)
    }

    #[inline]
    pub fn rgba(&self) -> Rgba<T, S> {
        rgba!(self.c, self.a)
    }

    #[inline]
    pub fn rbga(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.r, self.c.b, self.c.g), self.a)
    }

    #[inline]
    pub fn grba(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.g, self.c.r, self.c.b), self.a)
    }

    #[inline]
    pub fn gbra(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.g, self.c.b, self.c.r), self.a)
    }

    #[inline]
    pub fn brga(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.b, self.c.r, self.c.g), self.a)
    }

    #[inline]
    pub fn bgra(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.b, self.c.g, self.c.r), self.a)
    }

    #[inline]
    pub fn argb(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.a, self.c.r, self.c.g), self.c.b)
    }

    #[inline]
    pub fn arbg(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.a, self.c.r, self.c.b), self.c.g)
    }

    #[inline]
    pub fn agrb(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.a, self.c.g, self.c.r), self.c.b)
    }

    #[inline]
    pub fn agbr(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.a, self.c.g, self.c.b), self.c.r)
    }

    #[inline]
    pub fn abrg(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.a, self.c.b, self.c.r), self.c.g)
    }

    #[inline]
    pub fn abgr(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.a, self.c.b, self.c.g), self.c.r)
    }

    #[inline]
    pub fn ragb(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.r, self.a, self.c.g), self.c.b)
    }

    #[inline]
    pub fn rabg(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.r, self.a, self.c.b), self.c.g)
    }

    #[inline]
    pub fn garb(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.g, self.a, self.c.r), self.c.b)
    }

    #[inline]
    pub fn gabr(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.g, self.a, self.c.b), self.c.r)
    }

    #[inline]
    pub fn barg(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.b, self.a, self.c.r), self.c.g)
    }

    #[inline]
    pub fn bagr(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.b, self.a, self.c.g), self.c.r)
    }

    #[inline]
    pub fn rgab(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.r, self.c.g, self.a), self.c.b)
    }

    #[inline]
    pub fn rbag(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.r, self.c.b, self.a), self.c.g)
    }

    #[inline]
    pub fn grab(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.g, self.c.r, self.a), self.c.b)
    }

    #[inline]
    pub fn gbar(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.g, self.c.b, self.a), self.c.r)
    }

    #[inline]
    pub fn brag(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.b, self.c.r, self.a), self.c.g)
    }

    #[inline]
    pub fn bgar(&self) -> Rgba<T, S> {
        Rgba::new(Rgb::new(self.c.b, self.c.g, self.a), self.c.r)
    }
}

impl<T: Channel, S: TransferFunction> Rgba<T, S> {
    pub fn to_standard<S2: TransferFunction>(&self) -> Rgba<T, S2>{
        let c = self.c.to_standard();
        Rgba{c, a: self.a}
    }

    pub fn to_linear(&self) -> Rgba<T, LinearRgb>{
        let c = self.c.to_linear();
        Rgba{c, a: self.a}
    }
}


pub trait ToRgba{
    type Standard: TransferFunction;
    fn to_rgba<T: Channel>(&self) -> Rgba<T, Self::Standard>;
}

impl<T: Channel, C: ToRgb> ToRgba for AlphaColor<T,C>{
    type Standard = <C as ToRgb>::Standard;
    #[inline]
    fn to_rgba<U: Channel>(&self) -> Rgba<U, Self::Standard>{
        Rgba{c: self.c.to_rgb(), a: self.a.to_channel()}
    }
}

impl<T, C: ToRgb> ToRgb for AlphaColor<T, C> {
    type Standard = <C as ToRgb>::Standard;
    #[inline]
    fn to_rgb<U:Channel>(&self) -> Rgb<U, Self::Standard> {
        self.c.to_rgb()
    }
}

impl<T, C: ToLuma> ToLuma for AlphaColor<T, C> {
    type Standard = <C as ToLuma>::Standard;
    #[inline]
    fn to_luma<U:Channel>(&self) -> Luma<U, Self::Standard> {
        self.c.to_luma()
    }
}

impl<T:Channel, C: Mul<Output=C>> Mul for AlphaColor<T,C> {
    type Output = AlphaColor<T,C>;

    #[inline]
    fn mul(self, rhs: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c.mul(rhs.c),
             a: self.a.normalized_mul(rhs.a) }
    }
}

impl<T:Channel + Mul<T,Output=T>, C: Mul<T,Output=C>> Mul<T> for AlphaColor<T,C> {
    type Output = AlphaColor<T,C>;

    #[inline]
    fn mul(self, rhs: T) -> AlphaColor<T,C> {
        let color = self.c * rhs;
        AlphaColor{ c: color,
             a: self.a * rhs }
    }
}

impl<T:Channel, C: Div<Output=C>> Div for AlphaColor<T,C> {
    type Output = AlphaColor<T,C>;

    #[inline]
    fn div(self, rhs: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c.div(rhs.c),
             a: self.a.normalized_div(rhs.a) }
    }
}

impl<T:Channel + Div<T,Output=T>, C: Div<T,Output=C>> Div<T> for AlphaColor<T,C> {
    type Output = AlphaColor<T,C>;

    #[inline]
    fn div(self, rhs: T) -> AlphaColor<T,C> {
        let color = self.c / rhs;
        AlphaColor{ c: color,
             a: self.a / rhs }
    }
}

impl<T:Channel + Add<T,Output=T>, C: Add<Output=C>> Add for AlphaColor<T,C>{
    type Output = AlphaColor<T,C>;

    #[inline]
    fn add(self, rhs: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c + rhs.c,
             a: self.a + rhs.a }
    }
}

impl<T:Channel + Sub<T,Output=T>, C: Sub<Output=C>> Sub for AlphaColor<T,C>{
    type Output = AlphaColor<T,C>;

    #[inline]
    fn sub(self, rhs: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c - rhs.c,
             a: self.a - rhs.a }
    }
}

impl<T:Channel + Saturating, C: Saturating> Saturating for AlphaColor<T,C>{
    fn saturating_add(self, v: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c.saturating_add(v.c),
              a: self.a.saturating_add(v.a) }
    }

    fn saturating_sub(self, v: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c.saturating_sub(v.c),
              a: self.a.saturating_sub(v.a) }
    }
}

impl<T, C: AsRef<[T;3]>> Index<usize> for AlphaColor<T,C> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        self.as_ref().index(index)
    }
}

impl<T, C: AsRef<[T;3]> + AsMut<[T;3]>> IndexMut<usize> for AlphaColor<T,C> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        self.as_mut().index_mut(index)
    }
}

impl<T, C: AsRef<[T;3]>> AsRef<[T;4]> for AlphaColor<T,C> {
    fn as_ref(&self) -> &[T;4] {
        unsafe{ mem::transmute(self)}
    }
}

impl<T, C: AsMut<[T;3]>> AsMut<[T;4]> for AlphaColor<T,C> {
    fn as_mut(&mut self) -> &mut [T;4] {
        unsafe{ mem::transmute(self)}
    }
}

impl<T, C: Borrow<[T;3]>> Borrow<[T;4]> for AlphaColor<T,C> {
    fn borrow(&self) -> &[T;4] {
        unsafe{ mem::transmute(self)}
    }
}

impl<T, C: BorrowMut<[T;3]>> BorrowMut<[T;4]> for AlphaColor<T,C> {
    fn borrow_mut(&mut self) -> &mut [T;4] {
        unsafe{ mem::transmute(self)}
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, C> bytemuck::Pod for AlphaColor<T, C>
where T: Copy + 'static, C: bytemuck::Pod {}

#[cfg(feature = "bytemuck")]
unsafe impl<T, C> bytemuck::Zeroable for AlphaColor<T, C>
where C: bytemuck::Zeroable {}