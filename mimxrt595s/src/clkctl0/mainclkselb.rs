#[doc = "Register `MAINCLKSELB` reader"]
pub struct R(crate::R<MAINCLKSELB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKSELB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKSELB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKSELB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKSELB` writer"]
pub struct W(crate::W<MAINCLKSELB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKSELB_SPEC>;
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
impl From<crate::W<MAINCLKSELB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKSELB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Main Clock Source Selection"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Main Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: MAINCLKSELA 1st Stage Clock"]
    MAINCLKSELA = 0,
    #[doc = "1: Main System PLL Clock"]
    SYSPLL = 1,
    #[doc = "2: RTC 32 KHz Clock"]
    RTC32KHZ = 2,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::MAINCLKSELA),
            1 => Some(SEL_A::SYSPLL),
            2 => Some(SEL_A::RTC32KHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAINCLKSELA`"]
    #[inline(always)]
    pub fn is_mainclksela(&self) -> bool {
        *self == SEL_A::MAINCLKSELA
    }
    #[doc = "Checks if the value of the field is `SYSPLL`"]
    #[inline(always)]
    pub fn is_syspll(&self) -> bool {
        *self == SEL_A::SYSPLL
    }
    #[doc = "Checks if the value of the field is `RTC32KHZ`"]
    #[inline(always)]
    pub fn is_rtc32khz(&self) -> bool {
        *self == SEL_A::RTC32KHZ
    }
}
#[doc = "Field `SEL` writer - Main Clock Source Selection"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAINCLKSELB_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "MAINCLKSELA 1st Stage Clock"]
    #[inline(always)]
    pub fn mainclksela(self) -> &'a mut W {
        self.variant(SEL_A::MAINCLKSELA)
    }
    #[doc = "Main System PLL Clock"]
    #[inline(always)]
    pub fn syspll(self) -> &'a mut W {
        self.variant(SEL_A::SYSPLL)
    }
    #[doc = "RTC 32 KHz Clock"]
    #[inline(always)]
    pub fn rtc32khz(self) -> &'a mut W {
        self.variant(SEL_A::RTC32KHZ)
    }
}
impl R {
    #[doc = "Bits 0:1 - Main Clock Source Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Main Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Select B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclkselb](index.html) module"]
pub struct MAINCLKSELB_SPEC;
impl crate::RegisterSpec for MAINCLKSELB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclkselb::R](R) reader structure"]
impl crate::Readable for MAINCLKSELB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclkselb::W](W) writer structure"]
impl crate::Writable for MAINCLKSELB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAINCLKSELB to value 0"]
impl crate::Resettable for MAINCLKSELB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
