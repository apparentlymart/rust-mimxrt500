#[doc = "Register `CLKOUTSEL0` reader"]
pub struct R(crate::R<CLKOUTSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKOUTSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKOUTSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKOUTSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKOUTSEL0` writer"]
pub struct W(crate::W<CLKOUTSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKOUTSEL0_SPEC>;
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
impl From<crate::W<CLKOUTSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKOUTSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Clock Output Select 1st Stage"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Clock Output Select 1st Stage\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: OSC_CLK Clock"]
    OSC_CLK = 0,
    #[doc = "1: Low Power Oscillator Clock (LPOSC)"]
    LPOSC = 1,
    #[doc = "2: FRO_DIV2 Clock"]
    FRO = 2,
    #[doc = "3: Main Clock"]
    MAIN = 3,
    #[doc = "4: DSP Main Clock"]
    DSP_MAIN = 4,
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
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::OSC_CLK),
            1 => Some(SEL_A::LPOSC),
            2 => Some(SEL_A::FRO),
            3 => Some(SEL_A::MAIN),
            4 => Some(SEL_A::DSP_MAIN),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OSC_CLK`"]
    #[inline(always)]
    pub fn is_osc_clk(&self) -> bool {
        *self == SEL_A::OSC_CLK
    }
    #[doc = "Checks if the value of the field is `LPOSC`"]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == SEL_A::LPOSC
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline(always)]
    pub fn is_fro(&self) -> bool {
        *self == SEL_A::FRO
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == SEL_A::MAIN
    }
    #[doc = "Checks if the value of the field is `DSP_MAIN`"]
    #[inline(always)]
    pub fn is_dsp_main(&self) -> bool {
        *self == SEL_A::DSP_MAIN
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Clock Output Select 1st Stage"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKOUTSEL0_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "OSC_CLK Clock"]
    #[inline(always)]
    pub fn osc_clk(self) -> &'a mut W {
        self.variant(SEL_A::OSC_CLK)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)"]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut W {
        self.variant(SEL_A::LPOSC)
    }
    #[doc = "FRO_DIV2 Clock"]
    #[inline(always)]
    pub fn fro(self) -> &'a mut W {
        self.variant(SEL_A::FRO)
    }
    #[doc = "Main Clock"]
    #[inline(always)]
    pub fn main(self) -> &'a mut W {
        self.variant(SEL_A::MAIN)
    }
    #[doc = "DSP Main Clock"]
    #[inline(always)]
    pub fn dsp_main(self) -> &'a mut W {
        self.variant(SEL_A::DSP_MAIN)
    }
    #[doc = "None, output gated to reduce power"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Output Select 1st Stage"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Output Select 1st Stage"]
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
#[doc = "CLKOUT Clock Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutsel0](index.html) module"]
pub struct CLKOUTSEL0_SPEC;
impl crate::RegisterSpec for CLKOUTSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkoutsel0::R](R) reader structure"]
impl crate::Readable for CLKOUTSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkoutsel0::W](W) writer structure"]
impl crate::Writable for CLKOUTSEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKOUTSEL0 to value 0x07"]
impl crate::Resettable for CLKOUTSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
