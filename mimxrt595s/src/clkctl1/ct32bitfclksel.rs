#[doc = "Register `CT32BITFCLKSEL[%s]` reader"]
pub struct R(crate::R<CT32BITFCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CT32BITFCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CT32BITFCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CT32BITFCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CT32BITFCLKSEL[%s]` writer"]
pub struct W(crate::W<CT32BITFCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CT32BITFCLKSEL_SPEC>;
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
impl From<crate::W<CT32BITFCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CT32BITFCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - CT32BIT bit timer 0 Functional Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "CT32BIT bit timer 0 Functional Clock Source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main Clock"]
    MAIN = 0,
    #[doc = "1: FRO_DIV1 Clock"]
    FRO = 1,
    #[doc = "2: Audio PLL Clock"]
    AUDIO_PLL = 2,
    #[doc = "3: Master Clock In"]
    MASTER_CLOCK = 3,
    #[doc = "4: 32 KHZ Wake Clock"]
    WAKE_32KHZ = 4,
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
            0 => Some(SEL_A::MAIN),
            1 => Some(SEL_A::FRO),
            2 => Some(SEL_A::AUDIO_PLL),
            3 => Some(SEL_A::MASTER_CLOCK),
            4 => Some(SEL_A::WAKE_32KHZ),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == SEL_A::MAIN
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline(always)]
    pub fn is_fro(&self) -> bool {
        *self == SEL_A::FRO
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL`"]
    #[inline(always)]
    pub fn is_audio_pll(&self) -> bool {
        *self == SEL_A::AUDIO_PLL
    }
    #[doc = "Checks if the value of the field is `MASTER_CLOCK`"]
    #[inline(always)]
    pub fn is_master_clock(&self) -> bool {
        *self == SEL_A::MASTER_CLOCK
    }
    #[doc = "Checks if the value of the field is `WAKE_32KHZ`"]
    #[inline(always)]
    pub fn is_wake_32khz(&self) -> bool {
        *self == SEL_A::WAKE_32KHZ
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - CT32BIT bit timer 0 Functional Clock Source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CT32BITFCLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Main Clock"]
    #[inline(always)]
    pub fn main(self) -> &'a mut W {
        self.variant(SEL_A::MAIN)
    }
    #[doc = "FRO_DIV1 Clock"]
    #[inline(always)]
    pub fn fro(self) -> &'a mut W {
        self.variant(SEL_A::FRO)
    }
    #[doc = "Audio PLL Clock"]
    #[inline(always)]
    pub fn audio_pll(self) -> &'a mut W {
        self.variant(SEL_A::AUDIO_PLL)
    }
    #[doc = "Master Clock In"]
    #[inline(always)]
    pub fn master_clock(self) -> &'a mut W {
        self.variant(SEL_A::MASTER_CLOCK)
    }
    #[doc = "32 KHZ Wake Clock"]
    #[inline(always)]
    pub fn wake_32khz(self) -> &'a mut W {
        self.variant(SEL_A::WAKE_32KHZ)
    }
    #[doc = "None, output gated to reduce power"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - CT32BIT bit timer 0 Functional Clock Source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CT32BIT bit timer 0 Functional Clock Source"]
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
#[doc = "CT32BIT bit timer index Functional Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ct32bitfclksel](index.html) module"]
pub struct CT32BITFCLKSEL_SPEC;
impl crate::RegisterSpec for CT32BITFCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ct32bitfclksel::R](R) reader structure"]
impl crate::Readable for CT32BITFCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ct32bitfclksel::W](W) writer structure"]
impl crate::Writable for CT32BITFCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CT32BITFCLKSEL[%s]
to value 0x07"]
impl crate::Resettable for CT32BITFCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
