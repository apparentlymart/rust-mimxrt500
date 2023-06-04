#[doc = "Register `APB_BRIDGE_PER0_RULE0` reader"]
pub struct R(crate::R<APB_BRIDGE_PER0_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_BRIDGE_PER0_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_BRIDGE_PER0_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_BRIDGE_PER0_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_BRIDGE_PER0_RULE0` writer"]
pub struct W(crate::W<APB_BRIDGE_PER0_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_BRIDGE_PER0_RULE0_SPEC>;
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
impl From<crate::W<APB_BRIDGE_PER0_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_BRIDGE_PER0_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTCTL_A` reader - RSTCTL_A"]
pub type RSTCTL_A_R = crate::FieldReader<u8, RSTCTL_A_A>;
#[doc = "RSTCTL_A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTCTL_A_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<RSTCTL_A_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTCTL_A_A) -> Self {
        variant as _
    }
}
impl RSTCTL_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTCTL_A_A {
        match self.bits {
            0 => RSTCTL_A_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => RSTCTL_A_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => RSTCTL_A_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => RSTCTL_A_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == RSTCTL_A_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == RSTCTL_A_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == RSTCTL_A_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == RSTCTL_A_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `RSTCTL_A` writer - RSTCTL_A"]
pub type RSTCTL_A_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER0_RULE0_SPEC, u8, RSTCTL_A_A, 2, O>;
impl<'a, const O: u8> RSTCTL_A_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RSTCTL_A_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RSTCTL_A_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RSTCTL_A_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RSTCTL_A_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CLKCTL_A` reader - CLKCTL_A"]
pub type CLKCTL_A_R = crate::FieldReader<u8, CLKCTL_A_A>;
#[doc = "CLKCTL_A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKCTL_A_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CLKCTL_A_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKCTL_A_A) -> Self {
        variant as _
    }
}
impl CLKCTL_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKCTL_A_A {
        match self.bits {
            0 => CLKCTL_A_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CLKCTL_A_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CLKCTL_A_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CLKCTL_A_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CLKCTL_A_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CLKCTL_A_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CLKCTL_A_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CLKCTL_A_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CLKCTL_A` writer - CLKCTL_A"]
pub type CLKCTL_A_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER0_RULE0_SPEC, u8, CLKCTL_A_A, 2, O>;
impl<'a, const O: u8> CLKCTL_A_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CLKCTL_A_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CLKCTL_A_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CLKCTL_A_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CLKCTL_A_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `SYSCTL_A` reader - SYSCTL_A"]
pub type SYSCTL_A_R = crate::FieldReader<u8, SYSCTL_A_A>;
#[doc = "SYSCTL_A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_A_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<SYSCTL_A_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_A_A) -> Self {
        variant as _
    }
}
impl SYSCTL_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_A_A {
        match self.bits {
            0 => SYSCTL_A_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => SYSCTL_A_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => SYSCTL_A_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => SYSCTL_A_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == SYSCTL_A_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == SYSCTL_A_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == SYSCTL_A_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == SYSCTL_A_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `SYSCTL_A` writer - SYSCTL_A"]
pub type SYSCTL_A_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER0_RULE0_SPEC, u8, SYSCTL_A_A, 2, O>;
impl<'a, const O: u8> SYSCTL_A_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SYSCTL_A_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SYSCTL_A_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SYSCTL_A_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SYSCTL_A_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `PVT` reader - PVT"]
pub type PVT_R = crate::FieldReader<u8, PVT_A>;
#[doc = "PVT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PVT_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<PVT_A> for u8 {
    #[inline(always)]
    fn from(variant: PVT_A) -> Self {
        variant as _
    }
}
impl PVT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVT_A {
        match self.bits {
            0 => PVT_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => PVT_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => PVT_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => PVT_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == PVT_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == PVT_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == PVT_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == PVT_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `PVT` writer - PVT"]
pub type PVT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER0_RULE0_SPEC, u8, PVT_A, 2, O>;
impl<'a, const O: u8> PVT_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(PVT_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(PVT_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(PVT_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(PVT_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `IOCON` reader - IOCON"]
pub type IOCON_R = crate::FieldReader<u8, IOCON_A>;
#[doc = "IOCON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOCON_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<IOCON_A> for u8 {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        variant as _
    }
}
impl IOCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            0 => IOCON_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => IOCON_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => IOCON_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => IOCON_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == IOCON_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == IOCON_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == IOCON_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == IOCON_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `IOCON` writer - IOCON"]
pub type IOCON_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER0_RULE0_SPEC, u8, IOCON_A, 2, O>;
impl<'a, const O: u8> IOCON_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(IOCON_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(IOCON_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(IOCON_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(IOCON_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `PUF` reader - PUF"]
pub type PUF_R = crate::FieldReader<u8, PUF_A>;
#[doc = "PUF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUF_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<PUF_A> for u8 {
    #[inline(always)]
    fn from(variant: PUF_A) -> Self {
        variant as _
    }
}
impl PUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_A {
        match self.bits {
            0 => PUF_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => PUF_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => PUF_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => PUF_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == PUF_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == PUF_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == PUF_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == PUF_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `PUF` writer - PUF"]
pub type PUF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER0_RULE0_SPEC, u8, PUF_A, 2, O>;
impl<'a, const O: u8> PUF_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(PUF_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(PUF_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(PUF_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(PUF_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - RSTCTL_A"]
    #[inline(always)]
    pub fn rstctl_a(&self) -> RSTCTL_A_R {
        RSTCTL_A_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - CLKCTL_A"]
    #[inline(always)]
    pub fn clkctl_a(&self) -> CLKCTL_A_R {
        CLKCTL_A_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SYSCTL_A"]
    #[inline(always)]
    pub fn sysctl_a(&self) -> SYSCTL_A_R {
        SYSCTL_A_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PVT"]
    #[inline(always)]
    pub fn pvt(&self) -> PVT_R {
        PVT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - IOCON"]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PUF"]
    #[inline(always)]
    pub fn puf(&self) -> PUF_R {
        PUF_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RSTCTL_A"]
    #[inline(always)]
    #[must_use]
    pub fn rstctl_a(&mut self) -> RSTCTL_A_W<0> {
        RSTCTL_A_W::new(self)
    }
    #[doc = "Bits 4:5 - CLKCTL_A"]
    #[inline(always)]
    #[must_use]
    pub fn clkctl_a(&mut self) -> CLKCTL_A_W<4> {
        CLKCTL_A_W::new(self)
    }
    #[doc = "Bits 8:9 - SYSCTL_A"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_a(&mut self) -> SYSCTL_A_W<8> {
        SYSCTL_A_W::new(self)
    }
    #[doc = "Bits 12:13 - PVT"]
    #[inline(always)]
    #[must_use]
    pub fn pvt(&mut self) -> PVT_W<12> {
        PVT_W::new(self)
    }
    #[doc = "Bits 16:17 - IOCON"]
    #[inline(always)]
    #[must_use]
    pub fn iocon(&mut self) -> IOCON_W<16> {
        IOCON_W::new(self)
    }
    #[doc = "Bits 24:25 - PUF"]
    #[inline(always)]
    #[must_use]
    pub fn puf(&mut self) -> PUF_W<24> {
        PUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Bridge Peripheral 0 Rule 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_bridge_per0_rule0](index.html) module"]
pub struct APB_BRIDGE_PER0_RULE0_SPEC;
impl crate::RegisterSpec for APB_BRIDGE_PER0_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_bridge_per0_rule0::R](R) reader structure"]
impl crate::Readable for APB_BRIDGE_PER0_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_bridge_per0_rule0::W](W) writer structure"]
impl crate::Writable for APB_BRIDGE_PER0_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_BRIDGE_PER0_RULE0 to value 0"]
impl crate::Resettable for APB_BRIDGE_PER0_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
