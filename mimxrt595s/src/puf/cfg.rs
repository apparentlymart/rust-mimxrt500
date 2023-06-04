#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCKENROLL_SETKEY` reader - Block Enroll and Set Key Operation"]
pub type BLOCKENROLL_SETKEY_R = crate::BitReader<BLOCKENROLL_SETKEY_A>;
#[doc = "Block Enroll and Set Key Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLOCKENROLL_SETKEY_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BLOCKENROLL_SETKEY_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCKENROLL_SETKEY_A) -> Self {
        variant as u8 != 0
    }
}
impl BLOCKENROLL_SETKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCKENROLL_SETKEY_A {
        match self.bits {
            false => BLOCKENROLL_SETKEY_A::DISABLED,
            true => BLOCKENROLL_SETKEY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BLOCKENROLL_SETKEY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BLOCKENROLL_SETKEY_A::ENABLED
    }
}
#[doc = "Field `BLOCKENROLL_SETKEY` writer - Block Enroll and Set Key Operation"]
pub type BLOCKENROLL_SETKEY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFG_SPEC, BLOCKENROLL_SETKEY_A, O>;
impl<'a, const O: u8> BLOCKENROLL_SETKEY_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BLOCKENROLL_SETKEY_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BLOCKENROLL_SETKEY_A::ENABLED)
    }
}
#[doc = "Field `BLOCKKEYOUTPUT` reader - Block Key Output Data"]
pub type BLOCKKEYOUTPUT_R = crate::BitReader<BLOCKKEYOUTPUT_A>;
#[doc = "Block Key Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLOCKKEYOUTPUT_A {
    #[doc = "0: Disabled. BLOCKKEYOUTPUT is cleared on reset."]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BLOCKKEYOUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCKKEYOUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl BLOCKKEYOUTPUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCKKEYOUTPUT_A {
        match self.bits {
            false => BLOCKKEYOUTPUT_A::DISABLED,
            true => BLOCKKEYOUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BLOCKKEYOUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BLOCKKEYOUTPUT_A::ENABLED
    }
}
#[doc = "Field `BLOCKKEYOUTPUT` writer - Block Key Output Data"]
pub type BLOCKKEYOUTPUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFG_SPEC, BLOCKKEYOUTPUT_A, O>;
impl<'a, const O: u8> BLOCKKEYOUTPUT_W<'a, O> {
    #[doc = "Disabled. BLOCKKEYOUTPUT is cleared on reset."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BLOCKKEYOUTPUT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BLOCKKEYOUTPUT_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Block Enroll and Set Key Operation"]
    #[inline(always)]
    pub fn blockenroll_setkey(&self) -> BLOCKENROLL_SETKEY_R {
        BLOCKENROLL_SETKEY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Key Output Data"]
    #[inline(always)]
    pub fn blockkeyoutput(&self) -> BLOCKKEYOUTPUT_R {
        BLOCKKEYOUTPUT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block Enroll and Set Key Operation"]
    #[inline(always)]
    #[must_use]
    pub fn blockenroll_setkey(&mut self) -> BLOCKENROLL_SETKEY_W<0> {
        BLOCKENROLL_SETKEY_W::new(self)
    }
    #[doc = "Bit 1 - Block Key Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn blockkeyoutput(&mut self) -> BLOCKKEYOUTPUT_W<1> {
        BLOCKKEYOUTPUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
