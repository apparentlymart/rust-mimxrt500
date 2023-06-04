#[doc = "Register `ENABLECLR1` reader"]
pub struct R(crate::R<ENABLECLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLECLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLECLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLECLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLECLR1` writer"]
pub struct W(crate::W<ENABLECLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLECLR1_SPEC>;
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
impl From<crate::W<ENABLECLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLECLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR32` reader - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR32_R = crate::BitReader<CLR32_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR32_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR32_A> for bool {
    #[inline(always)]
    fn from(variant: CLR32_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR32_A {
        match self.bits {
            false => CLR32_A::NO_EFFECT,
            true => CLR32_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR32_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR32_A::CLEARED
    }
}
#[doc = "Field `CLR32` writer - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR32_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR1_SPEC, CLR32_A, O>;
impl<'a, const O: u8> CLR32_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR32_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR32_A::CLEARED)
    }
}
#[doc = "Field `CLR33` reader - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR33_R = crate::BitReader<CLR33_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR33_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR33_A> for bool {
    #[inline(always)]
    fn from(variant: CLR33_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR33_A {
        match self.bits {
            false => CLR33_A::NO_EFFECT,
            true => CLR33_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR33_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR33_A::CLEARED
    }
}
#[doc = "Field `CLR33` writer - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR33_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR1_SPEC, CLR33_A, O>;
impl<'a, const O: u8> CLR33_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR33_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR33_A::CLEARED)
    }
}
#[doc = "Field `CLR34` reader - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR34_R = crate::BitReader<CLR34_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR34_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR34_A> for bool {
    #[inline(always)]
    fn from(variant: CLR34_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR34_A {
        match self.bits {
            false => CLR34_A::NO_EFFECT,
            true => CLR34_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR34_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR34_A::CLEARED
    }
}
#[doc = "Field `CLR34` writer - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR34_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR1_SPEC, CLR34_A, O>;
impl<'a, const O: u8> CLR34_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR34_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR34_A::CLEARED)
    }
}
#[doc = "Field `CLR35` reader - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR35_R = crate::BitReader<CLR35_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR35_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR35_A> for bool {
    #[inline(always)]
    fn from(variant: CLR35_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR35_A {
        match self.bits {
            false => CLR35_A::NO_EFFECT,
            true => CLR35_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR35_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR35_A::CLEARED
    }
}
#[doc = "Field `CLR35` writer - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR35_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR1_SPEC, CLR35_A, O>;
impl<'a, const O: u8> CLR35_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR35_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR35_A::CLEARED)
    }
}
#[doc = "Field `CLR36` reader - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR36_R = crate::BitReader<CLR36_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR36_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR36_A> for bool {
    #[inline(always)]
    fn from(variant: CLR36_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR36_A {
        match self.bits {
            false => CLR36_A::NO_EFFECT,
            true => CLR36_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR36_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR36_A::CLEARED
    }
}
#[doc = "Field `CLR36` writer - Writing ones to this register clears the corresponding bits in ENABLESET1."]
pub type CLR36_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR1_SPEC, CLR36_A, O>;
impl<'a, const O: u8> CLR36_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR36_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR36_A::CLEARED)
    }
}
impl R {
    #[doc = "Bit 0 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    pub fn clr32(&self) -> CLR32_R {
        CLR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    pub fn clr33(&self) -> CLR33_R {
        CLR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    pub fn clr34(&self) -> CLR34_R {
        CLR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    pub fn clr35(&self) -> CLR35_R {
        CLR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    pub fn clr36(&self) -> CLR36_R {
        CLR36_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    #[must_use]
    pub fn clr32(&mut self) -> CLR32_W<0> {
        CLR32_W::new(self)
    }
    #[doc = "Bit 1 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    #[must_use]
    pub fn clr33(&mut self) -> CLR33_W<1> {
        CLR33_W::new(self)
    }
    #[doc = "Bit 2 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    #[must_use]
    pub fn clr34(&mut self) -> CLR34_W<2> {
        CLR34_W::new(self)
    }
    #[doc = "Bit 3 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    #[must_use]
    pub fn clr35(&mut self) -> CLR35_W<3> {
        CLR35_W::new(self)
    }
    #[doc = "Bit 4 - Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    #[must_use]
    pub fn clr36(&mut self) -> CLR36_W<4> {
        CLR36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Clear for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableclr1](index.html) module"]
pub struct ENABLECLR1_SPEC;
impl crate::RegisterSpec for ENABLECLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enableclr1::R](R) reader structure"]
impl crate::Readable for ENABLECLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enableclr1::W](W) writer structure"]
impl crate::Writable for ENABLECLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1f;
}
#[doc = "`reset()` method sets ENABLECLR1 to value 0"]
impl crate::Resettable for ENABLECLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
