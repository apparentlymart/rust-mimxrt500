#[doc = "Register `ADC0FCLKSEL0` reader"]
pub struct R(crate::R<ADC0FCLKSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0FCLKSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0FCLKSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0FCLKSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0FCLKSEL0` writer"]
pub struct W(crate::W<ADC0FCLKSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0FCLKSEL0_SPEC>;
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
impl From<crate::W<ADC0FCLKSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0FCLKSEL0_SPEC>) -> Self {
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
    #[doc = "1: Low Power Oscillator Clock (LPOSC)"]
    LPOSC = 1,
    #[doc = "2: FRO_DIV4"]
    FRO_DIV4 = 2,
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
            1 => Some(SEL_A::LPOSC),
            2 => Some(SEL_A::FRO_DIV4),
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
    #[doc = "Checks if the value of the field is `FRO_DIV4`"]
    #[inline(always)]
    pub fn is_fro_div4(&self) -> bool {
        *self == SEL_A::FRO_DIV4
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Select Clock Source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC0FCLKSEL0_SPEC, u8, SEL_A, 3, O>;
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
    #[doc = "FRO_DIV4"]
    #[inline(always)]
    pub fn fro_div4(self) -> &'a mut W {
        self.variant(SEL_A::FRO_DIV4)
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
#[doc = "ADC0 Functional Clock Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0fclksel0](index.html) module"]
pub struct ADC0FCLKSEL0_SPEC;
impl crate::RegisterSpec for ADC0FCLKSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0fclksel0::R](R) reader structure"]
impl crate::Readable for ADC0FCLKSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0fclksel0::W](W) writer structure"]
impl crate::Writable for ADC0FCLKSEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0FCLKSEL0 to value 0x07"]
impl crate::Resettable for ADC0FCLKSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
