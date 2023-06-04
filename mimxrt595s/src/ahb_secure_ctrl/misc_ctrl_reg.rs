#[doc = "Register `MISC_CTRL_REG` reader"]
pub struct R(crate::R<MISC_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_CTRL_REG` writer"]
pub struct W(crate::W<MISC_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_CTRL_REG_SPEC>;
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
impl From<crate::W<MISC_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE_LOCK` reader - Write Lock"]
pub type WRITE_LOCK_R = crate::FieldReader<u8, WRITE_LOCK_A>;
#[doc = "Write Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRITE_LOCK_A {
    #[doc = "1: Writes to this register and to the Memory and Peripheral RULE registers are not allowed"]
    LOCKED = 1,
    #[doc = "2: Writes to this register and to the Memory and Peripheral RULE registers are allowed"]
    NOT_LOCKED = 2,
}
impl From<WRITE_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITE_LOCK_A) -> Self {
        variant as _
    }
}
impl WRITE_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRITE_LOCK_A> {
        match self.bits {
            1 => Some(WRITE_LOCK_A::LOCKED),
            2 => Some(WRITE_LOCK_A::NOT_LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == WRITE_LOCK_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == WRITE_LOCK_A::NOT_LOCKED
    }
}
#[doc = "Field `WRITE_LOCK` writer - Write Lock"]
pub type WRITE_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_REG_SPEC, u8, WRITE_LOCK_A, 2, O>;
impl<'a, const O: u8> WRITE_LOCK_W<'a, O> {
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::LOCKED)
    }
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed"]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::NOT_LOCKED)
    }
}
#[doc = "Field `ENABLE_SECURE_CHECKING` reader - Enable Secure Checking"]
pub type ENABLE_SECURE_CHECKING_R = crate::FieldReader<u8, ENABLE_SECURE_CHECKING_A>;
#[doc = "Enable Secure Checking\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENABLE_SECURE_CHECKING_A {
    #[doc = "1: Enabled (restrictive mode)"]
    ENABLED = 1,
    #[doc = "2: Disabled"]
    DISABLED = 2,
}
impl From<ENABLE_SECURE_CHECKING_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_SECURE_CHECKING_A) -> Self {
        variant as _
    }
}
impl ENABLE_SECURE_CHECKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENABLE_SECURE_CHECKING_A> {
        match self.bits {
            1 => Some(ENABLE_SECURE_CHECKING_A::ENABLED),
            2 => Some(ENABLE_SECURE_CHECKING_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_SECURE_CHECKING_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_SECURE_CHECKING_A::DISABLED
    }
}
#[doc = "Field `ENABLE_SECURE_CHECKING` writer - Enable Secure Checking"]
pub type ENABLE_SECURE_CHECKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_REG_SPEC, u8, ENABLE_SECURE_CHECKING_A, 2, O>;
impl<'a, const O: u8> ENABLE_SECURE_CHECKING_W<'a, O> {
    #[doc = "Enabled (restrictive mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_SECURE_CHECKING_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_SECURE_CHECKING_A::DISABLED)
    }
}
#[doc = "Field `ENABLE_S_PRIV_CHECK` reader - Enable Secure Privilege Checking"]
pub type ENABLE_S_PRIV_CHECK_R = crate::FieldReader<u8, ENABLE_S_PRIV_CHECK_A>;
#[doc = "Enable Secure Privilege Checking\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENABLE_S_PRIV_CHECK_A {
    #[doc = "1: Enabled (restrictive mode)"]
    ENABLED = 1,
    #[doc = "2: Disabled"]
    DISABLED = 2,
}
impl From<ENABLE_S_PRIV_CHECK_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_S_PRIV_CHECK_A) -> Self {
        variant as _
    }
}
impl ENABLE_S_PRIV_CHECK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENABLE_S_PRIV_CHECK_A> {
        match self.bits {
            1 => Some(ENABLE_S_PRIV_CHECK_A::ENABLED),
            2 => Some(ENABLE_S_PRIV_CHECK_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_S_PRIV_CHECK_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_S_PRIV_CHECK_A::DISABLED
    }
}
#[doc = "Field `ENABLE_S_PRIV_CHECK` writer - Enable Secure Privilege Checking"]
pub type ENABLE_S_PRIV_CHECK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_REG_SPEC, u8, ENABLE_S_PRIV_CHECK_A, 2, O>;
impl<'a, const O: u8> ENABLE_S_PRIV_CHECK_W<'a, O> {
    #[doc = "Enabled (restrictive mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_S_PRIV_CHECK_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_S_PRIV_CHECK_A::DISABLED)
    }
}
#[doc = "Field `ENABLE_NS_PRIV_CHECK` reader - Enable Non-Secure Privilege Checking"]
pub type ENABLE_NS_PRIV_CHECK_R = crate::FieldReader<u8, ENABLE_NS_PRIV_CHECK_A>;
#[doc = "Enable Non-Secure Privilege Checking\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENABLE_NS_PRIV_CHECK_A {
    #[doc = "1: Enabled (restrictive mode)"]
    ENABLED = 1,
    #[doc = "2: Disabled"]
    DISABLED = 2,
}
impl From<ENABLE_NS_PRIV_CHECK_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_NS_PRIV_CHECK_A) -> Self {
        variant as _
    }
}
impl ENABLE_NS_PRIV_CHECK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENABLE_NS_PRIV_CHECK_A> {
        match self.bits {
            1 => Some(ENABLE_NS_PRIV_CHECK_A::ENABLED),
            2 => Some(ENABLE_NS_PRIV_CHECK_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_NS_PRIV_CHECK_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_NS_PRIV_CHECK_A::DISABLED
    }
}
#[doc = "Field `ENABLE_NS_PRIV_CHECK` writer - Enable Non-Secure Privilege Checking"]
pub type ENABLE_NS_PRIV_CHECK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_REG_SPEC, u8, ENABLE_NS_PRIV_CHECK_A, 2, O>;
impl<'a, const O: u8> ENABLE_NS_PRIV_CHECK_W<'a, O> {
    #[doc = "Enabled (restrictive mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_NS_PRIV_CHECK_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_NS_PRIV_CHECK_A::DISABLED)
    }
}
#[doc = "Field `DISABLE_VIOLATION_ABORT` reader - Disable Violation Abort"]
pub type DISABLE_VIOLATION_ABORT_R = crate::FieldReader<u8, DISABLE_VIOLATION_ABORT_A>;
#[doc = "Disable Violation Abort\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISABLE_VIOLATION_ABORT_A {
    #[doc = "1: The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NO_ABORT = 1,
    #[doc = "2: The violation detected by the secure checker will cause an abort."]
    ABORT = 2,
}
impl From<DISABLE_VIOLATION_ABORT_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_VIOLATION_ABORT_A) -> Self {
        variant as _
    }
}
impl DISABLE_VIOLATION_ABORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISABLE_VIOLATION_ABORT_A> {
        match self.bits {
            1 => Some(DISABLE_VIOLATION_ABORT_A::NO_ABORT),
            2 => Some(DISABLE_VIOLATION_ABORT_A::ABORT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ABORT`"]
    #[inline(always)]
    pub fn is_no_abort(&self) -> bool {
        *self == DISABLE_VIOLATION_ABORT_A::NO_ABORT
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == DISABLE_VIOLATION_ABORT_A::ABORT
    }
}
#[doc = "Field `DISABLE_VIOLATION_ABORT` writer - Disable Violation Abort"]
pub type DISABLE_VIOLATION_ABORT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_REG_SPEC, u8, DISABLE_VIOLATION_ABORT_A, 2, O>;
impl<'a, const O: u8> DISABLE_VIOLATION_ABORT_W<'a, O> {
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    #[inline(always)]
    pub fn no_abort(self) -> &'a mut W {
        self.variant(DISABLE_VIOLATION_ABORT_A::NO_ABORT)
    }
    #[doc = "The violation detected by the secure checker will cause an abort."]
    #[inline(always)]
    pub fn abort(self) -> &'a mut W {
        self.variant(DISABLE_VIOLATION_ABORT_A::ABORT)
    }
}
#[doc = "Field `DISABLE_SIMPLE_MASTER_STRICT_MODE` reader - Disable Simple Master Strict Mode"]
pub type DISABLE_SIMPLE_MASTER_STRICT_MODE_R =
    crate::FieldReader<u8, DISABLE_SIMPLE_MASTER_STRICT_MODE_A>;
#[doc = "Disable Simple Master Strict Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISABLE_SIMPLE_MASTER_STRICT_MODE_A {
    #[doc = "1: Can access memories and peripherals at the same level or below that level"]
    ACCESS_AT_SAME_LEVEL_OR_BELOW = 1,
    #[doc = "2: Can access memories and peripherals at same level only"]
    ACCESS_AT_SAME_LEVEL_ONLY = 2,
}
impl From<DISABLE_SIMPLE_MASTER_STRICT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_SIMPLE_MASTER_STRICT_MODE_A) -> Self {
        variant as _
    }
}
impl DISABLE_SIMPLE_MASTER_STRICT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISABLE_SIMPLE_MASTER_STRICT_MODE_A> {
        match self.bits {
            1 => Some(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_OR_BELOW),
            2 => Some(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_ONLY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ACCESS_AT_SAME_LEVEL_OR_BELOW`"]
    #[inline(always)]
    pub fn is_access_at_same_level_or_below(&self) -> bool {
        *self == DISABLE_SIMPLE_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_OR_BELOW
    }
    #[doc = "Checks if the value of the field is `ACCESS_AT_SAME_LEVEL_ONLY`"]
    #[inline(always)]
    pub fn is_access_at_same_level_only(&self) -> bool {
        *self == DISABLE_SIMPLE_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_ONLY
    }
}
#[doc = "Field `DISABLE_SIMPLE_MASTER_STRICT_MODE` writer - Disable Simple Master Strict Mode"]
pub type DISABLE_SIMPLE_MASTER_STRICT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_REG_SPEC, u8, DISABLE_SIMPLE_MASTER_STRICT_MODE_A, 2, O>;
impl<'a, const O: u8> DISABLE_SIMPLE_MASTER_STRICT_MODE_W<'a, O> {
    #[doc = "Can access memories and peripherals at the same level or below that level"]
    #[inline(always)]
    pub fn access_at_same_level_or_below(self) -> &'a mut W {
        self.variant(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_OR_BELOW)
    }
    #[doc = "Can access memories and peripherals at same level only"]
    #[inline(always)]
    pub fn access_at_same_level_only(self) -> &'a mut W {
        self.variant(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_ONLY)
    }
}
#[doc = "Field `DISABLE_SMART_MASTER_STRICT_MODE` reader - Disable Smart Master Strict Mode"]
pub type DISABLE_SMART_MASTER_STRICT_MODE_R =
    crate::FieldReader<u8, DISABLE_SMART_MASTER_STRICT_MODE_A>;
