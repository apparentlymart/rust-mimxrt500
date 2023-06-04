#[doc = "Register `AHB_PERIPH3_SLAVE_RULE0` reader"]
pub struct R(crate::R<AHB_PERIPH3_SLAVE_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_PERIPH3_SLAVE_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_PERIPH3_SLAVE_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_PERIPH3_SLAVE_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_PERIPH3_SLAVE_RULE0` writer"]
pub struct W(crate::W<AHB_PERIPH3_SLAVE_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_PERIPH3_SLAVE_RULE0_SPEC>;
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
impl From<crate::W<AHB_PERIPH3_SLAVE_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_PERIPH3_SLAVE_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWERQUAD` reader - POWERQUAD"]
pub type POWERQUAD_R = crate::FieldReader<u8, POWERQUAD_A>;
#[doc = "POWERQUAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POWERQUAD_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<POWERQUAD_A> for u8 {
    #[inline(always)]
    fn from(variant: POWERQUAD_A) -> Self {
        variant as _
    }
}
impl POWERQUAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWERQUAD_A {
        match self.bits {
            0 => POWERQUAD_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => POWERQUAD_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => POWERQUAD_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => POWERQUAD_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == POWERQUAD_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == POWERQUAD_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == POWERQUAD_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == POWERQUAD_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `POWERQUAD` writer - POWERQUAD"]
pub type POWERQUAD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE0_SPEC, u8, POWERQUAD_A, 2, O>;
impl<'a, const O: u8> POWERQUAD_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(POWERQUAD_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(POWERQUAD_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(POWERQUAD_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(POWERQUAD_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CASPER` reader - CASPER"]
pub type CASPER_R = crate::FieldReader<u8, CASPER_A>;
#[doc = "CASPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CASPER_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CASPER_A> for u8 {
    #[inline(always)]
    fn from(variant: CASPER_A) -> Self {
        variant as _
    }
}
impl CASPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_A {
        match self.bits {
            0 => CASPER_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CASPER_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CASPER_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CASPER_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CASPER_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CASPER_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CASPER_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CASPER_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CASPER` writer - CASPER"]
pub type CASPER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE0_SPEC, u8, CASPER_A, 2, O>;
impl<'a, const O: u8> CASPER_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CASPER_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CASPER_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CASPER_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CASPER_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CASPER_RAM` reader - CASPER RAM"]
pub type CASPER_RAM_R = crate::FieldReader<u8, CASPER_RAM_A>;
#[doc = "CASPER RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CASPER_RAM_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CASPER_RAM_A> for u8 {
    #[inline(always)]
    fn from(variant: CASPER_RAM_A) -> Self {
        variant as _
    }
}
impl CASPER_RAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_RAM_A {
        match self.bits {
            0 => CASPER_RAM_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CASPER_RAM_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CASPER_RAM_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CASPER_RAM_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CASPER_RAM_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CASPER_RAM_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CASPER_RAM_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CASPER_RAM_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CASPER_RAM` writer - CASPER RAM"]
pub type CASPER_RAM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE0_SPEC, u8, CASPER_RAM_A, 2, O>;
impl<'a, const O: u8> CASPER_RAM_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CASPER_RAM_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CASPER_RAM_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CASPER_RAM_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CASPER_RAM_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `SECURE_GPIO` reader - Secure GPIO"]
pub type SECURE_GPIO_R = crate::FieldReader<u8, SECURE_GPIO_A>;
#[doc = "Secure GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SECURE_GPIO_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<SECURE_GPIO_A> for u8 {
    #[inline(always)]
    fn from(variant: SECURE_GPIO_A) -> Self {
        variant as _
    }
}
impl SECURE_GPIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECURE_GPIO_A {
        match self.bits {
            0 => SECURE_GPIO_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => SECURE_GPIO_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => SECURE_GPIO_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => SECURE_GPIO_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == SECURE_GPIO_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == SECURE_GPIO_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == SECURE_GPIO_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == SECURE_GPIO_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `SECURE_GPIO` writer - Secure GPIO"]
pub type SECURE_GPIO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE0_SPEC, u8, SECURE_GPIO_A, 2, O>;
impl<'a, const O: u8> SECURE_GPIO_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SECURE_GPIO_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SECURE_GPIO_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SECURE_GPIO_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SECURE_GPIO_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `HASH` reader - HASH"]
pub type HASH_R = crate::FieldReader<u8, HASH_A>;
#[doc = "HASH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HASH_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<HASH_A> for u8 {
    #[inline(always)]
    fn from(variant: HASH_A) -> Self {
        variant as _
    }
}
impl HASH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_A {
        match self.bits {
            0 => HASH_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => HASH_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => HASH_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => HASH_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == HASH_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == HASH_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == HASH_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == HASH_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `HASH` writer - HASH"]
pub type HASH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE0_SPEC, u8, HASH_A, 2, O>;
impl<'a, const O: u8> HASH_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(HASH_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(HASH_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(HASH_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(HASH_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM8` reader - FLEXCOMM 8"]
pub type FLEXCOMM8_R = crate::FieldReader<u8, FLEXCOMM8_A>;
#[doc = "FLEXCOMM 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM8_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM8_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM8_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM8_A {
        match self.bits {
            0 => FLEXCOMM8_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM8_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM8_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM8_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM8_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM8_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM8_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM8_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM8` writer - FLEXCOMM 8"]
pub type FLEXCOMM8_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE0_SPEC, u8, FLEXCOMM8_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM8_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM8_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM8_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM8_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM8_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM9` reader - FLEXCOMM 9"]
pub type FLEXCOMM9_R = crate::FieldReader<u8, FLEXCOMM9_A>;
#[doc = "FLEXCOMM 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM9_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM9_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM9_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM9_A {
        match self.bits {
            0 => FLEXCOMM9_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM9_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM9_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM9_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM9_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM9_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM9_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM9_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM9` writer - FLEXCOMM 9"]
pub type FLEXCOMM9_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE0_SPEC, u8, FLEXCOMM9_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM9_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM9_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM9_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM9_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM9_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM10` reader - FLEXCOMM 10"]
pub type FLEXCOMM10_R = crate::FieldReader<u8, FLEXCOMM10_A>;
#[doc = "FLEXCOMM 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM10_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM10_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM10_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM10_A {
        match self.bits {
            0 => FLEXCOMM10_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM10_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM10_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM10_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM10_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM10_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM10_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM10_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM10` writer - FLEXCOMM 10"]
pub type FLEXCOMM10_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE0_SPEC, u8, FLEXCOMM10_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM10_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM10_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM10_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM10_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM10_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - POWERQUAD"]
    #[inline(always)]
    pub fn powerquad(&self) -> POWERQUAD_R {
        POWERQUAD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - CASPER"]
    #[inline(always)]
    pub fn casper(&self) -> CASPER_R {
        CASPER_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CASPER RAM"]
    #[inline(always)]
    pub fn casper_ram(&self) -> CASPER_RAM_R {
        CASPER_RAM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Secure GPIO"]
    #[inline(always)]
    pub fn secure_gpio(&self) -> SECURE_GPIO_R {
        SECURE_GPIO_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - HASH"]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - FLEXCOMM 8"]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - FLEXCOMM 9"]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - FLEXCOMM 10"]
    #[inline(always)]
    pub fn flexcomm10(&self) -> FLEXCOMM10_R {
        FLEXCOMM10_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - POWERQUAD"]
    #[inline(always)]
    #[must_use]
    pub fn powerquad(&mut self) -> POWERQUAD_W<0> {
        POWERQUAD_W::new(self)
    }
    #[doc = "Bits 4:5 - CASPER"]
    #[inline(always)]
    #[must_use]
    pub fn casper(&mut self) -> CASPER_W<4> {
        CASPER_W::new(self)
    }
    #[doc = "Bits 8:9 - CASPER RAM"]
    #[inline(always)]
    #[must_use]
    pub fn casper_ram(&mut self) -> CASPER_RAM_W<8> {
        CASPER_RAM_W::new(self)
    }
    #[doc = "Bits 12:13 - Secure GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn secure_gpio(&mut self) -> SECURE_GPIO_W<12> {
        SECURE_GPIO_W::new(self)
    }
    #[doc = "Bits 16:17 - HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hash(&mut self) -> HASH_W<16> {
        HASH_W::new(self)
    }
    #[doc = "Bits 20:21 - FLEXCOMM 8"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm8(&mut self) -> FLEXCOMM8_W<20> {
        FLEXCOMM8_W::new(self)
    }
    #[doc = "Bits 24:25 - FLEXCOMM 9"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm9(&mut self) -> FLEXCOMM9_W<24> {
        FLEXCOMM9_W::new(self)
    }
    #[doc = "Bits 28:29 - FLEXCOMM 10"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm10(&mut self) -> FLEXCOMM10_W<28> {
        FLEXCOMM10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Peripheral 3 Slave Rule 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_periph3_slave_rule0](index.html) module"]
pub struct AHB_PERIPH3_SLAVE_RULE0_SPEC;
impl crate::RegisterSpec for AHB_PERIPH3_SLAVE_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_periph3_slave_rule0::R](R) reader structure"]
impl crate::Readable for AHB_PERIPH3_SLAVE_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_periph3_slave_rule0::W](W) writer structure"]
impl crate::Writable for AHB_PERIPH3_SLAVE_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH3_SLAVE_RULE0 to value 0"]
impl crate::Resettable for AHB_PERIPH3_SLAVE_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
