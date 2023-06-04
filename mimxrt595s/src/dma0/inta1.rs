#[doc = "Register `INTA1` reader"]
pub struct R(crate::R<INTA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTA1` writer"]
pub struct W(crate::W<INTA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTA1_SPEC>;
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
impl From<crate::W<INTA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTA32` reader - Interrupt A status for DMA channel."]
pub type INTA32_R = crate::BitReader<INTA32_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA32_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA32_A> for bool {
    #[inline(always)]
    fn from(variant: INTA32_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA32_A {
        match self.bits {
            false => INTA32_A::NOT_ACTIVE,
            true => INTA32_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA32_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA32_A::ACTIVE
    }
}
#[doc = "Field `INTA32` writer - Interrupt A status for DMA channel."]
pub type INTA32_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA1_SPEC, INTA32_A, O>;
impl<'a, const O: u8> INTA32_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA32_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA32_A::ACTIVE)
    }
}
#[doc = "Field `INTA33` reader - Interrupt A status for DMA channel."]
pub type INTA33_R = crate::BitReader<INTA33_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA33_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA33_A> for bool {
    #[inline(always)]
    fn from(variant: INTA33_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA33_A {
        match self.bits {
            false => INTA33_A::NOT_ACTIVE,
            true => INTA33_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA33_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA33_A::ACTIVE
    }
}
#[doc = "Field `INTA33` writer - Interrupt A status for DMA channel."]
pub type INTA33_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA1_SPEC, INTA33_A, O>;
impl<'a, const O: u8> INTA33_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA33_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA33_A::ACTIVE)
    }
}
#[doc = "Field `INTA34` reader - Interrupt A status for DMA channel."]
pub type INTA34_R = crate::BitReader<INTA34_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA34_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA34_A> for bool {
    #[inline(always)]
    fn from(variant: INTA34_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA34_A {
        match self.bits {
            false => INTA34_A::NOT_ACTIVE,
            true => INTA34_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA34_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA34_A::ACTIVE
    }
}
#[doc = "Field `INTA34` writer - Interrupt A status for DMA channel."]
pub type INTA34_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA1_SPEC, INTA34_A, O>;
impl<'a, const O: u8> INTA34_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA34_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA34_A::ACTIVE)
    }
}
#[doc = "Field `INTA35` reader - Interrupt A status for DMA channel."]
pub type INTA35_R = crate::BitReader<INTA35_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA35_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA35_A> for bool {
    #[inline(always)]
    fn from(variant: INTA35_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA35_A {
        match self.bits {
            false => INTA35_A::NOT_ACTIVE,
            true => INTA35_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA35_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA35_A::ACTIVE
    }
}
#[doc = "Field `INTA35` writer - Interrupt A status for DMA channel."]
pub type INTA35_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA1_SPEC, INTA35_A, O>;
impl<'a, const O: u8> INTA35_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA35_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA35_A::ACTIVE)
    }
}
#[doc = "Field `INTA36` reader - Interrupt A status for DMA channel."]
pub type INTA36_R = crate::BitReader<INTA36_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA36_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA36_A> for bool {
    #[inline(always)]
    fn from(variant: INTA36_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA36_A {
        match self.bits {
            false => INTA36_A::NOT_ACTIVE,
            true => INTA36_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA36_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA36_A::ACTIVE
    }
}
#[doc = "Field `INTA36` writer - Interrupt A status for DMA channel."]
pub type INTA36_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA1_SPEC, INTA36_A, O>;
impl<'a, const O: u8> INTA36_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA36_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA36_A::ACTIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta32(&self) -> INTA32_R {
        INTA32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta33(&self) -> INTA33_R {
        INTA33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta34(&self) -> INTA34_R {
        INTA34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta35(&self) -> INTA35_R {
        INTA35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta36(&self) -> INTA36_R {
        INTA36_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta32(&mut self) -> INTA32_W<0> {
        INTA32_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta33(&mut self) -> INTA33_W<1> {
        INTA33_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta34(&mut self) -> INTA34_W<2> {
        INTA34_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta35(&mut self) -> INTA35_W<3> {
        INTA35_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta36(&mut self) -> INTA36_W<4> {
        INTA36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt A status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inta1](index.html) module"]
pub struct INTA1_SPEC;
impl crate::RegisterSpec for INTA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inta1::R](R) reader structure"]
impl crate::Readable for INTA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inta1::W](W) writer structure"]
impl crate::Writable for INTA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTA1 to value 0"]
impl crate::Resettable for INTA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
