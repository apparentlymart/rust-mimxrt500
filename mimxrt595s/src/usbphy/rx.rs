#[doc = "Register `RX` reader"]
pub struct R(crate::R<RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX` writer"]
pub struct W(crate::W<RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_SPEC>;
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
impl From<crate::W<RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENVADJ` reader - Envelope detector trip point."]
pub type ENVADJ_R = crate::FieldReader<u8, ENVADJ_A>;
#[doc = "Envelope detector trip point.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENVADJ_A {
    #[doc = "0: Trip-Level Voltage is 0.1000 V"]
    ENVADJ_0 = 0,
    #[doc = "1: Trip-Level Voltage is 0.1125 V"]
    ENVADJ_1 = 1,
    #[doc = "2: Trip-Level Voltage is 0.1250 V"]
    ENVADJ_2 = 2,
    #[doc = "3: Trip-Level Voltage is 0.0875 V"]
    ENVADJ_3 = 3,
}
impl From<ENVADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: ENVADJ_A) -> Self {
        variant as _
    }
}
impl ENVADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENVADJ_A> {
        match self.bits {
            0 => Some(ENVADJ_A::ENVADJ_0),
            1 => Some(ENVADJ_A::ENVADJ_1),
            2 => Some(ENVADJ_A::ENVADJ_2),
            3 => Some(ENVADJ_A::ENVADJ_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENVADJ_0`"]
    #[inline(always)]
    pub fn is_envadj_0(&self) -> bool {
        *self == ENVADJ_A::ENVADJ_0
    }
    #[doc = "Checks if the value of the field is `ENVADJ_1`"]
    #[inline(always)]
    pub fn is_envadj_1(&self) -> bool {
        *self == ENVADJ_A::ENVADJ_1
    }
    #[doc = "Checks if the value of the field is `ENVADJ_2`"]
    #[inline(always)]
    pub fn is_envadj_2(&self) -> bool {
        *self == ENVADJ_A::ENVADJ_2
    }
    #[doc = "Checks if the value of the field is `ENVADJ_3`"]
    #[inline(always)]
    pub fn is_envadj_3(&self) -> bool {
        *self == ENVADJ_A::ENVADJ_3
    }
}
#[doc = "Field `ENVADJ` writer - Envelope detector trip point."]
pub type ENVADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_SPEC, u8, ENVADJ_A, 3, O>;
impl<'a, const O: u8> ENVADJ_W<'a, O> {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline(always)]
    pub fn envadj_0(self) -> &'a mut W {
        self.variant(ENVADJ_A::ENVADJ_0)
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline(always)]
    pub fn envadj_1(self) -> &'a mut W {
        self.variant(ENVADJ_A::ENVADJ_1)
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline(always)]
    pub fn envadj_2(self) -> &'a mut W {
        self.variant(ENVADJ_A::ENVADJ_2)
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline(always)]
    pub fn envadj_3(self) -> &'a mut W {
        self.variant(ENVADJ_A::ENVADJ_3)
    }
}
#[doc = "Field `DISCONADJ` reader - Disconnect detector trip point."]
pub type DISCONADJ_R = crate::FieldReader<u8, DISCONADJ_A>;
#[doc = "Disconnect detector trip point.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISCONADJ_A {
    #[doc = "0: Trip-Level Voltage is 0.56875 V"]
    DISCONADJ_1 = 0,
    #[doc = "1: Trip-Level Voltage is 0.55000 V"]
    DISCONADJ_2 = 1,
    #[doc = "2: Trip-Level Voltage is 0.58125 V"]
    DISCONADJ_3 = 2,
    #[doc = "3: Trip-Level Voltage is 0.60000 V"]
    DISCONADJ_4 = 3,
}
impl From<DISCONADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: DISCONADJ_A) -> Self {
        variant as _
    }
}
impl DISCONADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISCONADJ_A> {
        match self.bits {
            0 => Some(DISCONADJ_A::DISCONADJ_1),
            1 => Some(DISCONADJ_A::DISCONADJ_2),
            2 => Some(DISCONADJ_A::DISCONADJ_3),
            3 => Some(DISCONADJ_A::DISCONADJ_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONADJ_1`"]
    #[inline(always)]
    pub fn is_disconadj_1(&self) -> bool {
        *self == DISCONADJ_A::DISCONADJ_1
    }
    #[doc = "Checks if the value of the field is `DISCONADJ_2`"]
    #[inline(always)]
    pub fn is_disconadj_2(&self) -> bool {
        *self == DISCONADJ_A::DISCONADJ_2
    }
    #[doc = "Checks if the value of the field is `DISCONADJ_3`"]
    #[inline(always)]
    pub fn is_disconadj_3(&self) -> bool {
        *self == DISCONADJ_A::DISCONADJ_3
    }
    #[doc = "Checks if the value of the field is `DISCONADJ_4`"]
    #[inline(always)]
    pub fn is_disconadj_4(&self) -> bool {
        *self == DISCONADJ_A::DISCONADJ_4
    }
}
#[doc = "Field `DISCONADJ` writer - Disconnect detector trip point."]
pub type DISCONADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_SPEC, u8, DISCONADJ_A, 3, O>;
impl<'a, const O: u8> DISCONADJ_W<'a, O> {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline(always)]
    pub fn disconadj_1(self) -> &'a mut W {
        self.variant(DISCONADJ_A::DISCONADJ_1)
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline(always)]
    pub fn disconadj_2(self) -> &'a mut W {
        self.variant(DISCONADJ_A::DISCONADJ_2)
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline(always)]
    pub fn disconadj_3(self) -> &'a mut W {
        self.variant(DISCONADJ_A::DISCONADJ_3)
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline(always)]
    pub fn disconadj_4(self) -> &'a mut W {
        self.variant(DISCONADJ_A::DISCONADJ_4)
    }
}
#[doc = "Field `RXDBYPASS` reader - DM bypass"]
pub type RXDBYPASS_R = crate::BitReader<RXDBYPASS_A>;
#[doc = "DM bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDBYPASS_A {
    #[doc = "0: Normal operation"]
    RXDBYPASS_0 = 0,
    #[doc = "1: Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    RXDBYPASS_1 = 1,
}
impl From<RXDBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: RXDBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDBYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDBYPASS_A {
        match self.bits {
            false => RXDBYPASS_A::RXDBYPASS_0,
            true => RXDBYPASS_A::RXDBYPASS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXDBYPASS_0`"]
    #[inline(always)]
    pub fn is_rxdbypass_0(&self) -> bool {
        *self == RXDBYPASS_A::RXDBYPASS_0
    }
    #[doc = "Checks if the value of the field is `RXDBYPASS_1`"]
    #[inline(always)]
    pub fn is_rxdbypass_1(&self) -> bool {
        *self == RXDBYPASS_A::RXDBYPASS_1
    }
}
#[doc = "Field `RXDBYPASS` writer - DM bypass"]
pub type RXDBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_SPEC, RXDBYPASS_A, O>;
impl<'a, const O: u8> RXDBYPASS_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn rxdbypass_0(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::RXDBYPASS_0)
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline(always)]
    pub fn rxdbypass_1(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::RXDBYPASS_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Envelope detector trip point."]
    #[inline(always)]
    pub fn envadj(&self) -> ENVADJ_R {
        ENVADJ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Disconnect detector trip point."]
    #[inline(always)]
    pub fn disconadj(&self) -> DISCONADJ_R {
        DISCONADJ_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 22 - DM bypass"]
    #[inline(always)]
    pub fn rxdbypass(&self) -> RXDBYPASS_R {
        RXDBYPASS_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Envelope detector trip point."]
    #[inline(always)]
    #[must_use]
    pub fn envadj(&mut self) -> ENVADJ_W<0> {
        ENVADJ_W::new(self)
    }
    #[doc = "Bits 4:6 - Disconnect detector trip point."]
    #[inline(always)]
    #[must_use]
    pub fn disconadj(&mut self) -> DISCONADJ_W<4> {
        DISCONADJ_W::new(self)
    }
    #[doc = "Bit 22 - DM bypass"]
    #[inline(always)]
    #[must_use]
    pub fn rxdbypass(&mut self) -> RXDBYPASS_W<22> {
        RXDBYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx](index.html) module"]
pub struct RX_SPEC;
impl crate::RegisterSpec for RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx::R](R) reader structure"]
impl crate::Readable for RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx::W](W) writer structure"]
impl crate::Writable for RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX to value 0"]
impl crate::Resettable for RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
