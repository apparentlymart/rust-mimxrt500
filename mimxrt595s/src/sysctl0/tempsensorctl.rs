#[doc = "Register `TEMPSENSORCTL` reader"]
pub struct R(crate::R<TEMPSENSORCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPSENSORCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPSENSORCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPSENSORCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPSENSORCTL` writer"]
pub struct W(crate::W<TEMPSENSORCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPSENSORCTL_SPEC>;
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
impl From<crate::W<TEMPSENSORCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPSENSORCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSSRC` reader - Temperature Sensor Source"]
pub type TSSRC_R = crate::BitReader<TSSRC_A>;
#[doc = "Temperature Sensor Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSSRC_A {
    #[doc = "0: ADC Built-in Temperature Sensor"]
    DISABLE = 0,
    #[doc = "1: PMC Temperature Sensor"]
    ENABLE = 1,
}
impl From<TSSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TSSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl TSSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSSRC_A {
        match self.bits {
            false => TSSRC_A::DISABLE,
            true => TSSRC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSSRC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSSRC_A::ENABLE
    }
}
#[doc = "Field `TSSRC` writer - Temperature Sensor Source"]
pub type TSSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEMPSENSORCTL_SPEC, TSSRC_A, O>;
impl<'a, const O: u8> TSSRC_W<'a, O> {
    #[doc = "ADC Built-in Temperature Sensor"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSSRC_A::DISABLE)
    }
    #[doc = "PMC Temperature Sensor"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSSRC_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature Sensor Source"]
    #[inline(always)]
    pub fn tssrc(&self) -> TSSRC_R {
        TSSRC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor Source"]
    #[inline(always)]
    #[must_use]
    pub fn tssrc(&mut self) -> TSSRC_W<0> {
        TSSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature Sensor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsensorctl](index.html) module"]
pub struct TEMPSENSORCTL_SPEC;
impl crate::RegisterSpec for TEMPSENSORCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempsensorctl::R](R) reader structure"]
impl crate::Readable for TEMPSENSORCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempsensorctl::W](W) writer structure"]
impl crate::Writable for TEMPSENSORCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPSENSORCTL to value 0"]
impl crate::Resettable for TEMPSENSORCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
