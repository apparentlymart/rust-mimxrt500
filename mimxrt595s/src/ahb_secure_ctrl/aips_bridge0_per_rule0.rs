#[doc = "Register `AIPS_BRIDGE0_PER_RULE0` reader"]
pub struct R(crate::R<AIPS_BRIDGE0_PER_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIPS_BRIDGE0_PER_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIPS_BRIDGE0_PER_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIPS_BRIDGE0_PER_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIPS_BRIDGE0_PER_RULE0` writer"]
pub struct W(crate::W<AIPS_BRIDGE0_PER_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIPS_BRIDGE0_PER_RULE0_SPEC>;
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
impl From<crate::W<AIPS_BRIDGE0_PER_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIPS_BRIDGE0_PER_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MU0` reader - MU0 (M33 PORT)"]
pub type MU0_R = crate::FieldReader<u8, MU0_A>;
#[doc = "MU0 (M33 PORT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MU0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<MU0_A> for u8 {
    #[inline(always)]
    fn from(variant: MU0_A) -> Self {
        variant as _
    }
}
impl MU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MU0_A {
        match self.bits {
            0 => MU0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => MU0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => MU0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => MU0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == MU0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == MU0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == MU0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == MU0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `MU0` writer - MU0 (M33 PORT)"]
pub type MU0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE0_PER_RULE0_SPEC, u8, MU0_A, 2, O>;
impl<'a, const O: u8> MU0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MU0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MU0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MU0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MU0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `MU1` reader - MU1 (DSP PORT)"]
pub type MU1_R = crate::FieldReader<u8, MU1_A>;
#[doc = "MU1 (DSP PORT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MU1_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<MU1_A> for u8 {
    #[inline(always)]
    fn from(variant: MU1_A) -> Self {
        variant as _
    }
}
impl MU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MU1_A {
        match self.bits {
            0 => MU1_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => MU1_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => MU1_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => MU1_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == MU1_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == MU1_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == MU1_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == MU1_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `MU1` writer - MU1 (DSP PORT)"]
pub type MU1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE0_PER_RULE0_SPEC, u8, MU1_A, 2, O>;
impl<'a, const O: u8> MU1_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MU1_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MU1_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(MU1_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(MU1_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `SEMAPHORE` reader - Semaphore"]
pub type SEMAPHORE_R = crate::FieldReader<u8, SEMAPHORE_A>;
#[doc = "Semaphore\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEMAPHORE_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<SEMAPHORE_A> for u8 {
    #[inline(always)]
    fn from(variant: SEMAPHORE_A) -> Self {
        variant as _
    }
}
impl SEMAPHORE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMAPHORE_A {
        match self.bits {
            0 => SEMAPHORE_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => SEMAPHORE_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => SEMAPHORE_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => SEMAPHORE_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == SEMAPHORE_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == SEMAPHORE_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == SEMAPHORE_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == SEMAPHORE_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `SEMAPHORE` writer - Semaphore"]
pub type SEMAPHORE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE0_PER_RULE0_SPEC, u8, SEMAPHORE_A, 2, O>;
impl<'a, const O: u8> SEMAPHORE_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SEMAPHORE_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SEMAPHORE_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SEMAPHORE_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SEMAPHORE_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `OS_EVENT_TIMER_M33_PORT` reader - OS_EVENT TIMER (M33 PORT)"]
pub type OS_EVENT_TIMER_M33_PORT_R = crate::FieldReader<u8, OS_EVENT_TIMER_M33_PORT_A>;
#[doc = "OS_EVENT TIMER (M33 PORT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OS_EVENT_TIMER_M33_PORT_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<OS_EVENT_TIMER_M33_PORT_A> for u8 {
    #[inline(always)]
    fn from(variant: OS_EVENT_TIMER_M33_PORT_A) -> Self {
        variant as _
    }
}
impl OS_EVENT_TIMER_M33_PORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OS_EVENT_TIMER_M33_PORT_A {
        match self.bits {
            0 => OS_EVENT_TIMER_M33_PORT_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => OS_EVENT_TIMER_M33_PORT_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => OS_EVENT_TIMER_M33_PORT_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => OS_EVENT_TIMER_M33_PORT_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == OS_EVENT_TIMER_M33_PORT_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == OS_EVENT_TIMER_M33_PORT_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == OS_EVENT_TIMER_M33_PORT_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == OS_EVENT_TIMER_M33_PORT_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `OS_EVENT_TIMER_M33_PORT` writer - OS_EVENT TIMER (M33 PORT)"]
pub type OS_EVENT_TIMER_M33_PORT_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    AIPS_BRIDGE0_PER_RULE0_SPEC,
    u8,
    OS_EVENT_TIMER_M33_PORT_A,
    2,
    O,
>;
impl<'a, const O: u8> OS_EVENT_TIMER_M33_PORT_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_M33_PORT_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_M33_PORT_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_M33_PORT_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_M33_PORT_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `OS_EVENT_TIMER_DSP_PORT` reader - OS_EVENT TIMER (DSP PORT)"]
pub type OS_EVENT_TIMER_DSP_PORT_R = crate::FieldReader<u8, OS_EVENT_TIMER_DSP_PORT_A>;
#[doc = "OS_EVENT TIMER (DSP PORT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OS_EVENT_TIMER_DSP_PORT_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<OS_EVENT_TIMER_DSP_PORT_A> for u8 {
    #[inline(always)]
    fn from(variant: OS_EVENT_TIMER_DSP_PORT_A) -> Self {
        variant as _
    }
}
impl OS_EVENT_TIMER_DSP_PORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OS_EVENT_TIMER_DSP_PORT_A {
        match self.bits {
            0 => OS_EVENT_TIMER_DSP_PORT_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => OS_EVENT_TIMER_DSP_PORT_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => OS_EVENT_TIMER_DSP_PORT_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => OS_EVENT_TIMER_DSP_PORT_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == OS_EVENT_TIMER_DSP_PORT_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == OS_EVENT_TIMER_DSP_PORT_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == OS_EVENT_TIMER_DSP_PORT_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == OS_EVENT_TIMER_DSP_PORT_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `OS_EVENT_TIMER_DSP_PORT` writer - OS_EVENT TIMER (DSP PORT)"]
pub type OS_EVENT_TIMER_DSP_PORT_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    AIPS_BRIDGE0_PER_RULE0_SPEC,
    u8,
    OS_EVENT_TIMER_DSP_PORT_A,
    2,
    O,
>;
impl<'a, const O: u8> OS_EVENT_TIMER_DSP_PORT_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_DSP_PORT_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_DSP_PORT_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_DSP_PORT_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_DSP_PORT_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `ROM` reader - ROM"]
pub type ROM_R = crate::FieldReader<u8, ROM_A>;
#[doc = "ROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ROM_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<ROM_A> for u8 {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        variant as _
    }
}
impl ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            0 => ROM_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => ROM_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => ROM_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => ROM_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == ROM_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == ROM_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == ROM_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == ROM_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `ROM` writer - ROM"]
pub type ROM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE0_PER_RULE0_SPEC, u8, ROM_A, 2, O>;
impl<'a, const O: u8> ROM_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(ROM_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(ROM_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(ROM_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(ROM_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - MU0 (M33 PORT)"]
    #[inline(always)]
    pub fn mu0(&self) -> MU0_R {
        MU0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - MU1 (DSP PORT)"]
    #[inline(always)]
    pub fn mu1(&self) -> MU1_R {
        MU1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Semaphore"]
    #[inline(always)]
    pub fn semaphore(&self) -> SEMAPHORE_R {
        SEMAPHORE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - OS_EVENT TIMER (M33 PORT)"]
    #[inline(always)]
    pub fn os_event_timer_m33_port(&self) -> OS_EVENT_TIMER_M33_PORT_R {
        OS_EVENT_TIMER_M33_PORT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - OS_EVENT TIMER (DSP PORT)"]
    #[inline(always)]
    pub fn os_event_timer_dsp_port(&self) -> OS_EVENT_TIMER_DSP_PORT_R {
        OS_EVENT_TIMER_DSP_PORT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - ROM"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MU0 (M33 PORT)"]
    #[inline(always)]
    #[must_use]
    pub fn mu0(&mut self) -> MU0_W<0> {
        MU0_W::new(self)
    }
    #[doc = "Bits 4:5 - MU1 (DSP PORT)"]
    #[inline(always)]
    #[must_use]
    pub fn mu1(&mut self) -> MU1_W<4> {
        MU1_W::new(self)
    }
    #[doc = "Bits 8:9 - Semaphore"]
    #[inline(always)]
    #[must_use]
    pub fn semaphore(&mut self) -> SEMAPHORE_W<8> {
        SEMAPHORE_W::new(self)
    }
    #[doc = "Bits 12:13 - OS_EVENT TIMER (M33 PORT)"]
    #[inline(always)]
    #[must_use]
    pub fn os_event_timer_m33_port(&mut self) -> OS_EVENT_TIMER_M33_PORT_W<12> {
        OS_EVENT_TIMER_M33_PORT_W::new(self)
    }
    #[doc = "Bits 16:17 - OS_EVENT TIMER (DSP PORT)"]
    #[inline(always)]
    #[must_use]
    pub fn os_event_timer_dsp_port(&mut self) -> OS_EVENT_TIMER_DSP_PORT_W<16> {
        OS_EVENT_TIMER_DSP_PORT_W::new(self)
    }
    #[doc = "Bits 20:21 - ROM"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<20> {
        ROM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AIPS Bridge Peripheral 0 Rule 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aips_bridge0_per_rule0](index.html) module"]
pub struct AIPS_BRIDGE0_PER_RULE0_SPEC;
impl crate::RegisterSpec for AIPS_BRIDGE0_PER_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aips_bridge0_per_rule0::R](R) reader structure"]
impl crate::Readable for AIPS_BRIDGE0_PER_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aips_bridge0_per_rule0::W](W) writer structure"]
impl crate::Writable for AIPS_BRIDGE0_PER_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIPS_BRIDGE0_PER_RULE0 to value 0"]
impl crate::Resettable for AIPS_BRIDGE0_PER_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
