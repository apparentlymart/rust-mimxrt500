#[doc = "Register `LUTCR` reader"]
pub struct R(crate::R<LUTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUTCR` writer"]
pub struct W(crate::W<LUTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUTCR_SPEC>;
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
impl From<crate::W<LUTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - Lock LUT"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "Lock LUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: No impact"]
    VALUE0 = 0,
    #[doc = "1: Lock LUT, LUT will be locked and can't be written"]
    VALUE1 = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::VALUE0,
            true => LOCK_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == LOCK_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOCK_A::VALUE1
    }
}
#[doc = "Field `LOCK` writer - Lock LUT"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUTCR_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "No impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(LOCK_A::VALUE0)
    }
    #[doc = "Lock LUT, LUT will be locked and can't be written"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCK_A::VALUE1)
    }
}
#[doc = "Field `UNLOCK` reader - Unlock LUT"]
pub type UNLOCK_R = crate::BitReader<UNLOCK_A>;
#[doc = "Unlock LUT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNLOCK_A {
    #[doc = "0: No impact"]
    VALUE0 = 0,
    #[doc = "1: Unlock LUT, the LUT can be written"]
    VALUE1 = 1,
}
impl From<UNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: UNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl UNLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNLOCK_A {
        match self.bits {
            false => UNLOCK_A::VALUE0,
            true => UNLOCK_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == UNLOCK_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UNLOCK_A::VALUE1
    }
}
#[doc = "Field `UNLOCK` writer - Unlock LUT"]
pub type UNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUTCR_SPEC, UNLOCK_A, O>;
impl<'a, const O: u8> UNLOCK_W<'a, O> {
    #[doc = "No impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(UNLOCK_A::VALUE0)
    }
    #[doc = "Unlock LUT, the LUT can be written"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(UNLOCK_A::VALUE1)
    }
}
impl R {
    #[doc = "Bit 0 - Lock LUT"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Unlock LUT"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock LUT"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 1 - Unlock LUT"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<1> {
        UNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutcr](index.html) module"]
pub struct LUTCR_SPEC;
impl crate::RegisterSpec for LUTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lutcr::R](R) reader structure"]
impl crate::Readable for LUTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lutcr::W](W) writer structure"]
impl crate::Writable for LUTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUTCR to value 0x02"]
impl crate::Resettable for LUTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
