#[doc = "Register `CLKDIV` reader"]
pub struct R(crate::R<CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV` writer"]
pub struct W(crate::W<CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV_SPEC>;
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
impl From<crate::W<CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVVAL` reader - Divider Value"]
pub type DIVVAL_R = crate::FieldReader<u16, DIVVAL_A>;
#[doc = "Divider Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DIVVAL_A {
    #[doc = "0: FCLK is used directly by the I2C."]
    FCLKUNDIVIDED = 0,
    #[doc = "1: FCLK is divided by 2 before being used by the I2C."]
    FCLKDIV2 = 1,
    #[doc = "2: FCLK is divided by 3 before being used by the I2C."]
    FCLKDIV3 = 2,
    #[doc = "65535: FCLK is divided by 65,536 before being used by the I2C."]
    FCLKDIV65K = 65535,
}
impl From<DIVVAL_A> for u16 {
    #[inline(always)]
    fn from(variant: DIVVAL_A) -> Self {
        variant as _
    }
}
impl DIVVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVVAL_A> {
        match self.bits {
            0 => Some(DIVVAL_A::FCLKUNDIVIDED),
            1 => Some(DIVVAL_A::FCLKDIV2),
            2 => Some(DIVVAL_A::FCLKDIV3),
            65535 => Some(DIVVAL_A::FCLKDIV65K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FCLKUNDIVIDED`"]
    #[inline(always)]
    pub fn is_fclkundivided(&self) -> bool {
        *self == DIVVAL_A::FCLKUNDIVIDED
    }
    #[doc = "Checks if the value of the field is `FCLKDIV2`"]
    #[inline(always)]
    pub fn is_fclkdiv2(&self) -> bool {
        *self == DIVVAL_A::FCLKDIV2
    }
    #[doc = "Checks if the value of the field is `FCLKDIV3`"]
    #[inline(always)]
    pub fn is_fclkdiv3(&self) -> bool {
        *self == DIVVAL_A::FCLKDIV3
    }
    #[doc = "Checks if the value of the field is `FCLKDIV65K`"]
    #[inline(always)]
    pub fn is_fclkdiv65k(&self) -> bool {
        *self == DIVVAL_A::FCLKDIV65K
    }
}
#[doc = "Field `DIVVAL` writer - Divider Value"]
pub type DIVVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKDIV_SPEC, u16, DIVVAL_A, 16, O>;
impl<'a, const O: u8> DIVVAL_W<'a, O> {
    #[doc = "FCLK is used directly by the I2C."]
    #[inline(always)]
    pub fn fclkundivided(self) -> &'a mut W {
        self.variant(DIVVAL_A::FCLKUNDIVIDED)
    }
    #[doc = "FCLK is divided by 2 before being used by the I2C."]
    #[inline(always)]
    pub fn fclkdiv2(self) -> &'a mut W {
        self.variant(DIVVAL_A::FCLKDIV2)
    }
    #[doc = "FCLK is divided by 3 before being used by the I2C."]
    #[inline(always)]
    pub fn fclkdiv3(self) -> &'a mut W {
        self.variant(DIVVAL_A::FCLKDIV3)
    }
    #[doc = "FCLK is divided by 65,536 before being used by the I2C."]
    #[inline(always)]
    pub fn fclkdiv65k(self) -> &'a mut W {
        self.variant(DIVVAL_A::FCLKDIV65K)
    }
}
impl R {
    #[doc = "Bits 0:15 - Divider Value"]
    #[inline(always)]
    pub fn divval(&self) -> DIVVAL_R {
        DIVVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn divval(&mut self) -> DIVVAL_W<0> {
        DIVVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](index.html) module"]
pub struct CLKDIV_SPEC;
impl crate::RegisterSpec for CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv::R](R) reader structure"]
impl crate::Readable for CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](W) writer structure"]
impl crate::Writable for CLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
