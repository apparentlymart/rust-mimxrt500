#[doc = "Register `FMEASURE_CH_SEL[%s]` reader"]
pub struct R(crate::R<FMEASURE_CH_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMEASURE_CH_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMEASURE_CH_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMEASURE_CH_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMEASURE_CH_SEL[%s]` writer"]
pub struct W(crate::W<FMEASURE_CH_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMEASURE_CH_SEL_SPEC>;
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
impl From<crate::W<FMEASURE_CH_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMEASURE_CH_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMEASURE_SEL` reader - Frequency Measure Channel Selection"]
pub type FMEASURE_SEL_R = crate::FieldReader<u8, FMEASURE_SEL_A>;
#[doc = "Frequency Measure Channel Selection\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMEASURE_SEL_A {
    #[doc = "0: OSC_CLK"]
    FMEASURE_SEL0 = 0,
    #[doc = "1: FRO_DIV16"]
    FMEASURE_SEL1 = 1,
    #[doc = "2: FRO_DIV1"]
    FMEASURE_SEL2 = 2,
    #[doc = "3: Low Power Oscillator Clock (LPOSC)"]
    FMEASURE_SEL3 = 3,
    #[doc = "4: RTC 32 kHz OSC"]
    FMEASURE_SEL4 = 4,
    #[doc = "5: Main SYSCLK"]
    FMEASURE_SEL5 = 5,
    #[doc = "6: FREQME_GPIO_CLK"]
    FMEASURE_SEL6 = 6,
    #[doc = "11: Clock Out"]
    FMEASURE_SEL11 = 11,
}
impl From<FMEASURE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FMEASURE_SEL_A) -> Self {
        variant as _
    }
}
impl FMEASURE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FMEASURE_SEL_A> {
        match self.bits {
            0 => Some(FMEASURE_SEL_A::FMEASURE_SEL0),
            1 => Some(FMEASURE_SEL_A::FMEASURE_SEL1),
            2 => Some(FMEASURE_SEL_A::FMEASURE_SEL2),
            3 => Some(FMEASURE_SEL_A::FMEASURE_SEL3),
            4 => Some(FMEASURE_SEL_A::FMEASURE_SEL4),
            5 => Some(FMEASURE_SEL_A::FMEASURE_SEL5),
            6 => Some(FMEASURE_SEL_A::FMEASURE_SEL6),
            11 => Some(FMEASURE_SEL_A::FMEASURE_SEL11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FMEASURE_SEL0`"]
    #[inline(always)]
    pub fn is_fmeasure_sel0(&self) -> bool {
        *self == FMEASURE_SEL_A::FMEASURE_SEL0
    }
    #[doc = "Checks if the value of the field is `FMEASURE_SEL1`"]
    #[inline(always)]
    pub fn is_fmeasure_sel1(&self) -> bool {
        *self == FMEASURE_SEL_A::FMEASURE_SEL1
    }
    #[doc = "Checks if the value of the field is `FMEASURE_SEL2`"]
    #[inline(always)]
    pub fn is_fmeasure_sel2(&self) -> bool {
        *self == FMEASURE_SEL_A::FMEASURE_SEL2
    }
    #[doc = "Checks if the value of the field is `FMEASURE_SEL3`"]
    #[inline(always)]
    pub fn is_fmeasure_sel3(&self) -> bool {
        *self == FMEASURE_SEL_A::FMEASURE_SEL3
    }
    #[doc = "Checks if the value of the field is `FMEASURE_SEL4`"]
    #[inline(always)]
    pub fn is_fmeasure_sel4(&self) -> bool {
        *self == FMEASURE_SEL_A::FMEASURE_SEL4
    }
    #[doc = "Checks if the value of the field is `FMEASURE_SEL5`"]
    #[inline(always)]
    pub fn is_fmeasure_sel5(&self) -> bool {
        *self == FMEASURE_SEL_A::FMEASURE_SEL5
    }
    #[doc = "Checks if the value of the field is `FMEASURE_SEL6`"]
    #[inline(always)]
    pub fn is_fmeasure_sel6(&self) -> bool {
        *self == FMEASURE_SEL_A::FMEASURE_SEL6
    }
    #[doc = "Checks if the value of the field is `FMEASURE_SEL11`"]
    #[inline(always)]
    pub fn is_fmeasure_sel11(&self) -> bool {
        *self == FMEASURE_SEL_A::FMEASURE_SEL11
    }
}
#[doc = "Field `FMEASURE_SEL` writer - Frequency Measure Channel Selection"]
pub type FMEASURE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FMEASURE_CH_SEL_SPEC, u8, FMEASURE_SEL_A, 5, O>;
impl<'a, const O: u8> FMEASURE_SEL_W<'a, O> {
    #[doc = "OSC_CLK"]
    #[inline(always)]
    pub fn fmeasure_sel0(self) -> &'a mut W {
        self.variant(FMEASURE_SEL_A::FMEASURE_SEL0)
    }
    #[doc = "FRO_DIV16"]
    #[inline(always)]
    pub fn fmeasure_sel1(self) -> &'a mut W {
        self.variant(FMEASURE_SEL_A::FMEASURE_SEL1)
    }
    #[doc = "FRO_DIV1"]
    #[inline(always)]
    pub fn fmeasure_sel2(self) -> &'a mut W {
        self.variant(FMEASURE_SEL_A::FMEASURE_SEL2)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)"]
    #[inline(always)]
    pub fn fmeasure_sel3(self) -> &'a mut W {
        self.variant(FMEASURE_SEL_A::FMEASURE_SEL3)
    }
    #[doc = "RTC 32 kHz OSC"]
    #[inline(always)]
    pub fn fmeasure_sel4(self) -> &'a mut W {
        self.variant(FMEASURE_SEL_A::FMEASURE_SEL4)
    }
    #[doc = "Main SYSCLK"]
    #[inline(always)]
    pub fn fmeasure_sel5(self) -> &'a mut W {
        self.variant(FMEASURE_SEL_A::FMEASURE_SEL5)
    }
    #[doc = "FREQME_GPIO_CLK"]
    #[inline(always)]
    pub fn fmeasure_sel6(self) -> &'a mut W {
        self.variant(FMEASURE_SEL_A::FMEASURE_SEL6)
    }
    #[doc = "Clock Out"]
    #[inline(always)]
    pub fn fmeasure_sel11(self) -> &'a mut W {
        self.variant(FMEASURE_SEL_A::FMEASURE_SEL11)
    }
}
impl R {
    #[doc = "Bits 0:4 - Frequency Measure Channel Selection"]
    #[inline(always)]
    pub fn fmeasure_sel(&self) -> FMEASURE_SEL_R {
        FMEASURE_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Frequency Measure Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fmeasure_sel(&mut self) -> FMEASURE_SEL_W<0> {
        FMEASURE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Measurement Input Channel Multiplexers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmeasure_ch_sel](index.html) module"]
pub struct FMEASURE_CH_SEL_SPEC;
impl crate::RegisterSpec for FMEASURE_CH_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmeasure_ch_sel::R](R) reader structure"]
impl crate::Readable for FMEASURE_CH_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmeasure_ch_sel::W](W) writer structure"]
impl crate::Writable for FMEASURE_CH_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMEASURE_CH_SEL[%s]
to value 0x1f"]
impl crate::Resettable for FMEASURE_CH_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
