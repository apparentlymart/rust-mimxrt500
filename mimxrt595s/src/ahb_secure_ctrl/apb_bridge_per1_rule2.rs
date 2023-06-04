#[doc = "Register `APB_BRIDGE_PER1_RULE2` reader"]
pub struct R(crate::R<APB_BRIDGE_PER1_RULE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_BRIDGE_PER1_RULE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_BRIDGE_PER1_RULE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_BRIDGE_PER1_RULE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_BRIDGE_PER1_RULE2` writer"]
pub struct W(crate::W<APB_BRIDGE_PER1_RULE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_BRIDGE_PER1_RULE2_SPEC>;
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
impl From<crate::W<APB_BRIDGE_PER1_RULE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_BRIDGE_PER1_RULE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_WAKEUP` reader - RTC Wakeup"]
pub type RTC_WAKEUP_R = crate::FieldReader<u8, RTC_WAKEUP_A>;
#[doc = "RTC Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTC_WAKEUP_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<RTC_WAKEUP_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_WAKEUP_A) -> Self {
        variant as _
    }
}
impl RTC_WAKEUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_WAKEUP_A {
        match self.bits {
            0 => RTC_WAKEUP_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => RTC_WAKEUP_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => RTC_WAKEUP_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => RTC_WAKEUP_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == RTC_WAKEUP_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == RTC_WAKEUP_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == RTC_WAKEUP_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == RTC_WAKEUP_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `RTC_WAKEUP` writer - RTC Wakeup"]
pub type RTC_WAKEUP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE2_SPEC, u8, RTC_WAKEUP_A, 2, O>;
impl<'a, const O: u8> RTC_WAKEUP_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RTC_WAKEUP_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RTC_WAKEUP_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RTC_WAKEUP_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RTC_WAKEUP_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `DSI_HOST_CONTROLLER` reader - DSI Host Controller"]
pub type DSI_HOST_CONTROLLER_R = crate::FieldReader<u8, DSI_HOST_CONTROLLER_A>;
#[doc = "DSI Host Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSI_HOST_CONTROLLER_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<DSI_HOST_CONTROLLER_A> for u8 {
    #[inline(always)]
    fn from(variant: DSI_HOST_CONTROLLER_A) -> Self {
        variant as _
    }
}
impl DSI_HOST_CONTROLLER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_HOST_CONTROLLER_A {
        match self.bits {
            0 => DSI_HOST_CONTROLLER_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => DSI_HOST_CONTROLLER_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => DSI_HOST_CONTROLLER_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => DSI_HOST_CONTROLLER_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == DSI_HOST_CONTROLLER_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == DSI_HOST_CONTROLLER_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == DSI_HOST_CONTROLLER_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == DSI_HOST_CONTROLLER_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `DSI_HOST_CONTROLLER` writer - DSI Host Controller"]
pub type DSI_HOST_CONTROLLER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE2_SPEC, u8, DSI_HOST_CONTROLLER_A, 2, O>;
impl<'a, const O: u8> DSI_HOST_CONTROLLER_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DSI_HOST_CONTROLLER_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DSI_HOST_CONTROLLER_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(DSI_HOST_CONTROLLER_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(DSI_HOST_CONTROLLER_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXIO_REGISTERS` reader - FLEXIO Registers"]
pub type FLEXIO_REGISTERS_R = crate::FieldReader<u8, FLEXIO_REGISTERS_A>;
#[doc = "FLEXIO Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXIO_REGISTERS_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXIO_REGISTERS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXIO_REGISTERS_A) -> Self {
        variant as _
    }
}
impl FLEXIO_REGISTERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO_REGISTERS_A {
        match self.bits {
            0 => FLEXIO_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXIO_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXIO_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXIO_REGISTERS_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXIO_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXIO_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXIO_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXIO_REGISTERS_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXIO_REGISTERS` writer - FLEXIO Registers"]
pub type FLEXIO_REGISTERS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE2_SPEC, u8, FLEXIO_REGISTERS_A, 2, O>;
impl<'a, const O: u8> FLEXIO_REGISTERS_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXIO_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXIO_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXIO_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXIO_REGISTERS_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CACHE_CONTROL_0_REGS` reader - Cache Control 0 Registers"]
pub type CACHE_CONTROL_0_REGS_R = crate::FieldReader<u8, CACHE_CONTROL_0_REGS_A>;
#[doc = "Cache Control 0 Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CACHE_CONTROL_0_REGS_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CACHE_CONTROL_0_REGS_A> for u8 {
    #[inline(always)]
    fn from(variant: CACHE_CONTROL_0_REGS_A) -> Self {
        variant as _
    }
}
impl CACHE_CONTROL_0_REGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_CONTROL_0_REGS_A {
        match self.bits {
            0 => CACHE_CONTROL_0_REGS_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CACHE_CONTROL_0_REGS_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CACHE_CONTROL_0_REGS_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CACHE_CONTROL_0_REGS_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CACHE_CONTROL_0_REGS_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CACHE_CONTROL_0_REGS_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CACHE_CONTROL_0_REGS_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CACHE_CONTROL_0_REGS_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CACHE_CONTROL_0_REGS` writer - Cache Control 0 Registers"]
pub type CACHE_CONTROL_0_REGS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE2_SPEC, u8, CACHE_CONTROL_0_REGS_A, 2, O>;
impl<'a, const O: u8> CACHE_CONTROL_0_REGS_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CACHE_CONTROL_0_REGS_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CACHE_CONTROL_0_REGS_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CACHE_CONTROL_0_REGS_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CACHE_CONTROL_0_REGS_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CACHE_CONTROL_1_REGS` reader - Cache Control 1 Registers"]
pub type CACHE_CONTROL_1_REGS_R = crate::FieldReader<u8, CACHE_CONTROL_1_REGS_A>;
#[doc = "Cache Control 1 Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CACHE_CONTROL_1_REGS_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CACHE_CONTROL_1_REGS_A> for u8 {
    #[inline(always)]
    fn from(variant: CACHE_CONTROL_1_REGS_A) -> Self {
        variant as _
    }
}
impl CACHE_CONTROL_1_REGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_CONTROL_1_REGS_A {
        match self.bits {
            0 => CACHE_CONTROL_1_REGS_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CACHE_CONTROL_1_REGS_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CACHE_CONTROL_1_REGS_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CACHE_CONTROL_1_REGS_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CACHE_CONTROL_1_REGS_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CACHE_CONTROL_1_REGS_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CACHE_CONTROL_1_REGS_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CACHE_CONTROL_1_REGS_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CACHE_CONTROL_1_REGS` writer - Cache Control 1 Registers"]
pub type CACHE_CONTROL_1_REGS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE2_SPEC, u8, CACHE_CONTROL_1_REGS_A, 2, O>;
impl<'a, const O: u8> CACHE_CONTROL_1_REGS_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CACHE_CONTROL_1_REGS_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CACHE_CONTROL_1_REGS_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CACHE_CONTROL_1_REGS_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CACHE_CONTROL_1_REGS_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `I3C0` reader - I3C0"]
pub type I3C0_R = crate::FieldReader<u8, I3C0_A>;
#[doc = "I3C0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I3C0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<I3C0_A> for u8 {
    #[inline(always)]
    fn from(variant: I3C0_A) -> Self {
        variant as _
    }
}
impl I3C0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C0_A {
        match self.bits {
            0 => I3C0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => I3C0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => I3C0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => I3C0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == I3C0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == I3C0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == I3C0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == I3C0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `I3C0` writer - I3C0"]
pub type I3C0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE2_SPEC, u8, I3C0_A, 2, O>;
impl<'a, const O: u8> I3C0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(I3C0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(I3C0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(I3C0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(I3C0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `I3C1` reader - I3C1"]
pub type I3C1_R = crate::FieldReader<u8, I3C1_A>;
#[doc = "I3C1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I3C1_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<I3C1_A> for u8 {
    #[inline(always)]
    fn from(variant: I3C1_A) -> Self {
        variant as _
    }
}
impl I3C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C1_A {
        match self.bits {
            0 => I3C1_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => I3C1_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => I3C1_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => I3C1_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == I3C1_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == I3C1_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == I3C1_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == I3C1_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `I3C1` writer - I3C1"]
pub type I3C1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE2_SPEC, u8, I3C1_A, 2, O>;
impl<'a, const O: u8> I3C1_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(I3C1_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(I3C1_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(I3C1_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(I3C1_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - RTC Wakeup"]
    #[inline(always)]
    pub fn rtc_wakeup(&self) -> RTC_WAKEUP_R {
        RTC_WAKEUP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - DSI Host Controller"]
    #[inline(always)]
    pub fn dsi_host_controller(&self) -> DSI_HOST_CONTROLLER_R {
        DSI_HOST_CONTROLLER_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - FLEXIO Registers"]
    #[inline(always)]
    pub fn flexio_registers(&self) -> FLEXIO_REGISTERS_R {
        FLEXIO_REGISTERS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Cache Control 0 Registers"]
    #[inline(always)]
    pub fn cache_control_0_regs(&self) -> CACHE_CONTROL_0_REGS_R {
        CACHE_CONTROL_0_REGS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Cache Control 1 Registers"]
    #[inline(always)]
    pub fn cache_control_1_regs(&self) -> CACHE_CONTROL_1_REGS_R {
        CACHE_CONTROL_1_REGS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - I3C0"]
    #[inline(always)]
    pub fn i3c0(&self) -> I3C0_R {
        I3C0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - I3C1"]
    #[inline(always)]
    pub fn i3c1(&self) -> I3C1_R {
        I3C1_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTC Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_wakeup(&mut self) -> RTC_WAKEUP_W<0> {
        RTC_WAKEUP_W::new(self)
    }
    #[doc = "Bits 4:5 - DSI Host Controller"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_host_controller(&mut self) -> DSI_HOST_CONTROLLER_W<4> {
        DSI_HOST_CONTROLLER_W::new(self)
    }
    #[doc = "Bits 8:9 - FLEXIO Registers"]
    #[inline(always)]
    #[must_use]
    pub fn flexio_registers(&mut self) -> FLEXIO_REGISTERS_W<8> {
        FLEXIO_REGISTERS_W::new(self)
    }
    #[doc = "Bits 12:13 - Cache Control 0 Registers"]
    #[inline(always)]
    #[must_use]
    pub fn cache_control_0_regs(&mut self) -> CACHE_CONTROL_0_REGS_W<12> {
        CACHE_CONTROL_0_REGS_W::new(self)
    }
    #[doc = "Bits 16:17 - Cache Control 1 Registers"]
    #[inline(always)]
    #[must_use]
    pub fn cache_control_1_regs(&mut self) -> CACHE_CONTROL_1_REGS_W<16> {
        CACHE_CONTROL_1_REGS_W::new(self)
    }
    #[doc = "Bits 24:25 - I3C0"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0(&mut self) -> I3C0_W<24> {
        I3C0_W::new(self)
    }
    #[doc = "Bits 28:29 - I3C1"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1(&mut self) -> I3C1_W<28> {
        I3C1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Bridge Peripheral 1 Rule 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_bridge_per1_rule2](index.html) module"]
pub struct APB_BRIDGE_PER1_RULE2_SPEC;
impl crate::RegisterSpec for APB_BRIDGE_PER1_RULE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_bridge_per1_rule2::R](R) reader structure"]
impl crate::Readable for APB_BRIDGE_PER1_RULE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_bridge_per1_rule2::W](W) writer structure"]
impl crate::Writable for APB_BRIDGE_PER1_RULE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_BRIDGE_PER1_RULE2 to value 0"]
impl crate::Resettable for APB_BRIDGE_PER1_RULE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
