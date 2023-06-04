#[doc = "Register `SCTFCLKSEL` reader"]
pub struct R(crate::R<SCTFCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTFCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTFCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTFCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTFCLKSEL` writer"]
pub struct W(crate::W<SCTFCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTFCLKSEL_SPEC>;
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
impl From<crate::W<SCTFCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTFCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Select Clock Source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main Clock"]
    MAIN = 0,
    #[doc = "1: Main System PLL Clock"]
    PLL = 1,
    #[doc = "2: SYSPLL0 AUX0_PLL_Clock"]
    AUX0_PLL = 2,
    #[doc = "3: FRO_DIV1 Clock"]
    FRO_192M = 3,
    #[doc = "4: SYSPLL0 AUX1_PLL_Clock"]
    AUX1_PLL = 4,
    #[doc = "5: AUDIO PLL Clock"]
    AUDIO_PLL = 5,
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
            0 => Some(SEL_A::MAIN),
            1 => Some(SEL_A::PLL),
            2 => Some(SEL_A::AUX0_PLL),
            3 => Some(SEL_A::FRO_192M),
            4 => Some(SEL_A::AUX1_PLL),
            5 => Some(SEL_A::AUDIO_PLL),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == SEL_A::MAIN
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `AUX0_PLL`"]
    #[inline(always)]
    pub fn is_aux0_pll(&self) -> bool {
        *self == SEL_A::AUX0_PLL
    }
    #[doc = "Checks if the value of the field is `FRO_192M`"]
    #[inline(always)]
    pub fn is_fro_192m(&self) -> bool {
        *self == SEL_A::FRO_192M
    }
    #[doc = "Checks if the value of the field is `AUX1_PLL`"]
    #[inline(always)]
    pub fn is_aux1_pll(&self) -> bool {
        *self == SEL_A::AUX1_PLL
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL`"]
    #[inline(always)]
    pub fn is_audio_pll(&self) -> bool {
        *self == SEL_A::AUDIO_PLL
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Select Clock Source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTFCLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Main Clock"]
    #[inline(always)]
    pub fn main(self) -> &'a mut W {
        self.variant(SEL_A::MAIN)
    }
    #[doc = "Main System PLL Clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SEL_A::PLL)
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock"]
    #[inline(always)]
    pub fn aux0_pll(self) -> &'a mut W {
        self.variant(SEL_A::AUX0_PLL)
    }
    #[doc = "FRO_DIV1 Clock"]
    #[inline(always)]
    pub fn fro_192m(self) -> &'a mut W {
        self.variant(SEL_A::FRO_192M)
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock"]
    #[inline(always)]
    pub fn aux1_pll(self) -> &'a mut W {
        self.variant(SEL_A::AUX1_PLL)
    }
    #[doc = "AUDIO PLL Clock"]
    #[inline(always)]
    pub fn audio_pll(self) -> &'a mut W {
        self.variant(SEL_A::AUDIO_PLL)
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
#[doc = "SCT Functional Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctfclksel](index.html) module"]
pub struct SCTFCLKSEL_SPEC;
impl crate::RegisterSpec for SCTFCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctfclksel::R](R) reader structure"]
impl crate::Readable for SCTFCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctfclksel::W](W) writer structure"]
impl crate::Writable for SCTFCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTFCLKSEL to value 0x07"]
impl crate::Resettable for SCTFCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