#[doc = "Disable Smart Master Strict Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISABLE_SMART_MASTER_STRICT_MODE_A {
    #[doc = "1: Can access memories and peripherals at the same level or below that level"]
    ACCESS_AT_SAME_LEVEL_OR_BELOW = 1,
    #[doc = "2: Can access memories and peripherals at same level only"]
    ACCESS_AT_SAME_LEVEL_ONLY = 2,
}
impl From<DISABLE_SMART_MASTER_STRICT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_SMART_MASTER_STRICT_MODE_A) -> Self {
        variant as _
    }
}
impl DISABLE_SMART_MASTER_STRICT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISABLE_SMART_MASTER_STRICT_MODE_A> {
        match self.bits {
            1 => Some(DISABLE_SMART_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_OR_BELOW),
            2 => Some(DISABLE_SMART_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_ONLY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ACCESS_AT_SAME_LEVEL_OR_BELOW`"]
    #[inline(always)]
    pub fn is_access_at_same_level_or_below(&self) -> bool {
        *self == DISABLE_SMART_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_OR_BELOW
    }
    #[doc = "Checks if the value of the field is `ACCESS_AT_SAME_LEVEL_ONLY`"]
    #[inline(always)]
    pub fn is_access_at_same_level_only(&self) -> bool {
        *self == DISABLE_SMART_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_ONLY
    }
}
#[doc = "Field `DISABLE_SMART_MASTER_STRICT_MODE` writer - Disable Smart Master Strict Mode"]
pub type DISABLE_SMART_MASTER_STRICT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_REG_SPEC, u8, DISABLE_SMART_MASTER_STRICT_MODE_A, 2, O>;
impl<'a, const O: u8> DISABLE_SMART_MASTER_STRICT_MODE_W<'a, O> {
    #[doc = "Can access memories and peripherals at the same level or below that level"]
    #[inline(always)]
    pub fn access_at_same_level_or_below(self) -> &'a mut W {
        self.variant(DISABLE_SMART_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_OR_BELOW)
    }
    #[doc = "Can access memories and peripherals at same level only"]
    #[inline(always)]
    pub fn access_at_same_level_only(self) -> &'a mut W {
        self.variant(DISABLE_SMART_MASTER_STRICT_MODE_A::ACCESS_AT_SAME_LEVEL_ONLY)
    }
}
#[doc = "Field `IDAU_ALL_NS` reader - IDAU All Non-Secure"]
pub type IDAU_ALL_NS_R = crate::FieldReader<u8, IDAU_ALL_NS_A>;
#[doc = "IDAU All Non-Secure\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDAU_ALL_NS_A {
    #[doc = "1: IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    DISABLED = 1,
    #[doc = "2: IDAU is enabled (restrictive mode)"]
    ENABLED = 2,
}
impl From<IDAU_ALL_NS_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAU_ALL_NS_A) -> Self {
        variant as _
    }
}
impl IDAU_ALL_NS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDAU_ALL_NS_A> {
        match self.bits {
            1 => Some(IDAU_ALL_NS_A::DISABLED),
            2 => Some(IDAU_ALL_NS_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDAU_ALL_NS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDAU_ALL_NS_A::ENABLED
    }
}
#[doc = "Field `IDAU_ALL_NS` writer - IDAU All Non-Secure"]
pub type IDAU_ALL_NS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_REG_SPEC, u8, IDAU_ALL_NS_A, 2, O>;
impl<'a, const O: u8> IDAU_ALL_NS_W<'a, O> {
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDAU_ALL_NS_A::DISABLED)
    }
    #[doc = "IDAU is enabled (restrictive mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDAU_ALL_NS_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:1 - Write Lock"]
    #[inline(always)]
    pub fn write_lock(&self) -> WRITE_LOCK_R {
        WRITE_LOCK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Enable Secure Checking"]
    #[inline(always)]
    pub fn enable_secure_checking(&self) -> ENABLE_SECURE_CHECKING_R {
        ENABLE_SECURE_CHECKING_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Enable Secure Privilege Checking"]
    #[inline(always)]
    pub fn enable_s_priv_check(&self) -> ENABLE_S_PRIV_CHECK_R {
        ENABLE_S_PRIV_CHECK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Enable Non-Secure Privilege Checking"]
    #[inline(always)]
    pub fn enable_ns_priv_check(&self) -> ENABLE_NS_PRIV_CHECK_R {
        ENABLE_NS_PRIV_CHECK_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Disable Violation Abort"]
    #[inline(always)]
    pub fn disable_violation_abort(&self) -> DISABLE_VIOLATION_ABORT_R {
        DISABLE_VIOLATION_ABORT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Disable Simple Master Strict Mode"]
    #[inline(always)]
    pub fn disable_simple_master_strict_mode(&self) -> DISABLE_SIMPLE_MASTER_STRICT_MODE_R {
        DISABLE_SIMPLE_MASTER_STRICT_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Disable Smart Master Strict Mode"]
    #[inline(always)]
    pub fn disable_smart_master_strict_mode(&self) -> DISABLE_SMART_MASTER_STRICT_MODE_R {
        DISABLE_SMART_MASTER_STRICT_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - IDAU All Non-Secure"]
    #[inline(always)]
    pub fn idau_all_ns(&self) -> IDAU_ALL_NS_R {
        IDAU_ALL_NS_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn write_lock(&mut self) -> WRITE_LOCK_W<0> {
        WRITE_LOCK_W::new(self)
    }
    #[doc = "Bits 2:3 - Enable Secure Checking"]
    #[inline(always)]
    #[must_use]
    pub fn enable_secure_checking(&mut self) -> ENABLE_SECURE_CHECKING_W<2> {
        ENABLE_SECURE_CHECKING_W::new(self)
    }
    #[doc = "Bits 4:5 - Enable Secure Privilege Checking"]
    #[inline(always)]
    #[must_use]
    pub fn enable_s_priv_check(&mut self) -> ENABLE_S_PRIV_CHECK_W<4> {
        ENABLE_S_PRIV_CHECK_W::new(self)
    }
    #[doc = "Bits 6:7 - Enable Non-Secure Privilege Checking"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ns_priv_check(&mut self) -> ENABLE_NS_PRIV_CHECK_W<6> {
        ENABLE_NS_PRIV_CHECK_W::new(self)
    }
    #[doc = "Bits 8:9 - Disable Violation Abort"]
    #[inline(always)]
    #[must_use]
    pub fn disable_violation_abort(&mut self) -> DISABLE_VIOLATION_ABORT_W<8> {
        DISABLE_VIOLATION_ABORT_W::new(self)
    }
    #[doc = "Bits 10:11 - Disable Simple Master Strict Mode"]
    #[inline(always)]
    #[must_use]
    pub fn disable_simple_master_strict_mode(&mut self) -> DISABLE_SIMPLE_MASTER_STRICT_MODE_W<10> {
        DISABLE_SIMPLE_MASTER_STRICT_MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Disable Smart Master Strict Mode"]
    #[inline(always)]
    #[must_use]
    pub fn disable_smart_master_strict_mode(&mut self) -> DISABLE_SMART_MASTER_STRICT_MODE_W<12> {
        DISABLE_SMART_MASTER_STRICT_MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - IDAU All Non-Secure"]
    #[inline(always)]
    #[must_use]
    pub fn idau_all_ns(&mut self) -> IDAU_ALL_NS_W<14> {
        IDAU_ALL_NS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_ctrl_reg](index.html) module"]
pub struct MISC_CTRL_REG_SPEC;
impl crate::RegisterSpec for MISC_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_ctrl_reg::R](R) reader structure"]
impl crate::Readable for MISC_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_ctrl_reg::W](W) writer structure"]
impl crate::Writable for MISC_CTRL_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC_CTRL_REG to value 0xaaaa"]
impl crate::Resettable for MISC_CTRL_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0xaaaa;
}
