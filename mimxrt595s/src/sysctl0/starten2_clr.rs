#[doc = "Register `STARTEN2_CLR` reader"]
pub struct R(crate::R<STARTEN2_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTEN2_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTEN2_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTEN2_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTEN2_CLR` writer"]
pub struct W(crate::W<STARTEN2_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTEN2_CLR_SPEC>;
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
impl From<crate::W<STARTEN2_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTEN2_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXCOMM12` reader - FlexComm 12 interrupt wake-up"]
pub type FLEXCOMM12_R = crate::BitReader<FLEXCOMM12_A>;
#[doc = "FlexComm 12 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM12_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the STARTEN2 bit"]
    ENABLE = 1,
}
impl From<FLEXCOMM12_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM12_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM12_A {
        match self.bits {
            false => FLEXCOMM12_A::DISABLE,
            true => FLEXCOMM12_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM12_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM12` writer - FlexComm 12 interrupt wake-up"]
pub type FLEXCOMM12_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, STARTEN2_CLR_SPEC, FLEXCOMM12_A, O>;
impl<'a, const O: u8> FLEXCOMM12_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM12_A::DISABLE)
    }
    #[doc = "Clears the STARTEN2 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM12_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM13` reader - FlexComm 13 interrupt wake-up"]
pub type FLEXCOMM13_R = crate::BitReader<FLEXCOMM13_A>;
#[doc = "FlexComm 13 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM13_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the STARTEN2 bit"]
    ENABLE = 1,
}
impl From<FLEXCOMM13_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM13_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM13_A {
        match self.bits {
            false => FLEXCOMM13_A::DISABLE,
            true => FLEXCOMM13_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM13_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM13` writer - FlexComm 13 interrupt wake-up"]
pub type FLEXCOMM13_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, STARTEN2_CLR_SPEC, FLEXCOMM13_A, O>;
impl<'a, const O: u8> FLEXCOMM13_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM13_A::DISABLE)
    }
    #[doc = "Clears the STARTEN2 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM13_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM16` reader - FlexComm 16 interrupt wake-up"]
pub type FLEXCOMM16_R = crate::BitReader<FLEXCOMM16_A>;
#[doc = "FlexComm 16 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM16_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the STARTEN2 bit"]
    ENABLE = 1,
}
impl From<FLEXCOMM16_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM16_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM16_A {
        match self.bits {
            false => FLEXCOMM16_A::DISABLE,
            true => FLEXCOMM16_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM16_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM16_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM16` writer - FlexComm 16 interrupt wake-up"]
pub type FLEXCOMM16_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, STARTEN2_CLR_SPEC, FLEXCOMM16_A, O>;
impl<'a, const O: u8> FLEXCOMM16_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM16_A::DISABLE)
    }
    #[doc = "Clears the STARTEN2 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM16_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - FlexComm 12 interrupt wake-up"]
    #[inline(always)]
    pub fn flexcomm12(&self) -> FLEXCOMM12_R {
        FLEXCOMM12_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FlexComm 13 interrupt wake-up"]
    #[inline(always)]
    pub fn flexcomm13(&self) -> FLEXCOMM13_R {
        FLEXCOMM13_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FlexComm 16 interrupt wake-up"]
    #[inline(always)]
    pub fn flexcomm16(&self) -> FLEXCOMM16_R {
        FLEXCOMM16_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FlexComm 12 interrupt wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm12(&mut self) -> FLEXCOMM12_W<0> {
        FLEXCOMM12_W::new(self)
    }
    #[doc = "Bit 1 - FlexComm 13 interrupt wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm13(&mut self) -> FLEXCOMM13_W<1> {
        FLEXCOMM13_W::new(self)
    }
    #[doc = "Bit 2 - FlexComm 16 interrupt wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm16(&mut self) -> FLEXCOMM16_W<2> {
        FLEXCOMM16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start Enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starten2_clr](index.html) module"]
pub struct STARTEN2_CLR_SPEC;
impl crate::RegisterSpec for STARTEN2_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starten2_clr::R](R) reader structure"]
impl crate::Readable for STARTEN2_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starten2_clr::W](W) writer structure"]
impl crate::Writable for STARTEN2_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x07;
}
#[doc = "`reset()` method sets STARTEN2_CLR to value 0"]
impl crate::Resettable for STARTEN2_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
