#[doc = "Register `CLKOUTSEL1` reader"]
pub struct R(crate::R<CLKOUTSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKOUTSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKOUTSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKOUTSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKOUTSEL1` writer"]
pub struct W(crate::W<CLKOUTSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKOUTSEL1_SPEC>;
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
impl From<crate::W<CLKOUTSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKOUTSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Clock Out Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Clock Out Source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: CLKOUTSEL0 Multiplexed Output"]
    CLKOUTSEL0_MUX_OUT = 0,
    #[doc = "1: Main System PLL Clock"]
    MAIN_PLL = 1,
    #[doc = "2: SYSPLL0 AUX0_PLL_Clock"]
    SYSPLL0_AUX0_PLL = 2,
    #[doc = "3: DSP PLL Clock"]
    DSP_PLL = 3,
    #[doc = "4: SYSPLL0 AUX1_PLL_Clock"]
    SYSPLL0_AUX1_PLL = 4,
    #[doc = "5: AUDIO PLL Clock"]
    AUDIO_PLL = 5,
    #[doc = "6: 32 KHz RTC Clock"]
    RTC_32KHZ = 6,
    #[doc = "7: None, output gated to reduce power"]
    NONE = 7,
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
            0 => SEL_A::CLKOUTSEL0_MUX_OUT,
            1 => SEL_A::MAIN_PLL,
            2 => SEL_A::SYSPLL0_AUX0_PLL,
            3 => SEL_A::DSP_PLL,
            4 => SEL_A::SYSPLL0_AUX1_PLL,
            5 => SEL_A::AUDIO_PLL,
            6 => SEL_A::RTC_32KHZ,
            7 => SEL_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL0_MUX_OUT`"]
    #[inline(always)]
    pub fn is_clkoutsel0_mux_out(&self) -> bool {
        *self == SEL_A::CLKOUTSEL0_MUX_OUT
    }
    #[doc = "Checks if the value of the field is `MAIN_PLL`"]
    #[inline(always)]
    pub fn is_main_pll(&self) -> bool {
        *self == SEL_A::MAIN_PLL
    }
    #[doc = "Checks if the value of the field is `SYSPLL0_AUX0_PLL`"]
    #[inline(always)]
    pub fn is_syspll0_aux0_pll(&self) -> bool {
        *self == SEL_A::SYSPLL0_AUX0_PLL
    }
    #[doc = "Checks if the value of the field is `DSP_PLL`"]
    #[inline(always)]
    pub fn is_dsp_pll(&self) -> bool {
        *self == SEL_A::DSP_PLL
    }
    #[doc = "Checks if the value of the field is `SYSPLL0_AUX1_PLL`"]
    #[inline(always)]
    pub fn is_syspll0_aux1_pll(&self) -> bool {
        *self == SEL_A::SYSPLL0_AUX1_PLL
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL`"]
    #[inline(always)]
    pub fn is_audio_pll(&self) -> bool {
        *self == SEL_A::AUDIO_PLL
    }
    #[doc = "Checks if the value of the field is `RTC_32KHZ`"]
    #[inline(always)]
    pub fn is_rtc_32khz(&self) -> bool {
        *self == SEL_A::RTC_32KHZ
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Clock Out Source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLKOUTSEL1_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "CLKOUTSEL0 Multiplexed Output"]
    #[inline(always)]
    pub fn clkoutsel0_mux_out(self) -> &'a mut W {
        self.variant(SEL_A::CLKOUTSEL0_MUX_OUT)
    }
    #[doc = "Main System PLL Clock"]
    #[inline(always)]
    pub fn main_pll(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_PLL)
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock"]
    #[inline(always)]
    pub fn syspll0_aux0_pll(self) -> &'a mut W {
        self.variant(SEL_A::SYSPLL0_AUX0_PLL)
    }
    #[doc = "DSP PLL Clock"]
    #[inline(always)]
    pub fn dsp_pll(self) -> &'a mut W {
        self.variant(SEL_A::DSP_PLL)
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock"]
    #[inline(always)]
    pub fn syspll0_aux1_pll(self) -> &'a mut W {
        self.variant(SEL_A::SYSPLL0_AUX1_PLL)
    }
    #[doc = "AUDIO PLL Clock"]
    #[inline(always)]
    pub fn audio_pll(self) -> &'a mut W {
        self.variant(SEL_A::AUDIO_PLL)
    }
    #[doc = "32 KHz RTC Clock"]
    #[inline(always)]
    pub fn rtc_32khz(self) -> &'a mut W {
        self.variant(SEL_A::RTC_32KHZ)
    }
    #[doc = "None, output gated to reduce power"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Out Source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Out Source"]
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
#[doc = "CLKOUT Clock Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutsel1](index.html) module"]
pub struct CLKOUTSEL1_SPEC;
impl crate::RegisterSpec for CLKOUTSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkoutsel1::R](R) reader structure"]
impl crate::Readable for CLKOUTSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkoutsel1::W](W) writer structure"]
impl crate::Writable for CLKOUTSEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKOUTSEL1 to value 0x07"]
impl crate::Resettable for CLKOUTSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
