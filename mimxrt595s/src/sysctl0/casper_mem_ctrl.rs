#[doc = "Register `CASPER_MEM_CTRL` reader"]
pub struct R(crate::R<CASPER_MEM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CASPER_MEM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CASPER_MEM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CASPER_MEM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CASPER_MEM_CTRL` writer"]
pub struct W(crate::W<CASPER_MEM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CASPER_MEM_CTRL_SPEC>;
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
impl From<crate::W<CASPER_MEM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CASPER_MEM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_IG` reader - Auto Input Gate Control Disable"]
pub type MEM_IG_R = crate::BitReader<MEM_IG_A>;
#[doc = "Auto Input Gate Control Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM_IG_A {
    #[doc = "0: Input Gate is controlled by auto clock gating signal."]
    ENABLE = 0,
    #[doc = "1: Input Gate Is forced low."]
    DISABLE = 1,
}
impl From<MEM_IG_A> for bool {
    #[inline(always)]
    fn from(variant: MEM_IG_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM_IG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_IG_A {
        match self.bits {
            false => MEM_IG_A::ENABLE,
            true => MEM_IG_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MEM_IG_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MEM_IG_A::DISABLE
    }
}
#[doc = "Field `MEM_IG` writer - Auto Input Gate Control Disable"]
pub type MEM_IG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CASPER_MEM_CTRL_SPEC, MEM_IG_A, O>;
impl<'a, const O: u8> MEM_IG_W<'a, O> {
    #[doc = "Input Gate is controlled by auto clock gating signal."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MEM_IG_A::ENABLE)
    }
    #[doc = "Input Gate Is forced low."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MEM_IG_A::DISABLE)
    }
}
#[doc = "Field `MEM_STDBY` reader - Auto Standby Control Disable"]
pub type MEM_STDBY_R = crate::BitReader<MEM_STDBY_A>;
#[doc = "Auto Standby Control Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM_STDBY_A {
    #[doc = "0: STDBY is controlled by auto clock gating signal."]
    ENABLE = 0,
    #[doc = "1: STDBY Is forced low."]
    DISABLE = 1,
}
impl From<MEM_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: MEM_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_STDBY_A {
        match self.bits {
            false => MEM_STDBY_A::ENABLE,
            true => MEM_STDBY_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MEM_STDBY_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MEM_STDBY_A::DISABLE
    }
}
#[doc = "Field `MEM_STDBY` writer - Auto Standby Control Disable"]
pub type MEM_STDBY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CASPER_MEM_CTRL_SPEC, MEM_STDBY_A, O>;
impl<'a, const O: u8> MEM_STDBY_W<'a, O> {
    #[doc = "STDBY is controlled by auto clock gating signal."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MEM_STDBY_A::ENABLE)
    }
    #[doc = "STDBY Is forced low."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MEM_STDBY_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Auto Input Gate Control Disable"]
    #[inline(always)]
    pub fn mem_ig(&self) -> MEM_IG_R {
        MEM_IG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto Standby Control Disable"]
    #[inline(always)]
    pub fn mem_stdby(&self) -> MEM_STDBY_R {
        MEM_STDBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Input Gate Control Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mem_ig(&mut self) -> MEM_IG_W<0> {
        MEM_IG_W::new(self)
    }
    #[doc = "Bit 1 - Auto Standby Control Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mem_stdby(&mut self) -> MEM_STDBY_W<1> {
        MEM_STDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CASPER Memory Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [casper_mem_ctrl](index.html) module"]
pub struct CASPER_MEM_CTRL_SPEC;
impl crate::RegisterSpec for CASPER_MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [casper_mem_ctrl::R](R) reader structure"]
impl crate::Readable for CASPER_MEM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [casper_mem_ctrl::W](W) writer structure"]
impl crate::Writable for CASPER_MEM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CASPER_MEM_CTRL to value 0x03"]
impl crate::Resettable for CASPER_MEM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
