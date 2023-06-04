#[doc = "Register `DSPCPUCLKSELB` reader"]
pub struct R(crate::R<DSPCPUCLKSELB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSPCPUCLKSELB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSPCPUCLKSELB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSPCPUCLKSELB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSPCPUCLKSELB` writer"]
pub struct W(crate::W<DSPCPUCLKSELB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSPCPUCLKSELB_SPEC>;
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
impl From<crate::W<DSPCPUCLKSELB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSPCPUCLKSELB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Main Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Main Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: MAINCLKSELA 1st Stage Clock"]
    MAINCLKSELA = 0,
    #[doc = "1: Main System PLL Clock"]
    MAIN_PLL = 1,
    #[doc = "2: DSP System PLL Clock"]
    DSP_PLL = 2,
    #[doc = "3: RTC 32 KHz Clock"]
    RTC_32KHZ = 3,
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
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::MAINCLKSELA,
            1 => SEL_A::MAIN_PLL,
            2 => SEL_A::DSP_PLL,
            3 => SEL_A::RTC_32KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAINCLKSELA`"]
    #[inline(always)]
    pub fn is_mainclksela(&self) -> bool {
        *self == SEL_A::MAINCLKSELA
    }
    #[doc = "Checks if the value of the field is `MAIN_PLL`"]
    #[inline(always)]
    pub fn is_main_pll(&self) -> bool {
        *self == SEL_A::MAIN_PLL
    }
    #[doc = "Checks if the value of the field is `DSP_PLL`"]
    #[inline(always)]
    pub fn is_dsp_pll(&self) -> bool {
        *self == SEL_A::DSP_PLL
    }
    #[doc = "Checks if the value of the field is `RTC_32KHZ`"]
    #[inline(always)]
    pub fn is_rtc_32khz(&self) -> bool {
        *self == SEL_A::RTC_32KHZ
    }
}
#[doc = "Field `SEL` writer - Main Clock Source"]
pub type SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DSPCPUCLKSELB_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "MAINCLKSELA 1st Stage Clock"]
    #[inline(always)]
    pub fn mainclksela(self) -> &'a mut W {
        self.variant(SEL_A::MAINCLKSELA)
    }
    #[doc = "Main System PLL Clock"]
    #[inline(always)]
    pub fn main_pll(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_PLL)
    }
    #[doc = "DSP System PLL Clock"]
    #[inline(always)]
    pub fn dsp_pll(self) -> &'a mut W {
        self.variant(SEL_A::DSP_PLL)
    }
    #[doc = "RTC 32 KHz Clock"]
    #[inline(always)]
    pub fn rtc_32khz(self) -> &'a mut W {
        self.variant(SEL_A::RTC_32KHZ)
    }
}
impl R {
    #[doc = "Bits 0:1 - Main Clock Source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Main Clock Source"]
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
#[doc = "DSP CPU Clock Select B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dspcpuclkselb](index.html) module"]
pub struct DSPCPUCLKSELB_SPEC;
impl crate::RegisterSpec for DSPCPUCLKSELB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dspcpuclkselb::R](R) reader structure"]
impl crate::Readable for DSPCPUCLKSELB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dspcpuclkselb::W](W) writer structure"]
impl crate::Writable for DSPCPUCLKSELB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSPCPUCLKSELB to value 0"]
impl crate::Resettable for DSPCPUCLKSELB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
