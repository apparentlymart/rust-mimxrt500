#[doc = "Register `APB_BRIDGE_PER1_RULE3` reader"]
pub struct R(crate::R<APB_BRIDGE_PER1_RULE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_BRIDGE_PER1_RULE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_BRIDGE_PER1_RULE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_BRIDGE_PER1_RULE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_BRIDGE_PER1_RULE3` writer"]
pub struct W(crate::W<APB_BRIDGE_PER1_RULE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_BRIDGE_PER1_RULE3_SPEC>;
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
impl From<crate::W<APB_BRIDGE_PER1_RULE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_BRIDGE_PER1_RULE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRT1` reader - MRT1"]
pub type MRT1_R = crate::FieldReader<u8, MRT1_A>;
#[doc = "MRT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MRT1_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<MRT1_A> for u8 {
    #[inline(always)]
    fn from(variant: MRT1_A) -> Self {
        variant as _
    }
}
impl MRT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT1_A {
        match self.bits {
            0 => MRT1_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => MRT1_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => MRT1_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => MRT1_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == MRT1_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == MRT1_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == MRT1_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == MRT1_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `MRT1` writer - MRT1"]
pub type MRT1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE3_SPEC, u8, MRT1_A, 2, O>;
impl<'a, const O: u8> MRT1_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MRT1_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MRT1_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MRT1_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MRT1_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 28:29 - MRT1"]
    #[inline(always)]
    pub fn mrt1(&self) -> MRT1_R {
        MRT1_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - MRT1"]
    #[inline(always)]
    #[must_use]
    pub fn mrt1(&mut self) -> MRT1_W<28> {
        MRT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Bridge Peripheral 1 Rule 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_bridge_per1_rule3](index.html) module"]
pub struct APB_BRIDGE_PER1_RULE3_SPEC;
impl crate::RegisterSpec for APB_BRIDGE_PER1_RULE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_bridge_per1_rule3::R](R) reader structure"]
impl crate::Readable for APB_BRIDGE_PER1_RULE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_bridge_per1_rule3::W](W) writer structure"]
impl crate::Writable for APB_BRIDGE_PER1_RULE3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_BRIDGE_PER1_RULE3 to value 0"]
impl crate::Resettable for APB_BRIDGE_PER1_RULE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
