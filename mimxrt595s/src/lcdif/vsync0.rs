#[doc = "Register `VSync0` reader"]
pub struct R(crate::R<VSYNC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VSYNC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VSYNC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VSYNC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VSync0` writer"]
pub struct W(crate::W<VSYNC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VSYNC0_SPEC>;
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
impl From<crate::W<VSYNC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VSYNC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Start of the vertical sync pulse."]
pub type START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `START` writer - Start of the vertical sync pulse."]
pub type START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VSYNC0_SPEC, u16, u16, 12, O>;
#[doc = "Field `END` reader - End of the vertical sync pulse."]
pub type END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `END` writer - End of the vertical sync pulse."]
pub type END_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VSYNC0_SPEC, u16, u16, 12, O>;
#[doc = "Field `PULSE` reader - Vertical sync pulse control."]
pub type PULSE_R = crate::BitReader<PULSE_A>;
#[doc = "Vertical sync pulse control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PULSE_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<PULSE_A> for bool {
    #[inline(always)]
    fn from(variant: PULSE_A) -> Self {
        variant as u8 != 0
    }
}
impl PULSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULSE_A {
        match self.bits {
            false => PULSE_A::DISABLE,
            true => PULSE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PULSE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PULSE_A::ENABLE
    }
}
#[doc = "Field `PULSE` writer - Vertical sync pulse control."]
pub type PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VSYNC0_SPEC, PULSE_A, O>;
impl<'a, const O: u8> PULSE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PULSE_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PULSE_A::ENABLE)
    }
}
#[doc = "Field `POLARITY` reader - Polarity of the vertical sync pulse."]
pub type POLARITY_R = crate::BitReader<POLARITY_A>;
#[doc = "Polarity of the vertical sync pulse.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLARITY_A {
    #[doc = "0: Positive"]
    ENABLE = 0,
    #[doc = "1: Active-low"]
    DISABLE = 1,
}
impl From<POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            false => POLARITY_A::ENABLE,
            true => POLARITY_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == POLARITY_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == POLARITY_A::DISABLE
    }
}
#[doc = "Field `POLARITY` writer - Polarity of the vertical sync pulse."]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, VSYNC0_SPEC, POLARITY_A, O>;
impl<'a, const O: u8> POLARITY_W<'a, O> {
    #[doc = "Positive"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(POLARITY_A::ENABLE)
    }
    #[doc = "Active-low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(POLARITY_A::DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:11 - Start of the vertical sync pulse."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - End of the vertical sync pulse."]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - Vertical sync pulse control."]
    #[inline(always)]
    pub fn pulse(&self) -> PULSE_R {
        PULSE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Polarity of the vertical sync pulse."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Start of the vertical sync pulse."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 16:27 - End of the vertical sync pulse."]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> END_W<16> {
        END_W::new(self)
    }
    #[doc = "Bit 30 - Vertical sync pulse control."]
    #[inline(always)]
    #[must_use]
    pub fn pulse(&mut self) -> PULSE_W<30> {
        PULSE_W::new(self)
    }
    #[doc = "Bit 31 - Polarity of the vertical sync pulse."]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<31> {
        POLARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vertical Sync Counters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vsync0](index.html) module"]
pub struct VSYNC0_SPEC;
impl crate::RegisterSpec for VSYNC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vsync0::R](R) reader structure"]
impl crate::Readable for VSYNC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vsync0::W](W) writer structure"]
impl crate::Writable for VSYNC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VSync0 to value 0"]
impl crate::Resettable for VSYNC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
