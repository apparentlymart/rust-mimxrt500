#[doc = "Register `TIMEOUT` reader"]
pub struct R(crate::R<TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMEOUT` writer"]
pub struct W(crate::W<TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUT_SPEC>;
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
impl From<crate::W<TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOMIN` reader - Time-out time value, the bottom 4 bits"]
pub type TOMIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOMIN` writer - Time-out time value, the bottom 4 bits"]
pub type TOMIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMEOUT_SPEC, u8, u8, 4, O>;
#[doc = "Field `TO` reader - Time-out time value"]
pub type TO_R = crate::FieldReader<u16, TO_A>;
#[doc = "Time-out time value\n\nValue on reset: 4095"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TO_A {
    #[doc = "0: A time-out will occur after 16 counts of the I2C function clock."]
    TIMEOUT16 = 0,
    #[doc = "1: A time-out will occur after 32 counts of the I2C function clock."]
    TIMEOUT32 = 1,
    #[doc = "4095: A time-out will occur after 65,536 counts of the I2C function clock."]
    TIMEOUT65K = 4095,
}
impl From<TO_A> for u16 {
    #[inline(always)]
    fn from(variant: TO_A) -> Self {
        variant as _
    }
}
impl TO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TO_A> {
        match self.bits {
            0 => Some(TO_A::TIMEOUT16),
            1 => Some(TO_A::TIMEOUT32),
            4095 => Some(TO_A::TIMEOUT65K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMEOUT16`"]
    #[inline(always)]
    pub fn is_timeout16(&self) -> bool {
        *self == TO_A::TIMEOUT16
    }
    #[doc = "Checks if the value of the field is `TIMEOUT32`"]
    #[inline(always)]
    pub fn is_timeout32(&self) -> bool {
        *self == TO_A::TIMEOUT32
    }
    #[doc = "Checks if the value of the field is `TIMEOUT65K`"]
    #[inline(always)]
    pub fn is_timeout65k(&self) -> bool {
        *self == TO_A::TIMEOUT65K
    }
}
#[doc = "Field `TO` writer - Time-out time value"]
pub type TO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMEOUT_SPEC, u16, TO_A, 12, O>;
impl<'a, const O: u8> TO_W<'a, O> {
    #[doc = "A time-out will occur after 16 counts of the I2C function clock."]
    #[inline(always)]
    pub fn timeout16(self) -> &'a mut W {
        self.variant(TO_A::TIMEOUT16)
    }
    #[doc = "A time-out will occur after 32 counts of the I2C function clock."]
    #[inline(always)]
    pub fn timeout32(self) -> &'a mut W {
        self.variant(TO_A::TIMEOUT32)
    }
    #[doc = "A time-out will occur after 65,536 counts of the I2C function clock."]
    #[inline(always)]
    pub fn timeout65k(self) -> &'a mut W {
        self.variant(TO_A::TIMEOUT65K)
    }
}
impl R {
    #[doc = "Bits 0:3 - Time-out time value, the bottom 4 bits"]
    #[inline(always)]
    pub fn tomin(&self) -> TOMIN_R {
        TOMIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Time-out time value"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time-out time value, the bottom 4 bits"]
    #[inline(always)]
    #[must_use]
    pub fn tomin(&mut self) -> TOMIN_W<0> {
        TOMIN_W::new(self)
    }
    #[doc = "Bits 4:15 - Time-out time value"]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<4> {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time-out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout](index.html) module"]
pub struct TIMEOUT_SPEC;
impl crate::RegisterSpec for TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timeout::R](R) reader structure"]
impl crate::Readable for TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timeout::W](W) writer structure"]
impl crate::Writable for TIMEOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0xffff"]
impl crate::Resettable for TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
