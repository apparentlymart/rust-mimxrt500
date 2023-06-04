#[doc = "Register `OSC32KHZCTL0` reader"]
pub struct R(crate::R<OSC32KHZCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC32KHZCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC32KHZCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC32KHZCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC32KHZCTL0` writer"]
pub struct W(crate::W<OSC32KHZCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC32KHZCTL0_SPEC>;
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
impl From<crate::W<OSC32KHZCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC32KHZCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA32KHZ` reader - 32 KHz Oscillator Enable"]
pub type ENA32KHZ_R = crate::BitReader<ENA32KHZ_A>;
#[doc = "32 KHz Oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA32KHZ_A {
    #[doc = "0: Disable oscillator"]
    ENA32KHZ_DISABLE = 0,
    #[doc = "1: Enable oscillator"]
    ENA32KHZ_ENABLE = 1,
}
impl From<ENA32KHZ_A> for bool {
    #[inline(always)]
    fn from(variant: ENA32KHZ_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA32KHZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA32KHZ_A {
        match self.bits {
            false => ENA32KHZ_A::ENA32KHZ_DISABLE,
            true => ENA32KHZ_A::ENA32KHZ_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENA32KHZ_DISABLE`"]
    #[inline(always)]
    pub fn is_ena32khz_disable(&self) -> bool {
        *self == ENA32KHZ_A::ENA32KHZ_DISABLE
    }
    #[doc = "Checks if the value of the field is `ENA32KHZ_ENABLE`"]
    #[inline(always)]
    pub fn is_ena32khz_enable(&self) -> bool {
        *self == ENA32KHZ_A::ENA32KHZ_ENABLE
    }
}
#[doc = "Field `ENA32KHZ` writer - 32 KHz Oscillator Enable"]
pub type ENA32KHZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC32KHZCTL0_SPEC, ENA32KHZ_A, O>;
impl<'a, const O: u8> ENA32KHZ_W<'a, O> {
    #[doc = "Disable oscillator"]
    #[inline(always)]
    pub fn ena32khz_disable(self) -> &'a mut W {
        self.variant(ENA32KHZ_A::ENA32KHZ_DISABLE)
    }
    #[doc = "Enable oscillator"]
    #[inline(always)]
    pub fn ena32khz_enable(self) -> &'a mut W {
        self.variant(ENA32KHZ_A::ENA32KHZ_ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - 32 KHz Oscillator Enable"]
    #[inline(always)]
    pub fn ena32khz(&self) -> ENA32KHZ_R {
        ENA32KHZ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 32 KHz Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ena32khz(&mut self) -> ENA32KHZ_W<0> {
        ENA32KHZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32 KHz Oscillator Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc32khzctl0](index.html) module"]
pub struct OSC32KHZCTL0_SPEC;
impl crate::RegisterSpec for OSC32KHZCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc32khzctl0::R](R) reader structure"]
impl crate::Readable for OSC32KHZCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc32khzctl0::W](W) writer structure"]
impl crate::Writable for OSC32KHZCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC32KHZCTL0 to value 0"]
impl crate::Resettable for OSC32KHZCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
