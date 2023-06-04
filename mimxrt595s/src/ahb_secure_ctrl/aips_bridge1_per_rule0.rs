#[doc = "Register `AIPS_BRIDGE1_PER_RULE0` reader"]
pub struct R(crate::R<AIPS_BRIDGE1_PER_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIPS_BRIDGE1_PER_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIPS_BRIDGE1_PER_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIPS_BRIDGE1_PER_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIPS_BRIDGE1_PER_RULE0` writer"]
pub struct W(crate::W<AIPS_BRIDGE1_PER_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIPS_BRIDGE1_PER_RULE0_SPEC>;
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
impl From<crate::W<AIPS_BRIDGE1_PER_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIPS_BRIDGE1_PER_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTP_CONTROLLER_0` reader - OTP Controller 0"]
pub type OTP_CONTROLLER_0_R = crate::FieldReader<u8, OTP_CONTROLLER_0_A>;
#[doc = "OTP Controller 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OTP_CONTROLLER_0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<OTP_CONTROLLER_0_A> for u8 {
    #[inline(always)]
    fn from(variant: OTP_CONTROLLER_0_A) -> Self {
        variant as _
    }
}
impl OTP_CONTROLLER_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTP_CONTROLLER_0_A {
        match self.bits {
            0 => OTP_CONTROLLER_0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => OTP_CONTROLLER_0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => OTP_CONTROLLER_0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => OTP_CONTROLLER_0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `OTP_CONTROLLER_0` writer - OTP Controller 0"]
pub type OTP_CONTROLLER_0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE0_SPEC, u8, OTP_CONTROLLER_0_A, 2, O>;
impl<'a, const O: u8> OTP_CONTROLLER_0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `OTP_CONTROLLER_1` reader - OTP Controller 1"]
pub type OTP_CONTROLLER_1_R = crate::FieldReader<u8, OTP_CONTROLLER_1_A>;
#[doc = "OTP Controller 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OTP_CONTROLLER_1_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<OTP_CONTROLLER_1_A> for u8 {
    #[inline(always)]
    fn from(variant: OTP_CONTROLLER_1_A) -> Self {
        variant as _
    }
}
impl OTP_CONTROLLER_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTP_CONTROLLER_1_A {
        match self.bits {
            0 => OTP_CONTROLLER_1_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => OTP_CONTROLLER_1_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => OTP_CONTROLLER_1_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => OTP_CONTROLLER_1_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_1_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_1_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_1_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_1_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `OTP_CONTROLLER_1` writer - OTP Controller 1"]
pub type OTP_CONTROLLER_1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE0_SPEC, u8, OTP_CONTROLLER_1_A, 2, O>;
impl<'a, const O: u8> OTP_CONTROLLER_1_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_1_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_1_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_1_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_1_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `OTP_CONTROLLER_2` reader - OTP Controller 2"]
pub type OTP_CONTROLLER_2_R = crate::FieldReader<u8, OTP_CONTROLLER_2_A>;
#[doc = "OTP Controller 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OTP_CONTROLLER_2_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<OTP_CONTROLLER_2_A> for u8 {
    #[inline(always)]
    fn from(variant: OTP_CONTROLLER_2_A) -> Self {
        variant as _
    }
}
impl OTP_CONTROLLER_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTP_CONTROLLER_2_A {
        match self.bits {
            0 => OTP_CONTROLLER_2_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => OTP_CONTROLLER_2_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => OTP_CONTROLLER_2_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => OTP_CONTROLLER_2_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_2_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_2_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_2_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_2_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `OTP_CONTROLLER_2` writer - OTP Controller 2"]
pub type OTP_CONTROLLER_2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE0_SPEC, u8, OTP_CONTROLLER_2_A, 2, O>;
impl<'a, const O: u8> OTP_CONTROLLER_2_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_2_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_2_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_2_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_2_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `OTP_CONTROLLER_3` reader - OTP Controller 3"]
pub type OTP_CONTROLLER_3_R = crate::FieldReader<u8, OTP_CONTROLLER_3_A>;
#[doc = "OTP Controller 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OTP_CONTROLLER_3_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<OTP_CONTROLLER_3_A> for u8 {
    #[inline(always)]
    fn from(variant: OTP_CONTROLLER_3_A) -> Self {
        variant as _
    }
}
impl OTP_CONTROLLER_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTP_CONTROLLER_3_A {
        match self.bits {
            0 => OTP_CONTROLLER_3_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => OTP_CONTROLLER_3_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => OTP_CONTROLLER_3_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => OTP_CONTROLLER_3_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_3_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_3_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_3_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == OTP_CONTROLLER_3_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `OTP_CONTROLLER_3` writer - OTP Controller 3"]
pub type OTP_CONTROLLER_3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE0_SPEC, u8, OTP_CONTROLLER_3_A, 2, O>;
impl<'a, const O: u8> OTP_CONTROLLER_3_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_3_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_3_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_3_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OTP_CONTROLLER_3_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXSPI0_REGISTERS` reader - FLEXSPI0 Registers"]
pub type FLEXSPI0_REGISTERS_R = crate::FieldReader<u8, FLEXSPI0_REGISTERS_A>;
#[doc = "FLEXSPI0 Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXSPI0_REGISTERS_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXSPI0_REGISTERS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXSPI0_REGISTERS_A) -> Self {
        variant as _
    }
}
impl FLEXSPI0_REGISTERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI0_REGISTERS_A {
        match self.bits {
            0 => FLEXSPI0_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXSPI0_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXSPI0_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXSPI0_REGISTERS_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXSPI0_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXSPI0_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXSPI0_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXSPI0_REGISTERS_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXSPI0_REGISTERS` writer - FLEXSPI0 Registers"]
pub type FLEXSPI0_REGISTERS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE0_SPEC, u8, FLEXSPI0_REGISTERS_A, 2, O>;
impl<'a, const O: u8> FLEXSPI0_REGISTERS_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXSPI0_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXSPI0_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXSPI0_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXSPI0_REGISTERS_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `PMC_PMU_CONTROL` reader - PMC (PMU CONTROL)"]
pub type PMC_PMU_CONTROL_R = crate::FieldReader<u8, PMC_PMU_CONTROL_A>;
#[doc = "PMC (PMU CONTROL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMC_PMU_CONTROL_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<PMC_PMU_CONTROL_A> for u8 {
    #[inline(always)]
    fn from(variant: PMC_PMU_CONTROL_A) -> Self {
        variant as _
    }
}
impl PMC_PMU_CONTROL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMC_PMU_CONTROL_A {
        match self.bits {
            0 => PMC_PMU_CONTROL_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => PMC_PMU_CONTROL_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => PMC_PMU_CONTROL_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => PMC_PMU_CONTROL_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == PMC_PMU_CONTROL_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == PMC_PMU_CONTROL_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == PMC_PMU_CONTROL_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == PMC_PMU_CONTROL_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `PMC_PMU_CONTROL` writer - PMC (PMU CONTROL)"]
pub type PMC_PMU_CONTROL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE0_SPEC, u8, PMC_PMU_CONTROL_A, 2, O>;
impl<'a, const O: u8> PMC_PMU_CONTROL_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(PMC_PMU_CONTROL_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(PMC_PMU_CONTROL_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(PMC_PMU_CONTROL_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(PMC_PMU_CONTROL_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `SDIO0_REGISTERS` reader - SDIO 0 Registers"]
pub type SDIO0_REGISTERS_R = crate::FieldReader<u8, SDIO0_REGISTERS_A>;
#[doc = "SDIO 0 Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDIO0_REGISTERS_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<SDIO0_REGISTERS_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIO0_REGISTERS_A) -> Self {
        variant as _
    }
}
impl SDIO0_REGISTERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO0_REGISTERS_A {
        match self.bits {
            0 => SDIO0_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => SDIO0_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => SDIO0_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => SDIO0_REGISTERS_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == SDIO0_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == SDIO0_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == SDIO0_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == SDIO0_REGISTERS_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `SDIO0_REGISTERS` writer - SDIO 0 Registers"]
pub type SDIO0_REGISTERS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE0_SPEC, u8, SDIO0_REGISTERS_A, 2, O>;
impl<'a, const O: u8> SDIO0_REGISTERS_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SDIO0_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SDIO0_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SDIO0_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SDIO0_REGISTERS_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `SDIO1_REGISTERS` reader - SDIO 1 Registers"]
pub type SDIO1_REGISTERS_R = crate::FieldReader<u8, SDIO1_REGISTERS_A>;
#[doc = "SDIO 1 Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDIO1_REGISTERS_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<SDIO1_REGISTERS_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIO1_REGISTERS_A) -> Self {
        variant as _
    }
}
impl SDIO1_REGISTERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO1_REGISTERS_A {
        match self.bits {
            0 => SDIO1_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => SDIO1_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => SDIO1_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => SDIO1_REGISTERS_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == SDIO1_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == SDIO1_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == SDIO1_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == SDIO1_REGISTERS_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `SDIO1_REGISTERS` writer - SDIO 1 Registers"]
pub type SDIO1_REGISTERS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE0_SPEC, u8, SDIO1_REGISTERS_A, 2, O>;
impl<'a, const O: u8> SDIO1_REGISTERS_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SDIO1_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SDIO1_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SDIO1_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SDIO1_REGISTERS_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - OTP Controller 0"]
    #[inline(always)]
    pub fn otp_controller_0(&self) -> OTP_CONTROLLER_0_R {
        OTP_CONTROLLER_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - OTP Controller 1"]
    #[inline(always)]
    pub fn otp_controller_1(&self) -> OTP_CONTROLLER_1_R {
        OTP_CONTROLLER_1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - OTP Controller 2"]
    #[inline(always)]
    pub fn otp_controller_2(&self) -> OTP_CONTROLLER_2_R {
        OTP_CONTROLLER_2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - OTP Controller 3"]
    #[inline(always)]
    pub fn otp_controller_3(&self) -> OTP_CONTROLLER_3_R {
        OTP_CONTROLLER_3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - FLEXSPI0 Registers"]
    #[inline(always)]
    pub fn flexspi0_registers(&self) -> FLEXSPI0_REGISTERS_R {
        FLEXSPI0_REGISTERS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PMC (PMU CONTROL)"]
    #[inline(always)]
    pub fn pmc_pmu_control(&self) -> PMC_PMU_CONTROL_R {
        PMC_PMU_CONTROL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - SDIO 0 Registers"]
    #[inline(always)]
    pub fn sdio0_registers(&self) -> SDIO0_REGISTERS_R {
        SDIO0_REGISTERS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - SDIO 1 Registers"]
    #[inline(always)]
    pub fn sdio1_registers(&self) -> SDIO1_REGISTERS_R {
        SDIO1_REGISTERS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OTP Controller 0"]
    #[inline(always)]
    #[must_use]
    pub fn otp_controller_0(&mut self) -> OTP_CONTROLLER_0_W<0> {
        OTP_CONTROLLER_0_W::new(self)
    }
    #[doc = "Bits 4:5 - OTP Controller 1"]
    #[inline(always)]
    #[must_use]
    pub fn otp_controller_1(&mut self) -> OTP_CONTROLLER_1_W<4> {
        OTP_CONTROLLER_1_W::new(self)
    }
    #[doc = "Bits 8:9 - OTP Controller 2"]
    #[inline(always)]
    #[must_use]
    pub fn otp_controller_2(&mut self) -> OTP_CONTROLLER_2_W<8> {
        OTP_CONTROLLER_2_W::new(self)
    }
    #[doc = "Bits 12:13 - OTP Controller 3"]
    #[inline(always)]
    #[must_use]
    pub fn otp_controller_3(&mut self) -> OTP_CONTROLLER_3_W<12> {
        OTP_CONTROLLER_3_W::new(self)
    }
    #[doc = "Bits 16:17 - FLEXSPI0 Registers"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_registers(&mut self) -> FLEXSPI0_REGISTERS_W<16> {
        FLEXSPI0_REGISTERS_W::new(self)
    }
    #[doc = "Bits 20:21 - PMC (PMU CONTROL)"]
    #[inline(always)]
    #[must_use]
    pub fn pmc_pmu_control(&mut self) -> PMC_PMU_CONTROL_W<20> {
        PMC_PMU_CONTROL_W::new(self)
    }
    #[doc = "Bits 24:25 - SDIO 0 Registers"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0_registers(&mut self) -> SDIO0_REGISTERS_W<24> {
        SDIO0_REGISTERS_W::new(self)
    }
    #[doc = "Bits 28:29 - SDIO 1 Registers"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_registers(&mut self) -> SDIO1_REGISTERS_W<28> {
        SDIO1_REGISTERS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AIPS Bridge Peripheral 1 Rule 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aips_bridge1_per_rule0](index.html) module"]
pub struct AIPS_BRIDGE1_PER_RULE0_SPEC;
impl crate::RegisterSpec for AIPS_BRIDGE1_PER_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aips_bridge1_per_rule0::R](R) reader structure"]
impl crate::Readable for AIPS_BRIDGE1_PER_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aips_bridge1_per_rule0::W](W) writer structure"]
impl crate::Writable for AIPS_BRIDGE1_PER_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIPS_BRIDGE1_PER_RULE0 to value 0"]
impl crate::Resettable for AIPS_BRIDGE1_PER_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
