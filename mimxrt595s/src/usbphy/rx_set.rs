#[doc = "Register `RX_SET` reader"]
pub struct R(crate::R<RX_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_SET` writer"]
pub struct W(crate::W<RX_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_SET_SPEC>;
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
impl From<crate::W<RX_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENVADJ` reader - Envelope detector trip point."]
pub type ENVADJ_R = crate::FieldReader<u8, ENVADJ_A>;
#[doc = "Envelope detector trip point.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENVADJ_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding TX bit"]
    ENABLE = 1,
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
            0 => Some(ENVADJ_A::DISABLE),
            1 => Some(ENVADJ_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENVADJ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENVADJ_A::ENABLE
    }
}
#[doc = "Field `ENVADJ` writer - Envelope detector trip point."]
pub type ENVADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_SET_SPEC, u8, ENVADJ_A, 3, O>;
impl<'a, const O: u8> ENVADJ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENVADJ_A::DISABLE)
    }
    #[doc = "Sets the corresponding TX bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENVADJ_A::ENABLE)
    }
}
#[doc = "Field `DISCONADJ` reader - Disconnect detector trip point."]
pub type DISCONADJ_R = crate::FieldReader<u8, DISCONADJ_A>;
#[doc = "Disconnect detector trip point.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISCONADJ_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding TX bit"]
    ENABLE = 1,
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
            0 => Some(DISCONADJ_A::DISABLE),
            1 => Some(DISCONADJ_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DISCONADJ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DISCONADJ_A::ENABLE
    }
}
#[doc = "Field `DISCONADJ` writer - Disconnect detector trip point."]
pub type DISCONADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_SET_SPEC, u8, DISCONADJ_A, 3, O>;
impl<'a, const O: u8> DISCONADJ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DISCONADJ_A::DISABLE)
    }
    #[doc = "Sets the corresponding TX bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DISCONADJ_A::ENABLE)
    }
}
#[doc = "Field `RXDBYPASS` reader - DM bypass"]
pub type RXDBYPASS_R = crate::BitReader<RXDBYPASS_A>;
#[doc = "DM bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDBYPASS_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding TX bit"]
    ENABLE = 1,
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
            false => RXDBYPASS_A::DISABLE,
            true => RXDBYPASS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXDBYPASS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXDBYPASS_A::ENABLE
    }
}
#[doc = "Field `RXDBYPASS` writer - DM bypass"]
pub type RXDBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_SET_SPEC, RXDBYPASS_A, O>;
impl<'a, const O: u8> RXDBYPASS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::DISABLE)
    }
    #[doc = "Sets the corresponding TX bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::ENABLE)
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
#[doc = "RX Control Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_set](index.html) module"]
pub struct RX_SET_SPEC;
impl crate::RegisterSpec for RX_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_set::R](R) reader structure"]
impl crate::Readable for RX_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_set::W](W) writer structure"]
impl crate::Writable for RX_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_SET to value 0"]
impl crate::Resettable for RX_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
