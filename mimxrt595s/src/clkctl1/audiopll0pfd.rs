#[doc = "Register `AUDIOPLL0PFD` reader"]
pub struct R(crate::R<AUDIOPLL0PFD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIOPLL0PFD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIOPLL0PFD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIOPLL0PFD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDIOPLL0PFD` writer"]
pub struct W(crate::W<AUDIOPLL0PFD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIOPLL0PFD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AUDIOPLL0PFD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIOPLL0PFD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFD0` reader - PLL Fractional Divider 0"]
pub type PFD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFD0` writer - PLL Fractional Divider 0"]
pub type PFD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUDIOPLL0PFD_SPEC, u8, u8, 6, O>;
#[doc = "Field `PFD0_CLKRDY` reader - PFD0 Clock Ready Status Flag"]
pub type PFD0_CLKRDY_R = crate::BitReader<PFD0_CLKRDY_A>;
#[doc = "PFD0 Clock Ready Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFD0_CLKRDY_A {
    #[doc = "0: Not ready"]
    NOT_READY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<PFD0_CLKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PFD0_CLKRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PFD0_CLKRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD0_CLKRDY_A {
        match self.bits {
            false => PFD0_CLKRDY_A::NOT_READY,
            true => PFD0_CLKRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == PFD0_CLKRDY_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PFD0_CLKRDY_A::READY
    }
}
#[doc = "Field `PFD0_CLKRDY` writer - PFD0 Clock Ready Status Flag"]
pub type PFD0_CLKRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIOPLL0PFD_SPEC, PFD0_CLKRDY_A, O>;
impl<'a, const O: u8> PFD0_CLKRDY_W<'a, O> {
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(PFD0_CLKRDY_A::NOT_READY)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(PFD0_CLKRDY_A::READY)
    }
}
#[doc = "Field `PFD0_CLKGATE` reader - PFD0 Clock Gate"]
pub type PFD0_CLKGATE_R = crate::BitReader<PFD0_CLKGATE_A>;
#[doc = "PFD0 Clock Gate\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFD0_CLKGATE_A {
    #[doc = "0: PFD0 clock is not gated"]
    NOT_GATED = 0,
    #[doc = "1: PFD0 clock is gated"]
    GATED = 1,
}
impl From<PFD0_CLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: PFD0_CLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
impl PFD0_CLKGATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD0_CLKGATE_A {
        match self.bits {
            false => PFD0_CLKGATE_A::NOT_GATED,
            true => PFD0_CLKGATE_A::GATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == PFD0_CLKGATE_A::NOT_GATED
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == PFD0_CLKGATE_A::GATED
    }
}
#[doc = "Field `PFD0_CLKGATE` writer - PFD0 Clock Gate"]
pub type PFD0_CLKGATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIOPLL0PFD_SPEC, PFD0_CLKGATE_A, O>;
impl<'a, const O: u8> PFD0_CLKGATE_W<'a, O> {
    #[doc = "PFD0 clock is not gated"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(PFD0_CLKGATE_A::NOT_GATED)
    }
    #[doc = "PFD0 clock is gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(PFD0_CLKGATE_A::GATED)
    }
}
#[doc = "Field `PFD1` reader - PLL Fractional Divider 1"]
pub type PFD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFD1` writer - PLL Fractional Divider 1"]
pub type PFD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUDIOPLL0PFD_SPEC, u8, u8, 6, O>;
#[doc = "Field `PFD1_CLKRDY` reader - PFD1 Clock Ready Status Flag"]
pub type PFD1_CLKRDY_R = crate::BitReader<PFD1_CLKRDY_A>;
#[doc = "PFD1 Clock Ready Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFD1_CLKRDY_A {
    #[doc = "0: Not ready"]
    NOT_READY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<PFD1_CLKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PFD1_CLKRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PFD1_CLKRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD1_CLKRDY_A {
        match self.bits {
            false => PFD1_CLKRDY_A::NOT_READY,
            true => PFD1_CLKRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == PFD1_CLKRDY_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PFD1_CLKRDY_A::READY
    }
}
#[doc = "Field `PFD1_CLKRDY` writer - PFD1 Clock Ready Status Flag"]
pub type PFD1_CLKRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIOPLL0PFD_SPEC, PFD1_CLKRDY_A, O>;
impl<'a, const O: u8> PFD1_CLKRDY_W<'a, O> {
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(PFD1_CLKRDY_A::NOT_READY)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(PFD1_CLKRDY_A::READY)
    }
}
#[doc = "Field `PFD1_CLKGATE` reader - PFD1 Clock Gate"]
pub type PFD1_CLKGATE_R = crate::BitReader<PFD1_CLKGATE_A>;
#[doc = "PFD1 Clock Gate\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFD1_CLKGATE_A {
    #[doc = "0: PFD1 clock is not gated"]
    NOT_GATED = 0,
    #[doc = "1: PFD1 clock is gated"]
    GATED = 1,
}
impl From<PFD1_CLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: PFD1_CLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
impl PFD1_CLKGATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD1_CLKGATE_A {
        match self.bits {
            false => PFD1_CLKGATE_A::NOT_GATED,
            true => PFD1_CLKGATE_A::GATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == PFD1_CLKGATE_A::NOT_GATED
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == PFD1_CLKGATE_A::GATED
    }
}
#[doc = "Field `PFD1_CLKGATE` writer - PFD1 Clock Gate"]
pub type PFD1_CLKGATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIOPLL0PFD_SPEC, PFD1_CLKGATE_A, O>;
impl<'a, const O: u8> PFD1_CLKGATE_W<'a, O> {
    #[doc = "PFD1 clock is not gated"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(PFD1_CLKGATE_A::NOT_GATED)
    }
    #[doc = "PFD1 clock is gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(PFD1_CLKGATE_A::GATED)
    }
}
#[doc = "Field `PFD2` reader - PLL Fractional Divider 2"]
pub type PFD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFD2` writer - PLL Fractional Divider 2"]
pub type PFD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUDIOPLL0PFD_SPEC, u8, u8, 6, O>;
#[doc = "Field `PFD2_CLKRDY` reader - PFD2 Clock Ready Status Flag"]
pub type PFD2_CLKRDY_R = crate::BitReader<PFD2_CLKRDY_A>;
#[doc = "PFD2 Clock Ready Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFD2_CLKRDY_A {
    #[doc = "0: Not ready"]
    NOT_READY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<PFD2_CLKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PFD2_CLKRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PFD2_CLKRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD2_CLKRDY_A {
        match self.bits {
            false => PFD2_CLKRDY_A::NOT_READY,
            true => PFD2_CLKRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == PFD2_CLKRDY_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PFD2_CLKRDY_A::READY
    }
}
#[doc = "Field `PFD2_CLKRDY` writer - PFD2 Clock Ready Status Flag"]
pub type PFD2_CLKRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIOPLL0PFD_SPEC, PFD2_CLKRDY_A, O>;
impl<'a, const O: u8> PFD2_CLKRDY_W<'a, O> {
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(PFD2_CLKRDY_A::NOT_READY)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(PFD2_CLKRDY_A::READY)
    }
}
#[doc = "Field `PFD2_CLKGATE` reader - PFD2 Clock Gate"]
pub type PFD2_CLKGATE_R = crate::BitReader<PFD2_CLKGATE_A>;
#[doc = "PFD2 Clock Gate\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFD2_CLKGATE_A {
    #[doc = "0: PFD2 clock is not gated"]
    NOT_GATED = 0,
    #[doc = "1: PFD2 clock is gated"]
    GATED = 1,
}
impl From<PFD2_CLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: PFD2_CLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
impl PFD2_CLKGATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD2_CLKGATE_A {
        match self.bits {
            false => PFD2_CLKGATE_A::NOT_GATED,
            true => PFD2_CLKGATE_A::GATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == PFD2_CLKGATE_A::NOT_GATED
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == PFD2_CLKGATE_A::GATED
    }
}
#[doc = "Field `PFD2_CLKGATE` writer - PFD2 Clock Gate"]
pub type PFD2_CLKGATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIOPLL0PFD_SPEC, PFD2_CLKGATE_A, O>;
impl<'a, const O: u8> PFD2_CLKGATE_W<'a, O> {
    #[doc = "PFD2 clock is not gated"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(PFD2_CLKGATE_A::NOT_GATED)
    }
    #[doc = "PFD2 clock is gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(PFD2_CLKGATE_A::GATED)
    }
}
#[doc = "Field `PFD3` reader - PLL Fractional Divider 3"]
pub type PFD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFD3` writer - PLL Fractional Divider 3"]
pub type PFD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUDIOPLL0PFD_SPEC, u8, u8, 6, O>;
#[doc = "Field `PFD3_CLKRDY` reader - PFD3 Clock Ready Status Flag"]
pub type PFD3_CLKRDY_R = crate::BitReader<PFD3_CLKRDY_A>;
#[doc = "PFD3 Clock Ready Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFD3_CLKRDY_A {
    #[doc = "0: Not ready"]
    NOT_READY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<PFD3_CLKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PFD3_CLKRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PFD3_CLKRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD3_CLKRDY_A {
        match self.bits {
            false => PFD3_CLKRDY_A::NOT_READY,
            true => PFD3_CLKRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == PFD3_CLKRDY_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PFD3_CLKRDY_A::READY
    }
}
#[doc = "Field `PFD3_CLKRDY` writer - PFD3 Clock Ready Status Flag"]
pub type PFD3_CLKRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIOPLL0PFD_SPEC, PFD3_CLKRDY_A, O>;
impl<'a, const O: u8> PFD3_CLKRDY_W<'a, O> {
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(PFD3_CLKRDY_A::NOT_READY)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(PFD3_CLKRDY_A::READY)
    }
}
#[doc = "Field `PFD3_CLKGATE` reader - PFD3 Clock Gate"]
pub type PFD3_CLKGATE_R = crate::BitReader<PFD3_CLKGATE_A>;
#[doc = "PFD3 Clock Gate\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFD3_CLKGATE_A {
    #[doc = "0: PFD3 clock is not gated"]
    NOT_GATED = 0,
    #[doc = "1: PFD3 clock is gated"]
    GATED = 1,
}
impl From<PFD3_CLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: PFD3_CLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
impl PFD3_CLKGATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD3_CLKGATE_A {
        match self.bits {
            false => PFD3_CLKGATE_A::NOT_GATED,
            true => PFD3_CLKGATE_A::GATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == PFD3_CLKGATE_A::NOT_GATED
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == PFD3_CLKGATE_A::GATED
    }
}
#[doc = "Field `PFD3_CLKGATE` writer - PFD3 Clock Gate"]
pub type PFD3_CLKGATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIOPLL0PFD_SPEC, PFD3_CLKGATE_A, O>;
impl<'a, const O: u8> PFD3_CLKGATE_W<'a, O> {
    #[doc = "PFD3 clock is not gated"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(PFD3_CLKGATE_A::NOT_GATED)
    }
    #[doc = "PFD3 clock is gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(PFD3_CLKGATE_A::GATED)
    }
}
impl R {
    #[doc = "Bits 0:5 - PLL Fractional Divider 0"]
    #[inline(always)]
    pub fn pfd0(&self) -> PFD0_R {
        PFD0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - PFD0 Clock Ready Status Flag"]
    #[inline(always)]
    pub fn pfd0_clkrdy(&self) -> PFD0_CLKRDY_R {
        PFD0_CLKRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PFD0 Clock Gate"]
    #[inline(always)]
    pub fn pfd0_clkgate(&self) -> PFD0_CLKGATE_R {
        PFD0_CLKGATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PLL Fractional Divider 1"]
    #[inline(always)]
    pub fn pfd1(&self) -> PFD1_R {
        PFD1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - PFD1 Clock Ready Status Flag"]
    #[inline(always)]
    pub fn pfd1_clkrdy(&self) -> PFD1_CLKRDY_R {
        PFD1_CLKRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PFD1 Clock Gate"]
    #[inline(always)]
    pub fn pfd1_clkgate(&self) -> PFD1_CLKGATE_R {
        PFD1_CLKGATE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - PLL Fractional Divider 2"]
    #[inline(always)]
    pub fn pfd2(&self) -> PFD2_R {
        PFD2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - PFD2 Clock Ready Status Flag"]
    #[inline(always)]
    pub fn pfd2_clkrdy(&self) -> PFD2_CLKRDY_R {
        PFD2_CLKRDY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PFD2 Clock Gate"]
    #[inline(always)]
    pub fn pfd2_clkgate(&self) -> PFD2_CLKGATE_R {
        PFD2_CLKGATE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - PLL Fractional Divider 3"]
    #[inline(always)]
    pub fn pfd3(&self) -> PFD3_R {
        PFD3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - PFD3 Clock Ready Status Flag"]
    #[inline(always)]
    pub fn pfd3_clkrdy(&self) -> PFD3_CLKRDY_R {
        PFD3_CLKRDY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PFD3 Clock Gate"]
    #[inline(always)]
    pub fn pfd3_clkgate(&self) -> PFD3_CLKGATE_R {
        PFD3_CLKGATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL Fractional Divider 0"]
    #[inline(always)]
    #[must_use]
    pub fn pfd0(&mut self) -> PFD0_W<0> {
        PFD0_W::new(self)
    }
    #[doc = "Bit 6 - PFD0 Clock Ready Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfd0_clkrdy(&mut self) -> PFD0_CLKRDY_W<6> {
        PFD0_CLKRDY_W::new(self)
    }
    #[doc = "Bit 7 - PFD0 Clock Gate"]
    #[inline(always)]
    #[must_use]
    pub fn pfd0_clkgate(&mut self) -> PFD0_CLKGATE_W<7> {
        PFD0_CLKGATE_W::new(self)
    }
    #[doc = "Bits 8:13 - PLL Fractional Divider 1"]
    #[inline(always)]
    #[must_use]
    pub fn pfd1(&mut self) -> PFD1_W<8> {
        PFD1_W::new(self)
    }
    #[doc = "Bit 14 - PFD1 Clock Ready Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfd1_clkrdy(&mut self) -> PFD1_CLKRDY_W<14> {
        PFD1_CLKRDY_W::new(self)
    }
    #[doc = "Bit 15 - PFD1 Clock Gate"]
    #[inline(always)]
    #[must_use]
    pub fn pfd1_clkgate(&mut self) -> PFD1_CLKGATE_W<15> {
        PFD1_CLKGATE_W::new(self)
    }
    #[doc = "Bits 16:21 - PLL Fractional Divider 2"]
    #[inline(always)]
    #[must_use]
    pub fn pfd2(&mut self) -> PFD2_W<16> {
        PFD2_W::new(self)
    }
    #[doc = "Bit 22 - PFD2 Clock Ready Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfd2_clkrdy(&mut self) -> PFD2_CLKRDY_W<22> {
        PFD2_CLKRDY_W::new(self)
    }
    #[doc = "Bit 23 - PFD2 Clock Gate"]
    #[inline(always)]
    #[must_use]
    pub fn pfd2_clkgate(&mut self) -> PFD2_CLKGATE_W<23> {
        PFD2_CLKGATE_W::new(self)
    }
    #[doc = "Bits 24:29 - PLL Fractional Divider 3"]
    #[inline(always)]
    #[must_use]
    pub fn pfd3(&mut self) -> PFD3_W<24> {
        PFD3_W::new(self)
    }
    #[doc = "Bit 30 - PFD3 Clock Ready Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfd3_clkrdy(&mut self) -> PFD3_CLKRDY_W<30> {
        PFD3_CLKRDY_W::new(self)
    }
    #[doc = "Bit 31 - PFD3 Clock Gate"]
    #[inline(always)]
    #[must_use]
    pub fn pfd3_clkgate(&mut self) -> PFD3_CLKGATE_W<31> {
        PFD3_CLKGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL0 PFD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audiopll0pfd](index.html) module"]
pub struct AUDIOPLL0PFD_SPEC;
impl crate::RegisterSpec for AUDIOPLL0PFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audiopll0pfd::R](R) reader structure"]
impl crate::Readable for AUDIOPLL0PFD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audiopll0pfd::W](W) writer structure"]
impl crate::Writable for AUDIOPLL0PFD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUDIOPLL0PFD to value 0x8080_8080"]
impl crate::Resettable for AUDIOPLL0PFD_SPEC {
    const RESET_VALUE: Self::Ux = 0x8080_8080;
}
