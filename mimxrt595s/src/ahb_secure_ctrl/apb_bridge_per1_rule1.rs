#[doc = "Register `APB_BRIDGE_PER1_RULE1` reader"]
pub struct R(crate::R<APB_BRIDGE_PER1_RULE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_BRIDGE_PER1_RULE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_BRIDGE_PER1_RULE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_BRIDGE_PER1_RULE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_BRIDGE_PER1_RULE1` writer"]
pub struct W(crate::W<APB_BRIDGE_PER1_RULE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_BRIDGE_PER1_RULE1_SPEC>;
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
impl From<crate::W<APB_BRIDGE_PER1_RULE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_BRIDGE_PER1_RULE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CT32B0` reader - CT32B0"]
pub type CT32B0_R = crate::FieldReader<u8, CT32B0_A>;
#[doc = "CT32B0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CT32B0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CT32B0_A> for u8 {
    #[inline(always)]
    fn from(variant: CT32B0_A) -> Self {
        variant as _
    }
}
impl CT32B0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B0_A {
        match self.bits {
            0 => CT32B0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CT32B0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CT32B0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CT32B0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CT32B0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CT32B0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CT32B0` writer - CT32B0"]
pub type CT32B0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE1_SPEC, u8, CT32B0_A, 2, O>;
impl<'a, const O: u8> CT32B0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CT32B1` reader - CT32B1"]
pub type CT32B1_R = crate::FieldReader<u8, CT32B1_A>;
#[doc = "CT32B1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CT32B1_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CT32B1_A> for u8 {
    #[inline(always)]
    fn from(variant: CT32B1_A) -> Self {
        variant as _
    }
}
impl CT32B1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B1_A {
        match self.bits {
            0 => CT32B1_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CT32B1_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CT32B1_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CT32B1_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B1_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CT32B1_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B1_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CT32B1_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CT32B1` writer - CT32B1"]
pub type CT32B1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE1_SPEC, u8, CT32B1_A, 2, O>;
impl<'a, const O: u8> CT32B1_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B1_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B1_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B1_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B1_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CT32B2` reader - CT32B2"]
pub type CT32B2_R = crate::FieldReader<u8, CT32B2_A>;
#[doc = "CT32B2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CT32B2_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CT32B2_A> for u8 {
    #[inline(always)]
    fn from(variant: CT32B2_A) -> Self {
        variant as _
    }
}
impl CT32B2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B2_A {
        match self.bits {
            0 => CT32B2_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CT32B2_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CT32B2_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CT32B2_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B2_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CT32B2_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B2_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CT32B2_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CT32B2` writer - CT32B2"]
pub type CT32B2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE1_SPEC, u8, CT32B2_A, 2, O>;
impl<'a, const O: u8> CT32B2_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B2_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B2_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B2_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B2_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CT32B3` reader - CT32B3"]
pub type CT32B3_R = crate::FieldReader<u8, CT32B3_A>;
#[doc = "CT32B3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CT32B3_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CT32B3_A> for u8 {
    #[inline(always)]
    fn from(variant: CT32B3_A) -> Self {
        variant as _
    }
}
impl CT32B3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B3_A {
        match self.bits {
            0 => CT32B3_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CT32B3_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CT32B3_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CT32B3_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B3_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CT32B3_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B3_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CT32B3_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CT32B3` writer - CT32B3"]
pub type CT32B3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE1_SPEC, u8, CT32B3_A, 2, O>;
impl<'a, const O: u8> CT32B3_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B3_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B3_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B3_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B3_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CT32B4` reader - CT32B4"]
pub type CT32B4_R = crate::FieldReader<u8, CT32B4_A>;
#[doc = "CT32B4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CT32B4_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CT32B4_A> for u8 {
    #[inline(always)]
    fn from(variant: CT32B4_A) -> Self {
        variant as _
    }
}
impl CT32B4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B4_A {
        match self.bits {
            0 => CT32B4_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CT32B4_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CT32B4_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CT32B4_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B4_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CT32B4_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CT32B4_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CT32B4_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CT32B4` writer - CT32B4"]
pub type CT32B4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE1_SPEC, u8, CT32B4_A, 2, O>;
impl<'a, const O: u8> CT32B4_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B4_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B4_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B4_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CT32B4_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `MRT0` reader - MRT0"]
pub type MRT0_R = crate::FieldReader<u8, MRT0_A>;
#[doc = "MRT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MRT0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<MRT0_A> for u8 {
    #[inline(always)]
    fn from(variant: MRT0_A) -> Self {
        variant as _
    }
}
impl MRT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT0_A {
        match self.bits {
            0 => MRT0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => MRT0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => MRT0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => MRT0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == MRT0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == MRT0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == MRT0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == MRT0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `MRT0` writer - MRT0"]
pub type MRT0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE1_SPEC, u8, MRT0_A, 2, O>;
impl<'a, const O: u8> MRT0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MRT0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MRT0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MRT0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MRT0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `WWDT1` reader - WWDT1"]
pub type WWDT1_R = crate::FieldReader<u8, WWDT1_A>;
#[doc = "WWDT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WWDT1_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<WWDT1_A> for u8 {
    #[inline(always)]
    fn from(variant: WWDT1_A) -> Self {
        variant as _
    }
}
impl WWDT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT1_A {
        match self.bits {
            0 => WWDT1_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => WWDT1_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => WWDT1_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => WWDT1_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == WWDT1_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == WWDT1_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == WWDT1_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == WWDT1_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `WWDT1` writer - WWDT1"]
pub type WWDT1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE1_SPEC, u8, WWDT1_A, 2, O>;
impl<'a, const O: u8> WWDT1_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(WWDT1_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(WWDT1_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(WWDT1_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(WWDT1_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FREQMEASURE` reader - FREQMEASURE"]
pub type FREQMEASURE_R = crate::FieldReader<u8, FREQMEASURE_A>;
#[doc = "FREQMEASURE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FREQMEASURE_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FREQMEASURE_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQMEASURE_A) -> Self {
        variant as _
    }
}
impl FREQMEASURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQMEASURE_A {
        match self.bits {
            0 => FREQMEASURE_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FREQMEASURE_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FREQMEASURE_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FREQMEASURE_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FREQMEASURE_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FREQMEASURE_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FREQMEASURE_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FREQMEASURE_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FREQMEASURE` writer - FREQMEASURE"]
pub type FREQMEASURE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE1_SPEC, u8, FREQMEASURE_A, 2, O>;
impl<'a, const O: u8> FREQMEASURE_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FREQMEASURE_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FREQMEASURE_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FREQMEASURE_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FREQMEASURE_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - CT32B0"]
    #[inline(always)]
    pub fn ct32b0(&self) -> CT32B0_R {
        CT32B0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - CT32B1"]
    #[inline(always)]
    pub fn ct32b1(&self) -> CT32B1_R {
        CT32B1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CT32B2"]
    #[inline(always)]
    pub fn ct32b2(&self) -> CT32B2_R {
        CT32B2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CT32B3"]
    #[inline(always)]
    pub fn ct32b3(&self) -> CT32B3_R {
        CT32B3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - CT32B4"]
    #[inline(always)]
    pub fn ct32b4(&self) -> CT32B4_R {
        CT32B4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - MRT0"]
    #[inline(always)]
    pub fn mrt0(&self) -> MRT0_R {
        MRT0_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - WWDT1"]
    #[inline(always)]
    pub fn wwdt1(&self) -> WWDT1_R {
        WWDT1_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - FREQMEASURE"]
    #[inline(always)]
    pub fn freqmeasure(&self) -> FREQMEASURE_R {
        FREQMEASURE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CT32B0"]
    #[inline(always)]
    #[must_use]
    pub fn ct32b0(&mut self) -> CT32B0_W<0> {
        CT32B0_W::new(self)
    }
    #[doc = "Bits 4:5 - CT32B1"]
    #[inline(always)]
    #[must_use]
    pub fn ct32b1(&mut self) -> CT32B1_W<4> {
        CT32B1_W::new(self)
    }
    #[doc = "Bits 8:9 - CT32B2"]
    #[inline(always)]
    #[must_use]
    pub fn ct32b2(&mut self) -> CT32B2_W<8> {
        CT32B2_W::new(self)
    }
    #[doc = "Bits 12:13 - CT32B3"]
    #[inline(always)]
    #[must_use]
    pub fn ct32b3(&mut self) -> CT32B3_W<12> {
        CT32B3_W::new(self)
    }
    #[doc = "Bits 16:17 - CT32B4"]
    #[inline(always)]
    #[must_use]
    pub fn ct32b4(&mut self) -> CT32B4_W<16> {
        CT32B4_W::new(self)
    }
    #[doc = "Bits 20:21 - MRT0"]
    #[inline(always)]
    #[must_use]
    pub fn mrt0(&mut self) -> MRT0_W<20> {
        MRT0_W::new(self)
    }
    #[doc = "Bits 24:25 - WWDT1"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt1(&mut self) -> WWDT1_W<24> {
        WWDT1_W::new(self)
    }
    #[doc = "Bits 28:29 - FREQMEASURE"]
    #[inline(always)]
    #[must_use]
    pub fn freqmeasure(&mut self) -> FREQMEASURE_W<28> {
        FREQMEASURE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Bridge Peripheral 1 Rule 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_bridge_per1_rule1](index.html) module"]
pub struct APB_BRIDGE_PER1_RULE1_SPEC;
impl crate::RegisterSpec for APB_BRIDGE_PER1_RULE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_bridge_per1_rule1::R](R) reader structure"]
impl crate::Readable for APB_BRIDGE_PER1_RULE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_bridge_per1_rule1::W](W) writer structure"]
impl crate::Writable for APB_BRIDGE_PER1_RULE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_BRIDGE_PER1_RULE1 to value 0"]
impl crate::Resettable for APB_BRIDGE_PER1_RULE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
