#[doc = "Register `PCFG13` reader"]
pub struct R(crate::R<PCFG13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFG13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFG13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFG13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFG13` writer"]
pub struct W(crate::W<PCFG13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFG13_SPEC>;
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
impl From<crate::W<PCFG13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFG13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAIRENABLE` reader - Pair Enable"]
pub type PAIRENABLE_R = crate::BitReader<PAIRENABLE_A>;
#[doc = "Pair Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAIRENABLE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PAIRENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PAIRENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PAIRENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAIRENABLE_A {
        match self.bits {
            false => PAIRENABLE_A::DISABLED,
            true => PAIRENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAIRENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAIRENABLE_A::ENABLED
    }
}
#[doc = "Field `PAIRENABLE` writer - Pair Enable"]
pub type PAIRENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCFG13_SPEC, PAIRENABLE_A, O>;
impl<'a, const O: u8> PAIRENABLE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAIRENABLE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAIRENABLE_A::ENABLED)
    }
}
#[doc = "Field `ONECHANNEL` reader - Single Channel Mode"]
pub type ONECHANNEL_R = crate::BitReader<ONECHANNEL_A>;
#[doc = "Single Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONECHANNEL_A {
    #[doc = "0: Dual Channel"]
    DUAL_CHANNEL = 0,
    #[doc = "1: Single Channel"]
    SINGLE_CHANNEL = 1,
}
impl From<ONECHANNEL_A> for bool {
    #[inline(always)]
    fn from(variant: ONECHANNEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ONECHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONECHANNEL_A {
        match self.bits {
            false => ONECHANNEL_A::DUAL_CHANNEL,
            true => ONECHANNEL_A::SINGLE_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `DUAL_CHANNEL`"]
    #[inline(always)]
    pub fn is_dual_channel(&self) -> bool {
        *self == ONECHANNEL_A::DUAL_CHANNEL
    }
    #[doc = "Checks if the value of the field is `SINGLE_CHANNEL`"]
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == ONECHANNEL_A::SINGLE_CHANNEL
    }
}
#[doc = "Field `ONECHANNEL` writer - Single Channel Mode"]
pub type ONECHANNEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCFG13_SPEC, ONECHANNEL_A, O>;
impl<'a, const O: u8> ONECHANNEL_W<'a, O> {
    #[doc = "Dual Channel"]
    #[inline(always)]
    pub fn dual_channel(self) -> &'a mut W {
        self.variant(ONECHANNEL_A::DUAL_CHANNEL)
    }
    #[doc = "Single Channel"]
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(ONECHANNEL_A::SINGLE_CHANNEL)
    }
}
impl R {
    #[doc = "Bit 0 - Pair Enable"]
    #[inline(always)]
    pub fn pairenable(&self) -> PAIRENABLE_R {
        PAIRENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - Single Channel Mode"]
    #[inline(always)]
    pub fn onechannel(&self) -> ONECHANNEL_R {
        ONECHANNEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pair Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pairenable(&mut self) -> PAIRENABLE_W<0> {
        PAIRENABLE_W::new(self)
    }
    #[doc = "Bit 10 - Single Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn onechannel(&mut self) -> ONECHANNEL_W<10> {
        ONECHANNEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register 1 for Channel Pair 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg13](index.html) module"]
pub struct PCFG13_SPEC;
impl crate::RegisterSpec for PCFG13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcfg13::R](R) reader structure"]
impl crate::Readable for PCFG13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfg13::W](W) writer structure"]
impl crate::Writable for PCFG13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFG13 to value 0"]
impl crate::Resettable for PCFG13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
