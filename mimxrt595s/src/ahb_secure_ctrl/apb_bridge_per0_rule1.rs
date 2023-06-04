#[doc = "Register `APB_BRIDGE_PER0_RULE1` reader"]
pub struct R(crate::R<APB_BRIDGE_PER0_RULE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_BRIDGE_PER0_RULE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_BRIDGE_PER0_RULE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_BRIDGE_PER0_RULE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_BRIDGE_PER0_RULE1` writer"]
pub struct W(crate::W<APB_BRIDGE_PER0_RULE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_BRIDGE_PER0_RULE1_SPEC>;
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
impl From<crate::W<APB_BRIDGE_PER0_RULE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_BRIDGE_PER0_RULE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WWDT0` reader - WWDT0"]
pub type WWDT0_R = crate::FieldReader<u8, WWDT0_A>;
#[doc = "WWDT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WWDT0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<WWDT0_A> for u8 {
    #[inline(always)]
    fn from(variant: WWDT0_A) -> Self {
        variant as _
    }
}
impl WWDT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT0_A {
        match self.bits {
            0 => WWDT0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => WWDT0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => WWDT0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => WWDT0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == WWDT0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == WWDT0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == WWDT0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == WWDT0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `WWDT0` writer - WWDT0"]
pub type WWDT0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER0_RULE1_SPEC, u8, WWDT0_A, 2, O>;
impl<'a, const O: u8> WWDT0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(WWDT0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(WWDT0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(WWDT0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(WWDT0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `MICRO_TICK` reader - MICRO_TICK"]
pub type MICRO_TICK_R = crate::FieldReader<u8, MICRO_TICK_A>;
#[doc = "MICRO_TICK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MICRO_TICK_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<MICRO_TICK_A> for u8 {
    #[inline(always)]
    fn from(variant: MICRO_TICK_A) -> Self {
        variant as _
    }
}
impl MICRO_TICK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MICRO_TICK_A {
        match self.bits {
            0 => MICRO_TICK_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => MICRO_TICK_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => MICRO_TICK_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => MICRO_TICK_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == MICRO_TICK_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == MICRO_TICK_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == MICRO_TICK_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == MICRO_TICK_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `MICRO_TICK` writer - MICRO_TICK"]
pub type MICRO_TICK_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER0_RULE1_SPEC, u8, MICRO_TICK_A, 2, O>;
impl<'a, const O: u8> MICRO_TICK_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MICRO_TICK_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MICRO_TICK_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MICRO_TICK_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MICRO_TICK_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 24:25 - WWDT0"]
    #[inline(always)]
    pub fn wwdt0(&self) -> WWDT0_R {
        WWDT0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - MICRO_TICK"]
    #[inline(always)]
    pub fn micro_tick(&self) -> MICRO_TICK_R {
        MICRO_TICK_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25 - WWDT0"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt0(&mut self) -> WWDT0_W<24> {
        WWDT0_W::new(self)
    }
    #[doc = "Bits 28:29 - MICRO_TICK"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tick(&mut self) -> MICRO_TICK_W<28> {
        MICRO_TICK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Bridge Peripheral 0 Rule 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_bridge_per0_rule1](index.html) module"]
pub struct APB_BRIDGE_PER0_RULE1_SPEC;
impl crate::RegisterSpec for APB_BRIDGE_PER0_RULE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_bridge_per0_rule1::R](R) reader structure"]
impl crate::Readable for APB_BRIDGE_PER0_RULE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_bridge_per0_rule1::W](W) writer structure"]
impl crate::Writable for APB_BRIDGE_PER0_RULE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_BRIDGE_PER0_RULE1 to value 0"]
impl crate::Resettable for APB_BRIDGE_PER0_RULE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
