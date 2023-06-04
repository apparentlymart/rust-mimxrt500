#[doc = "Register `INTB1` reader"]
pub struct R(crate::R<INTB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTB1` writer"]
pub struct W(crate::W<INTB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTB1_SPEC>;
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
impl From<crate::W<INTB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTB0` reader - Interrupt B status for DMA channel."]
pub type INTB0_R = crate::BitReader<INTB0_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB0_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB0_A> for bool {
    #[inline(always)]
    fn from(variant: INTB0_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB0_A {
        match self.bits {
            false => INTB0_A::NOT_ACTIVE,
            true => INTB0_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB0_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB0_A::ACTIVE
    }
}
#[doc = "Field `INTB0` writer - Interrupt B status for DMA channel."]
pub type INTB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB1_SPEC, INTB0_A, O>;
impl<'a, const O: u8> INTB0_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB0_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB0_A::ACTIVE)
    }
}
#[doc = "Field `INTB1` reader - Interrupt B status for DMA channel."]
pub type INTB1_R = crate::BitReader<INTB1_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB1_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB1_A> for bool {
    #[inline(always)]
    fn from(variant: INTB1_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB1_A {
        match self.bits {
            false => INTB1_A::NOT_ACTIVE,
            true => INTB1_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB1_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB1_A::ACTIVE
    }
}
#[doc = "Field `INTB1` writer - Interrupt B status for DMA channel."]
pub type INTB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB1_SPEC, INTB1_A, O>;
impl<'a, const O: u8> INTB1_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB1_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB1_A::ACTIVE)
    }
}
#[doc = "Field `INTB2` reader - Interrupt B status for DMA channel."]
pub type INTB2_R = crate::BitReader<INTB2_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB2_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB2_A> for bool {
    #[inline(always)]
    fn from(variant: INTB2_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB2_A {
        match self.bits {
            false => INTB2_A::NOT_ACTIVE,
            true => INTB2_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB2_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB2_A::ACTIVE
    }
}
#[doc = "Field `INTB2` writer - Interrupt B status for DMA channel."]
pub type INTB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB1_SPEC, INTB2_A, O>;
impl<'a, const O: u8> INTB2_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB2_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB2_A::ACTIVE)
    }
}
#[doc = "Field `INTB3` reader - Interrupt B status for DMA channel."]
pub type INTB3_R = crate::BitReader<INTB3_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB3_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB3_A> for bool {
    #[inline(always)]
    fn from(variant: INTB3_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB3_A {
        match self.bits {
            false => INTB3_A::NOT_ACTIVE,
            true => INTB3_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB3_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB3_A::ACTIVE
    }
}
#[doc = "Field `INTB3` writer - Interrupt B status for DMA channel."]
pub type INTB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB1_SPEC, INTB3_A, O>;
impl<'a, const O: u8> INTB3_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB3_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB3_A::ACTIVE)
    }
}
#[doc = "Field `INTB4` reader - Interrupt B status for DMA channel."]
pub type INTB4_R = crate::BitReader<INTB4_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB4_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB4_A> for bool {
    #[inline(always)]
    fn from(variant: INTB4_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB4_A {
        match self.bits {
            false => INTB4_A::NOT_ACTIVE,
            true => INTB4_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB4_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB4_A::ACTIVE
    }
}
#[doc = "Field `INTB4` writer - Interrupt B status for DMA channel."]
pub type INTB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB1_SPEC, INTB4_A, O>;
impl<'a, const O: u8> INTB4_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB4_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB4_A::ACTIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb0(&self) -> INTB0_R {
        INTB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb1(&self) -> INTB1_R {
        INTB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb2(&self) -> INTB2_R {
        INTB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb3(&self) -> INTB3_R {
        INTB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb4(&self) -> INTB4_R {
        INTB4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb0(&mut self) -> INTB0_W<0> {
        INTB0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb1(&mut self) -> INTB1_W<1> {
        INTB1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb2(&mut self) -> INTB2_W<2> {
        INTB2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb3(&mut self) -> INTB3_W<3> {
        INTB3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb4(&mut self) -> INTB4_W<4> {
        INTB4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt B status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intb1](index.html) module"]
pub struct INTB1_SPEC;
impl crate::RegisterSpec for INTB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intb1::R](R) reader structure"]
impl crate::Readable for INTB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intb1::W](W) writer structure"]
impl crate::Writable for INTB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTB1 to value 0"]
impl crate::Resettable for INTB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
