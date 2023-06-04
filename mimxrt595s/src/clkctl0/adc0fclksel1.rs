#[doc = "Register `ADC0FCLKSEL1` reader"]
pub struct R(crate::R<ADC0FCLKSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0FCLKSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0FCLKSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0FCLKSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0FCLKSEL1` writer"]
pub struct W(crate::W<ADC0FCLKSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0FCLKSEL1_SPEC>;
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
impl From<crate::W<ADC0FCLKSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0FCLKSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Select Clock Source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: ADC0FCLKSEL0 Multiplexed Output"]
    ADC0FCLKSEL0_MUX = 0,
    #[doc = "1: SYSPLL0 MAIN_CLK (PFD0 Output)"]
    SYSPLL0_MAIN = 1,
    #[doc = "2: SYSPLL0 AUX0_PLL_Clock"]
    SYSPLL0_AUX0_PLL = 2,
    #[doc = "3: SYSPLL0 AUX1_PLL_Clock"]
    SYSPLL0_AUX1_PLL = 3,
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
            0 => Some(SEL_A::ADC0FCLKSEL0_MUX),
            1 => Some(SEL_A::SYSPLL0_MAIN),
            2 => Some(SEL_A::SYSPLL0_AUX0_PLL),
            3 => Some(SEL_A::SYSPLL0_AUX1_PLL),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC0FCLKSEL0_MUX`"]
    #[inline(always)]
    pub fn is_adc0fclksel0_mux(&self) -> bool {
        *self == SEL_A::ADC0FCLKSEL0_MUX
    }
    #[doc = "Checks if the value of the field is `SYSPLL0_MAIN`"]
    #[inline(always)]
    pub fn is_syspll0_main(&self) -> bool {
        *self == SEL_A::SYSPLL0_MAIN
    }
    #[doc = "Checks if the value of the field is `SYSPLL0_AUX0_PLL`"]
    #[inline(always)]
    pub fn is_syspll0_aux0_pll(&self) -> bool {
        *self == SEL_A::SYSPLL0_AUX0_PLL
    }
    #[doc = "Checks if the value of the field is `SYSPLL0_AUX1_PLL`"]
    #[inline(always)]
    pub fn is_syspll0_aux1_pll(&self) -> bool {
        *self == SEL_A::SYSPLL0_AUX1_PLL
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Select Clock Source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC0FCLKSEL1_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "ADC0FCLKSEL0 Multiplexed Output"]
    #[inline(always)]
    pub fn adc0fclksel0_mux(self) -> &'a mut W {
        self.variant(SEL_A::ADC0FCLKSEL0_MUX)
    }
    #[doc = "SYSPLL0 MAIN_CLK (PFD0 Output)"]
    #[inline(always)]
    pub fn syspll0_main(self) -> &'a mut W {
        self.variant(SEL_A::SYSPLL0_MAIN)
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock"]
    #[inline(always)]
    pub fn syspll0_aux0_pll(self) -> &'a mut W {
        self.variant(SEL_A::SYSPLL0_AUX0_PLL)
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock"]
    #[inline(always)]
    pub fn syspll0_aux1_pll(self) -> &'a mut W {
        self.variant(SEL_A::SYSPLL0_AUX1_PLL)
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
#[doc = "ADC0 Functional Clock Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0fclksel1](index.html) module"]
pub struct ADC0FCLKSEL1_SPEC;
impl crate::RegisterSpec for ADC0FCLKSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0fclksel1::R](R) reader structure"]
impl crate::Readable for ADC0FCLKSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0fclksel1::W](W) writer structure"]
impl crate::Writable for ADC0FCLKSEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0FCLKSEL1 to value 0x07"]
impl crate::Resettable for ADC0FCLKSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
