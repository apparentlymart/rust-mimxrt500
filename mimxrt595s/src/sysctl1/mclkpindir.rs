#[doc = "Register `MCLKPINDIR` reader"]
pub struct R(crate::R<MCLKPINDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKPINDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKPINDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKPINDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKPINDIR` writer"]
pub struct W(crate::W<MCLKPINDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKPINDIR_SPEC>;
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
impl From<crate::W<MCLKPINDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKPINDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCLKPINDIR` reader - MCLK direction control"]
pub type MCLKPINDIR_R = crate::BitReader<MCLKPINDIR_A>;
#[doc = "MCLK direction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLKPINDIR_A {
    #[doc = "0: I2S MCLK is in input direction"]
    MCLKPINDIR_0 = 0,
    #[doc = "1: I2S MCLK is in the output direction"]
    MCLKPINDIR_1 = 1,
}
impl From<MCLKPINDIR_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKPINDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl MCLKPINDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKPINDIR_A {
        match self.bits {
            false => MCLKPINDIR_A::MCLKPINDIR_0,
            true => MCLKPINDIR_A::MCLKPINDIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCLKPINDIR_0`"]
    #[inline(always)]
    pub fn is_mclkpindir_0(&self) -> bool {
        *self == MCLKPINDIR_A::MCLKPINDIR_0
    }
    #[doc = "Checks if the value of the field is `MCLKPINDIR_1`"]
    #[inline(always)]
    pub fn is_mclkpindir_1(&self) -> bool {
        *self == MCLKPINDIR_A::MCLKPINDIR_1
    }
}
#[doc = "Field `MCLKPINDIR` writer - MCLK direction control"]
pub type MCLKPINDIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MCLKPINDIR_SPEC, MCLKPINDIR_A, O>;
impl<'a, const O: u8> MCLKPINDIR_W<'a, O> {
    #[doc = "I2S MCLK is in input direction"]
    #[inline(always)]
    pub fn mclkpindir_0(self) -> &'a mut W {
        self.variant(MCLKPINDIR_A::MCLKPINDIR_0)
    }
    #[doc = "I2S MCLK is in the output direction"]
    #[inline(always)]
    pub fn mclkpindir_1(self) -> &'a mut W {
        self.variant(MCLKPINDIR_A::MCLKPINDIR_1)
    }
}
impl R {
    #[doc = "Bit 0 - MCLK direction control"]
    #[inline(always)]
    pub fn mclkpindir(&self) -> MCLKPINDIR_R {
        MCLKPINDIR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCLK direction control"]
    #[inline(always)]
    #[must_use]
    pub fn mclkpindir(&mut self) -> MCLKPINDIR_W<0> {
        MCLKPINDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK direction control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkpindir](index.html) module"]
pub struct MCLKPINDIR_SPEC;
impl crate::RegisterSpec for MCLKPINDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclkpindir::R](R) reader structure"]
impl crate::Readable for MCLKPINDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkpindir::W](W) writer structure"]
impl crate::Writable for MCLKPINDIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKPINDIR to value 0"]
impl crate::Resettable for MCLKPINDIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
