#[doc = "Register `AUTOWKUP` reader"]
pub struct R(crate::R<AUTOWKUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOWKUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOWKUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOWKUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOWKUP` writer"]
pub struct W(crate::W<AUTOWKUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOWKUP_SPEC>;
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
impl From<crate::W<AUTOWKUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOWKUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOWKTIME` reader - Auto wake up delay timer. Added delay after sequencer delay delay time = value/16MHz"]
pub type AUTOWKTIME_R = crate::FieldReader<u16, AUTOWKTIME_A>;
#[doc = "Auto wake up delay timer. Added delay after sequencer delay delay time = value/16MHz\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum AUTOWKTIME_A {
    #[doc = "4095: Delay time = 0x0FFF/16MHz (example)"]
    VALUE_0X0FFF = 4095,
}
impl From<AUTOWKTIME_A> for u16 {
    #[inline(always)]
    fn from(variant: AUTOWKTIME_A) -> Self {
        variant as _
    }
}
impl AUTOWKTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AUTOWKTIME_A> {
        match self.bits {
            4095 => Some(AUTOWKTIME_A::VALUE_0X0FFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0X0FFF`"]
    #[inline(always)]
    pub fn is_value_0x0fff(&self) -> bool {
        *self == AUTOWKTIME_A::VALUE_0X0FFF
    }
}
#[doc = "Field `AUTOWKTIME` writer - Auto wake up delay timer. Added delay after sequencer delay delay time = value/16MHz"]
pub type AUTOWKTIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUTOWKUP_SPEC, u16, AUTOWKTIME_A, 16, O>;
impl<'a, const O: u8> AUTOWKTIME_W<'a, O> {
    #[doc = "Delay time = 0x0FFF/16MHz (example)"]
    #[inline(always)]
    pub fn value_0x0fff(self) -> &'a mut W {
        self.variant(AUTOWKTIME_A::VALUE_0X0FFF)
    }
}
impl R {
    #[doc = "Bits 0:15 - Auto wake up delay timer. Added delay after sequencer delay delay time = value/16MHz"]
    #[inline(always)]
    pub fn autowktime(&self) -> AUTOWKTIME_R {
        AUTOWKTIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto wake up delay timer. Added delay after sequencer delay delay time = value/16MHz"]
    #[inline(always)]
    #[must_use]
    pub fn autowktime(&mut self) -> AUTOWKTIME_W<0> {
        AUTOWKTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMC Automatic wakeup from deepsleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autowkup](index.html) module"]
pub struct AUTOWKUP_SPEC;
impl crate::RegisterSpec for AUTOWKUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autowkup::R](R) reader structure"]
impl crate::Readable for AUTOWKUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autowkup::W](W) writer structure"]
impl crate::Writable for AUTOWKUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOWKUP to value 0"]
impl crate::Resettable for AUTOWKUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
