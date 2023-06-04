#[doc = "Register `SYSPLL0CLKSEL` reader"]
pub struct R(crate::R<SYSPLL0CLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLL0CLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLL0CLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLL0CLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLL0CLKSEL` writer"]
pub struct W(crate::W<SYSPLL0CLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLL0CLKSEL_SPEC>;
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
impl From<crate::W<SYSPLL0CLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLL0CLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - System PLL0 Reference Input Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "System PLL0 Reference Input Clock Source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO_DIV8 Clock"]
    FRRO_DIV8 = 0,
    #[doc = "1: OSC_CLK clock"]
    OSC_CLK = 1,
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
            0 => Some(SEL_A::FRRO_DIV8),
            1 => Some(SEL_A::OSC_CLK),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRRO_DIV8`"]
    #[inline(always)]
    pub fn is_frro_div8(&self) -> bool {
        *self == SEL_A::FRRO_DIV8
    }
    #[doc = "Checks if the value of the field is `OSC_CLK`"]
    #[inline(always)]
    pub fn is_osc_clk(&self) -> bool {
        *self == SEL_A::OSC_CLK
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - System PLL0 Reference Input Clock Source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLL0CLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "FRO_DIV8 Clock"]
    #[inline(always)]
    pub fn frro_div8(self) -> &'a mut W {
        self.variant(SEL_A::FRRO_DIV8)
    }
    #[doc = "OSC_CLK clock"]
    #[inline(always)]
    pub fn osc_clk(self) -> &'a mut W {
        self.variant(SEL_A::OSC_CLK)
    }
    #[doc = "None, output gated to reduce power"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - System PLL0 Reference Input Clock Source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System PLL0 Reference Input Clock Source"]
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
#[doc = "System PLL 0 Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspll0clksel](index.html) module"]
pub struct SYSPLL0CLKSEL_SPEC;
impl crate::RegisterSpec for SYSPLL0CLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspll0clksel::R](R) reader structure"]
impl crate::Readable for SYSPLL0CLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspll0clksel::W](W) writer structure"]
impl crate::Writable for SYSPLL0CLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSPLL0CLKSEL to value 0x07"]
impl crate::Resettable for SYSPLL0CLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
