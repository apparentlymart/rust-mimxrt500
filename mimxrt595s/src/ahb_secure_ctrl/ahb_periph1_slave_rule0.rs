#[doc = "Register `AHB_PERIPH1_SLAVE_RULE0` reader"]
pub struct R(crate::R<AHB_PERIPH1_SLAVE_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_PERIPH1_SLAVE_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_PERIPH1_SLAVE_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_PERIPH1_SLAVE_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_PERIPH1_SLAVE_RULE0` writer"]
pub struct W(crate::W<AHB_PERIPH1_SLAVE_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_PERIPH1_SLAVE_RULE0_SPEC>;
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
impl From<crate::W<AHB_PERIPH1_SLAVE_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_PERIPH1_SLAVE_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC` reader - CRC"]
pub type CRC_R = crate::FieldReader<u8, CRC_A>;
#[doc = "CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRC_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as _
    }
}
impl CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            0 => CRC_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CRC_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CRC_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CRC_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CRC_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CRC_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CRC_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CRC_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CRC` writer - CRC"]
pub type CRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH1_SLAVE_RULE0_SPEC, u8, CRC_A, 2, O>;
impl<'a, const O: u8> CRC_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CRC_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CRC_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CRC_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CRC_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `DMIC0` reader - DMIC0"]
pub type DMIC0_R = crate::FieldReader<u8, DMIC0_A>;
#[doc = "DMIC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMIC0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<DMIC0_A> for u8 {
    #[inline(always)]
    fn from(variant: DMIC0_A) -> Self {
        variant as _
    }
}
impl DMIC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_A {
        match self.bits {
            0 => DMIC0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => DMIC0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => DMIC0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => DMIC0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == DMIC0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == DMIC0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == DMIC0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == DMIC0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `DMIC0` writer - DMIC0"]
pub type DMIC0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH1_SLAVE_RULE0_SPEC, u8, DMIC0_A, 2, O>;
impl<'a, const O: u8> DMIC0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DMIC0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DMIC0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DMIC0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DMIC0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM4` reader - FLEXCOMM 4"]
pub type FLEXCOMM4_R = crate::FieldReader<u8, FLEXCOMM4_A>;
#[doc = "FLEXCOMM 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM4_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM4_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM4_A {
        match self.bits {
            0 => FLEXCOMM4_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM4_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM4_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM4_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM4_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM4_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM4_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM4_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM4` writer - FLEXCOMM 4"]
pub type FLEXCOMM4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH1_SLAVE_RULE0_SPEC, u8, FLEXCOMM4_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM4_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM4_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM4_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM4_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM4_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM5` reader - FLEXCOMM 5"]
pub type FLEXCOMM5_R = crate::FieldReader<u8, FLEXCOMM5_A>;
#[doc = "FLEXCOMM 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM5_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM5_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM5_A {
        match self.bits {
            0 => FLEXCOMM5_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM5_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM5_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM5_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM5_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM5_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM5_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM5_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM5` writer - FLEXCOMM 5"]
pub type FLEXCOMM5_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH1_SLAVE_RULE0_SPEC, u8, FLEXCOMM5_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM5_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM5_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM5_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM5_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM5_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM6` reader - FLEXCOMM 6"]
pub type FLEXCOMM6_R = crate::FieldReader<u8, FLEXCOMM6_A>;
#[doc = "FLEXCOMM 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM6_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM6_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM6_A {
        match self.bits {
            0 => FLEXCOMM6_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM6_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM6_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM6_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM6_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM6_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM6_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM6_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM6` writer - FLEXCOMM 6"]
pub type FLEXCOMM6_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH1_SLAVE_RULE0_SPEC, u8, FLEXCOMM6_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM6_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM6_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM6_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM6_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM6_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM7` reader - FLEXCOMM 7"]
pub type FLEXCOMM7_R = crate::FieldReader<u8, FLEXCOMM7_A>;
#[doc = "FLEXCOMM 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM7_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM7_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM7_A {
        match self.bits {
            0 => FLEXCOMM7_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM7_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM7_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM7_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM7_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM7_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM7_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM7_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM7` writer - FLEXCOMM 7"]
pub type FLEXCOMM7_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH1_SLAVE_RULE0_SPEC, u8, FLEXCOMM7_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM7_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM7_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM7_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM7_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM7_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM14` reader - FLEXCOMM 14"]
pub type FLEXCOMM14_R = crate::FieldReader<u8, FLEXCOMM14_A>;
#[doc = "FLEXCOMM 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM14_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM14_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM14_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM14_A {
        match self.bits {
            0 => FLEXCOMM14_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM14_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM14_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM14_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM14_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM14_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM14_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM14_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM14` writer - FLEXCOMM 14"]
pub type FLEXCOMM14_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH1_SLAVE_RULE0_SPEC, u8, FLEXCOMM14_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM14_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM15` reader - FLEXCOMM 15"]
pub type FLEXCOMM15_R = crate::FieldReader<u8, FLEXCOMM15_A>;
#[doc = "FLEXCOMM 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM15_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM15_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM15_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM15_A {
        match self.bits {
            0 => FLEXCOMM15_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM15_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM15_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM15_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM15_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM15_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM15_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM15_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM15` writer - FLEXCOMM 15"]
pub type FLEXCOMM15_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH1_SLAVE_RULE0_SPEC, u8, FLEXCOMM15_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM15_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM15_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM15_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM15_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM15_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - DMIC0"]
    #[inline(always)]
    pub fn dmic0(&self) -> DMIC0_R {
        DMIC0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - FLEXCOMM 4"]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - FLEXCOMM 5"]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - FLEXCOMM 6"]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - FLEXCOMM 7"]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - FLEXCOMM 14"]
    #[inline(always)]
    pub fn flexcomm14(&self) -> FLEXCOMM14_R {
        FLEXCOMM14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - FLEXCOMM 15"]
    #[inline(always)]
    pub fn flexcomm15(&self) -> FLEXCOMM15_R {
        FLEXCOMM15_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<0> {
        CRC_W::new(self)
    }
    #[doc = "Bits 4:5 - DMIC0"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0(&mut self) -> DMIC0_W<4> {
        DMIC0_W::new(self)
    }
    #[doc = "Bits 8:9 - FLEXCOMM 4"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W<8> {
        FLEXCOMM4_W::new(self)
    }
    #[doc = "Bits 12:13 - FLEXCOMM 5"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W<12> {
        FLEXCOMM5_W::new(self)
    }
    #[doc = "Bits 16:17 - FLEXCOMM 6"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W<16> {
        FLEXCOMM6_W::new(self)
    }
    #[doc = "Bits 20:21 - FLEXCOMM 7"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W<20> {
        FLEXCOMM7_W::new(self)
    }
    #[doc = "Bits 24:25 - FLEXCOMM 14"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14(&mut self) -> FLEXCOMM14_W<24> {
        FLEXCOMM14_W::new(self)
    }
    #[doc = "Bits 28:29 - FLEXCOMM 15"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm15(&mut self) -> FLEXCOMM15_W<28> {
        FLEXCOMM15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Peripheral 1 Slave Rule 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_periph1_slave_rule0](index.html) module"]
pub struct AHB_PERIPH1_SLAVE_RULE0_SPEC;
impl crate::RegisterSpec for AHB_PERIPH1_SLAVE_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_periph1_slave_rule0::R](R) reader structure"]
impl crate::Readable for AHB_PERIPH1_SLAVE_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_periph1_slave_rule0::W](W) writer structure"]
impl crate::Writable for AHB_PERIPH1_SLAVE_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH1_SLAVE_RULE0 to value 0"]
impl crate::Resettable for AHB_PERIPH1_SLAVE_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
