#[doc = "Register `USBHSFCLKSEL` reader"]
pub struct R(crate::R<USBHSFCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHSFCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHSFCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHSFCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHSFCLKSEL` writer"]
pub struct W(crate::W<USBHSFCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHSFCLKSEL_SPEC>;
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
impl From<crate::W<USBHSFCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHSFCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Select Clock Source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: OSC_CLK Clock"]
    OSC_CLK = 0,
    #[doc = "1: Main Clock"]
    MAIN = 1,
    #[doc = "3: AUX0_PLL_CLOCK"]
    AUX0_PLL_CLOCK = 3,
    #[doc = "7: None; this may be selected to reduce power when no output is needed."]
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
            1 => Some(SEL_A::MAIN),
            3 => Some(SEL_A::AUX0_PLL_CLOCK),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OSC_CLK`"]
    #[inline(always)]
    pub fn is_osc_clk(&self) -> bool {
        *self == SEL_A::OSC_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == SEL_A::MAIN
    }
    #[doc = "Checks if the value of the field is `AUX0_PLL_CLOCK`"]
    #[inline(always)]
    pub fn is_aux0_pll_clock(&self) -> bool {
        *self == SEL_A::AUX0_PLL_CLOCK
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Select Clock Source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBHSFCLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "OSC_CLK Clock"]
    #[inline(always)]
    pub fn osc_clk(self) -> &'a mut W {
        self.variant(SEL_A::OSC_CLK)
    }
    #[doc = "Main Clock"]
    #[inline(always)]
    pub fn main(self) -> &'a mut W {
        self.variant(SEL_A::MAIN)
    }
    #[doc = "AUX0_PLL_CLOCK"]
    #[inline(always)]
    pub fn aux0_pll_clock(self) -> &'a mut W {
        self.variant(SEL_A::AUX0_PLL_CLOCK)
    }
    #[doc = "None; this may be selected to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Select Clock Source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select Clock Source"]
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
#[doc = "High Speed USB Functional Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhsfclksel](index.html) module"]
pub struct USBHSFCLKSEL_SPEC;
impl crate::RegisterSpec for USBHSFCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhsfclksel::R](R) reader structure"]
impl crate::Readable for USBHSFCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhsfclksel::W](W) writer structure"]
impl crate::Writable for USBHSFCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBHSFCLKSEL to value 0x07"]
impl crate::Resettable for USBHSFCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
