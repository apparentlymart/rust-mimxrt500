#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: The SAU is disabled."]
    DISABLED = 0,
    #[doc = "1: The SAU is enabled."]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Field `ENABLE` writer - Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "The SAU is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "The SAU is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
}
#[doc = "Field `ALLNS` reader - All Non-secure."]
pub type ALLNS_R = crate::BitReader<ALLNS_A>;
#[doc = "All Non-secure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALLNS_A {
    #[doc = "0: Memory is marked as Secure and is not Non-secure callable."]
    SECURED_MEMORY = 0,
    #[doc = "1: Memory is marked as Non-secure."]
    NON_SECURED_MEMORY = 1,
}
impl From<ALLNS_A> for bool {
    #[inline(always)]
    fn from(variant: ALLNS_A) -> Self {
        variant as u8 != 0
    }
}
impl ALLNS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALLNS_A {
        match self.bits {
            false => ALLNS_A::SECURED_MEMORY,
            true => ALLNS_A::NON_SECURED_MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURED_MEMORY`"]
    #[inline(always)]
    pub fn is_secured_memory(&self) -> bool {
        *self == ALLNS_A::SECURED_MEMORY
    }
    #[doc = "Checks if the value of the field is `NON_SECURED_MEMORY`"]
    #[inline(always)]
    pub fn is_non_secured_memory(&self) -> bool {
        *self == ALLNS_A::NON_SECURED_MEMORY
    }
}
#[doc = "Field `ALLNS` writer - All Non-secure."]
pub type ALLNS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ALLNS_A, O>;
impl<'a, const O: u8> ALLNS_W<'a, O> {
    #[doc = "Memory is marked as Secure and is not Non-secure callable."]
    #[inline(always)]
    pub fn secured_memory(self) -> &'a mut W {
        self.variant(ALLNS_A::SECURED_MEMORY)
    }
    #[doc = "Memory is marked as Non-secure."]
    #[inline(always)]
    pub fn non_secured_memory(self) -> &'a mut W {
        self.variant(ALLNS_A::NON_SECURED_MEMORY)
    }
}
impl R {
    #[doc = "Bit 0 - Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - All Non-secure."]
    #[inline(always)]
    pub fn allns(&self) -> ALLNS_R {
        ALLNS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - All Non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn allns(&mut self) -> ALLNS_W<1> {
        ALLNS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Attribution Unit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
