#[doc = "Register `AHB_PERIPH0_SLAVE_RULE0` reader"]
pub struct R(crate::R<AHB_PERIPH0_SLAVE_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_PERIPH0_SLAVE_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_PERIPH0_SLAVE_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_PERIPH0_SLAVE_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_PERIPH0_SLAVE_RULE0` writer"]
pub struct W(crate::W<AHB_PERIPH0_SLAVE_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_PERIPH0_SLAVE_RULE0_SPEC>;
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
impl From<crate::W<AHB_PERIPH0_SLAVE_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_PERIPH0_SLAVE_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSGPIO` reader - HSGPIO"]
pub type HSGPIO_R = crate::FieldReader<u8, HSGPIO_A>;
#[doc = "HSGPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSGPIO_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<HSGPIO_A> for u8 {
    #[inline(always)]
    fn from(variant: HSGPIO_A) -> Self {
        variant as _
    }
}
impl HSGPIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO_A {
        match self.bits {
            0 => HSGPIO_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => HSGPIO_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => HSGPIO_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => HSGPIO_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == HSGPIO_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == HSGPIO_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == HSGPIO_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == HSGPIO_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `HSGPIO` writer - HSGPIO"]
pub type HSGPIO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH0_SLAVE_RULE0_SPEC, u8, HSGPIO_A, 2, O>;
impl<'a, const O: u8> HSGPIO_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(HSGPIO_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(HSGPIO_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(HSGPIO_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(HSGPIO_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `DMA0` reader - DMA 0"]
pub type DMA0_R = crate::FieldReader<u8, DMA0_A>;
#[doc = "DMA 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<DMA0_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0_A) -> Self {
        variant as _
    }
}
impl DMA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_A {
        match self.bits {
            0 => DMA0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => DMA0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => DMA0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => DMA0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == DMA0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == DMA0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == DMA0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == DMA0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `DMA0` writer - DMA 0"]
pub type DMA0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH0_SLAVE_RULE0_SPEC, u8, DMA0_A, 2, O>;
impl<'a, const O: u8> DMA0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DMA0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DMA0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DMA0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DMA0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `DMA1` reader - DMA 1"]
pub type DMA1_R = crate::FieldReader<u8, DMA1_A>;
#[doc = "DMA 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<DMA1_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1_A) -> Self {
        variant as _
    }
}
impl DMA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_A {
        match self.bits {
            0 => DMA1_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => DMA1_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => DMA1_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => DMA1_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == DMA1_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == DMA1_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == DMA1_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == DMA1_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `DMA1` writer - DMA 1"]
pub type DMA1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH0_SLAVE_RULE0_SPEC, u8, DMA1_A, 2, O>;
impl<'a, const O: u8> DMA1_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DMA1_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DMA1_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DMA1_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DMA1_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM0` reader - FLEXCOMM 0"]
pub type FLEXCOMM0_R = crate::FieldReader<u8, FLEXCOMM0_A>;
#[doc = "FLEXCOMM 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM0_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM0_A {
        match self.bits {
            0 => FLEXCOMM0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM0` writer - FLEXCOMM 0"]
pub type FLEXCOMM0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH0_SLAVE_RULE0_SPEC, u8, FLEXCOMM0_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM1` reader - FLEXCOMM 1"]
pub type FLEXCOMM1_R = crate::FieldReader<u8, FLEXCOMM1_A>;
#[doc = "FLEXCOMM 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM1_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM1_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM1_A {
        match self.bits {
            0 => FLEXCOMM1_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM1_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM1_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM1_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM1_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM1_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM1_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM1_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM1` writer - FLEXCOMM 1"]
pub type FLEXCOMM1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH0_SLAVE_RULE0_SPEC, u8, FLEXCOMM1_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM1_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM1_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM1_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM1_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM1_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM2` reader - FLEXCOMM 2"]
pub type FLEXCOMM2_R = crate::FieldReader<u8, FLEXCOMM2_A>;
#[doc = "FLEXCOMM 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM2_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM2_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM2_A {
        match self.bits {
            0 => FLEXCOMM2_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM2_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM2_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM2_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM2_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM2_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM2_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM2_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM2` writer - FLEXCOMM 2"]
pub type FLEXCOMM2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH0_SLAVE_RULE0_SPEC, u8, FLEXCOMM2_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM2_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM2_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM2_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM2_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM2_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM3` reader - FLEXCOMM 3"]
pub type FLEXCOMM3_R = crate::FieldReader<u8, FLEXCOMM3_A>;
#[doc = "FLEXCOMM 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM3_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM3_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM3_A {
        match self.bits {
            0 => FLEXCOMM3_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM3_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM3_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM3_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM3_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM3_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM3_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM3_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM3` writer - FLEXCOMM 3"]
pub type FLEXCOMM3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH0_SLAVE_RULE0_SPEC, u8, FLEXCOMM3_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM3_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM3_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM3_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM3_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM3_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `DEBUG_MAILBOX` reader - DEBUG_MAILBOX"]
pub type DEBUG_MAILBOX_R = crate::FieldReader<u8, DEBUG_MAILBOX_A>;
#[doc = "DEBUG_MAILBOX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEBUG_MAILBOX_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<DEBUG_MAILBOX_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBUG_MAILBOX_A) -> Self {
        variant as _
    }
}
impl DEBUG_MAILBOX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUG_MAILBOX_A {
        match self.bits {
            0 => DEBUG_MAILBOX_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => DEBUG_MAILBOX_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => DEBUG_MAILBOX_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => DEBUG_MAILBOX_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == DEBUG_MAILBOX_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == DEBUG_MAILBOX_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == DEBUG_MAILBOX_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == DEBUG_MAILBOX_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `DEBUG_MAILBOX` writer - DEBUG_MAILBOX"]
pub type DEBUG_MAILBOX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH0_SLAVE_RULE0_SPEC, u8, DEBUG_MAILBOX_A, 2, O>;
impl<'a, const O: u8> DEBUG_MAILBOX_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DEBUG_MAILBOX_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DEBUG_MAILBOX_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DEBUG_MAILBOX_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DEBUG_MAILBOX_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - HSGPIO"]
    #[inline(always)]
    pub fn hsgpio(&self) -> HSGPIO_R {
        HSGPIO_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - DMA 0"]
    #[inline(always)]
    pub fn dma0(&self) -> DMA0_R {
        DMA0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DMA 1"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - FLEXCOMM 0"]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - FLEXCOMM 1"]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - FLEXCOMM 2"]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - FLEXCOMM 3"]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DEBUG_MAILBOX"]
    #[inline(always)]
    pub fn debug_mailbox(&self) -> DEBUG_MAILBOX_R {
        DEBUG_MAILBOX_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - HSGPIO"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio(&mut self) -> HSGPIO_W<0> {
        HSGPIO_W::new(self)
    }
    #[doc = "Bits 4:5 - DMA 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma0(&mut self) -> DMA0_W<4> {
        DMA0_W::new(self)
    }
    #[doc = "Bits 8:9 - DMA 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma1(&mut self) -> DMA1_W<8> {
        DMA1_W::new(self)
    }
    #[doc = "Bits 12:13 - FLEXCOMM 0"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W<12> {
        FLEXCOMM0_W::new(self)
    }
    #[doc = "Bits 16:17 - FLEXCOMM 1"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W<16> {
        FLEXCOMM1_W::new(self)
    }
    #[doc = "Bits 20:21 - FLEXCOMM 2"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W<20> {
        FLEXCOMM2_W::new(self)
    }
    #[doc = "Bits 24:25 - FLEXCOMM 3"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W<24> {
        FLEXCOMM3_W::new(self)
    }
    #[doc = "Bits 28:29 - DEBUG_MAILBOX"]
    #[inline(always)]
    #[must_use]
    pub fn debug_mailbox(&mut self) -> DEBUG_MAILBOX_W<28> {
        DEBUG_MAILBOX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Peripheral 0 Slave Rule 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_periph0_slave_rule0](index.html) module"]
pub struct AHB_PERIPH0_SLAVE_RULE0_SPEC;
impl crate::RegisterSpec for AHB_PERIPH0_SLAVE_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_periph0_slave_rule0::R](R) reader structure"]
impl crate::Readable for AHB_PERIPH0_SLAVE_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_periph0_slave_rule0::W](W) writer structure"]
impl crate::Writable for AHB_PERIPH0_SLAVE_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH0_SLAVE_RULE0 to value 0"]
impl crate::Resettable for AHB_PERIPH0_SLAVE_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
