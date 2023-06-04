#[doc = "Register `DC_CTRL` reader"]
pub struct R(crate::R<DC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_CTRL` writer"]
pub struct W(crate::W<DC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_CTRL_SPEC>;
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
impl From<crate::W<DC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCPOLE` reader - DC Block Filter"]
pub type DCPOLE_R = crate::FieldReader<u8, DCPOLE_A>;
#[doc = "DC Block Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCPOLE_A {
    #[doc = "0: Flat Response, no filter"]
    FLAT_RESPONSE = 0,
    #[doc = "1: 155 Hz"]
    HZ_155 = 1,
    #[doc = "2: 78 Hz"]
    HZ_78 = 2,
    #[doc = "3: 39 Hz"]
    HZ_39 = 3,
}
impl From<DCPOLE_A> for u8 {
    #[inline(always)]
    fn from(variant: DCPOLE_A) -> Self {
        variant as _
    }
}
impl DCPOLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCPOLE_A {
        match self.bits {
            0 => DCPOLE_A::FLAT_RESPONSE,
            1 => DCPOLE_A::HZ_155,
            2 => DCPOLE_A::HZ_78,
            3 => DCPOLE_A::HZ_39,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLAT_RESPONSE`"]
    #[inline(always)]
    pub fn is_flat_response(&self) -> bool {
        *self == DCPOLE_A::FLAT_RESPONSE
    }
    #[doc = "Checks if the value of the field is `HZ_155`"]
    #[inline(always)]
    pub fn is_hz_155(&self) -> bool {
        *self == DCPOLE_A::HZ_155
    }
    #[doc = "Checks if the value of the field is `HZ_78`"]
    #[inline(always)]
    pub fn is_hz_78(&self) -> bool {
        *self == DCPOLE_A::HZ_78
    }
    #[doc = "Checks if the value of the field is `HZ_39`"]
    #[inline(always)]
    pub fn is_hz_39(&self) -> bool {
        *self == DCPOLE_A::HZ_39
    }
}
#[doc = "Field `DCPOLE` writer - DC Block Filter"]
pub type DCPOLE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DC_CTRL_SPEC, u8, DCPOLE_A, 2, O>;
impl<'a, const O: u8> DCPOLE_W<'a, O> {
    #[doc = "Flat Response, no filter"]
    #[inline(always)]
    pub fn flat_response(self) -> &'a mut W {
        self.variant(DCPOLE_A::FLAT_RESPONSE)
    }
    #[doc = "155 Hz"]
    #[inline(always)]
    pub fn hz_155(self) -> &'a mut W {
        self.variant(DCPOLE_A::HZ_155)
    }
    #[doc = "78 Hz"]
    #[inline(always)]
    pub fn hz_78(self) -> &'a mut W {
        self.variant(DCPOLE_A::HZ_78)
    }
    #[doc = "39 Hz"]
    #[inline(always)]
    pub fn hz_39(self) -> &'a mut W {
        self.variant(DCPOLE_A::HZ_39)
    }
}
#[doc = "Field `DCGAIN` reader - DC Gain"]
pub type DCGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCGAIN` writer - DC Gain"]
pub type DCGAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DC_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SATURATEAT16BIT` reader - Saturate at 16 Bit"]
pub type SATURATEAT16BIT_R = crate::BitReader<SATURATEAT16BIT_A>;
#[doc = "Saturate at 16 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SATURATEAT16BIT_A {
    #[doc = "0: Do not Saturate. Results roll over if out range and do not saturate."]
    DO_NOT_SATURATE = 0,
    #[doc = "1: Saturate. If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    SATURATE = 1,
}
impl From<SATURATEAT16BIT_A> for bool {
    #[inline(always)]
    fn from(variant: SATURATEAT16BIT_A) -> Self {
        variant as u8 != 0
    }
}
impl SATURATEAT16BIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SATURATEAT16BIT_A {
        match self.bits {
            false => SATURATEAT16BIT_A::DO_NOT_SATURATE,
            true => SATURATEAT16BIT_A::SATURATE,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOT_SATURATE`"]
    #[inline(always)]
    pub fn is_do_not_saturate(&self) -> bool {
        *self == SATURATEAT16BIT_A::DO_NOT_SATURATE
    }
    #[doc = "Checks if the value of the field is `SATURATE`"]
    #[inline(always)]
    pub fn is_saturate(&self) -> bool {
        *self == SATURATEAT16BIT_A::SATURATE
    }
}
#[doc = "Field `SATURATEAT16BIT` writer - Saturate at 16 Bit"]
pub type SATURATEAT16BIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DC_CTRL_SPEC, SATURATEAT16BIT_A, O>;
impl<'a, const O: u8> SATURATEAT16BIT_W<'a, O> {
    #[doc = "Do not Saturate. Results roll over if out range and do not saturate."]
    #[inline(always)]
    pub fn do_not_saturate(self) -> &'a mut W {
        self.variant(SATURATEAT16BIT_A::DO_NOT_SATURATE)
    }
    #[doc = "Saturate. If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    #[inline(always)]
    pub fn saturate(self) -> &'a mut W {
        self.variant(SATURATEAT16BIT_A::SATURATE)
    }
}
#[doc = "Field `SIGNEXTEND` reader - Sign Extend"]
pub type SIGNEXTEND_R = crate::BitReader<SIGNEXTEND_A>;
#[doc = "Sign Extend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGNEXTEND_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<SIGNEXTEND_A> for bool {
    #[inline(always)]
    fn from(variant: SIGNEXTEND_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGNEXTEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGNEXTEND_A {
        match self.bits {
            false => SIGNEXTEND_A::DISABLE,
            true => SIGNEXTEND_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SIGNEXTEND_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SIGNEXTEND_A::ENABLE
    }
}
#[doc = "Field `SIGNEXTEND` writer - Sign Extend"]
pub type SIGNEXTEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, DC_CTRL_SPEC, SIGNEXTEND_A, O>;
impl<'a, const O: u8> SIGNEXTEND_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SIGNEXTEND_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SIGNEXTEND_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - DC Block Filter"]
    #[inline(always)]
    pub fn dcpole(&self) -> DCPOLE_R {
        DCPOLE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - DC Gain"]
    #[inline(always)]
    pub fn dcgain(&self) -> DCGAIN_R {
        DCGAIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Saturate at 16 Bit"]
    #[inline(always)]
    pub fn saturateat16bit(&self) -> SATURATEAT16BIT_R {
        SATURATEAT16BIT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sign Extend"]
    #[inline(always)]
    pub fn signextend(&self) -> SIGNEXTEND_R {
        SIGNEXTEND_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DC Block Filter"]
    #[inline(always)]
    #[must_use]
    pub fn dcpole(&mut self) -> DCPOLE_W<0> {
        DCPOLE_W::new(self)
    }
    #[doc = "Bits 4:7 - DC Gain"]
    #[inline(always)]
    #[must_use]
    pub fn dcgain(&mut self) -> DCGAIN_W<4> {
        DCGAIN_W::new(self)
    }
    #[doc = "Bit 8 - Saturate at 16 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn saturateat16bit(&mut self) -> SATURATEAT16BIT_W<8> {
        SATURATEAT16BIT_W::new(self)
    }
    #[doc = "Bit 9 - Sign Extend"]
    #[inline(always)]
    #[must_use]
    pub fn signextend(&mut self) -> SIGNEXTEND_W<9> {
        SIGNEXTEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DC Filter Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_ctrl](index.html) module"]
pub struct DC_CTRL_SPEC;
impl crate::RegisterSpec for DC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_ctrl::R](R) reader structure"]
impl crate::Readable for DC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_ctrl::W](W) writer structure"]
impl crate::Writable for DC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_CTRL to value 0"]
impl crate::Resettable for DC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
