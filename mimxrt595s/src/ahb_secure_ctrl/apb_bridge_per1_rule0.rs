#[doc = "Register `APB_BRIDGE_PER1_RULE0` reader"]
pub struct R(crate::R<APB_BRIDGE_PER1_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_BRIDGE_PER1_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_BRIDGE_PER1_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_BRIDGE_PER1_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_BRIDGE_PER1_RULE0` writer"]
pub struct W(crate::W<APB_BRIDGE_PER1_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_BRIDGE_PER1_RULE0_SPEC>;
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
impl From<crate::W<APB_BRIDGE_PER1_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_BRIDGE_PER1_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTCTL_B` reader - RSTCTL_B"]
pub type RSTCTL_B_R = crate::FieldReader<u8, RSTCTL_B_A>;
#[doc = "RSTCTL_B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTCTL_B_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<RSTCTL_B_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTCTL_B_A) -> Self {
        variant as _
    }
}
impl RSTCTL_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTCTL_B_A {
        match self.bits {
            0 => RSTCTL_B_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => RSTCTL_B_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => RSTCTL_B_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => RSTCTL_B_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == RSTCTL_B_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == RSTCTL_B_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == RSTCTL_B_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == RSTCTL_B_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `RSTCTL_B` writer - RSTCTL_B"]
pub type RSTCTL_B_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE0_SPEC, u8, RSTCTL_B_A, 2, O>;
impl<'a, const O: u8> RSTCTL_B_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RSTCTL_B_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RSTCTL_B_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RSTCTL_B_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RSTCTL_B_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `CLKCTL_B` reader - CLKCTL_B"]
pub type CLKCTL_B_R = crate::FieldReader<u8, CLKCTL_B_A>;
#[doc = "CLKCTL_B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKCTL_B_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<CLKCTL_B_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKCTL_B_A) -> Self {
        variant as _
    }
}
impl CLKCTL_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKCTL_B_A {
        match self.bits {
            0 => CLKCTL_B_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => CLKCTL_B_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => CLKCTL_B_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => CLKCTL_B_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == CLKCTL_B_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == CLKCTL_B_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == CLKCTL_B_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == CLKCTL_B_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `CLKCTL_B` writer - CLKCTL_B"]
pub type CLKCTL_B_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE0_SPEC, u8, CLKCTL_B_A, 2, O>;
impl<'a, const O: u8> CLKCTL_B_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CLKCTL_B_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CLKCTL_B_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(CLKCTL_B_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(CLKCTL_B_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `SYSCTL_B` reader - SYSCTL_B"]
pub type SYSCTL_B_R = crate::FieldReader<u8, SYSCTL_B_A>;
#[doc = "SYSCTL_B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_B_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<SYSCTL_B_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_B_A) -> Self {
        variant as _
    }
}
impl SYSCTL_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_B_A {
        match self.bits {
            0 => SYSCTL_B_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => SYSCTL_B_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => SYSCTL_B_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => SYSCTL_B_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == SYSCTL_B_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == SYSCTL_B_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == SYSCTL_B_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == SYSCTL_B_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `SYSCTL_B` writer - SYSCTL_B"]
pub type SYSCTL_B_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE0_SPEC, u8, SYSCTL_B_A, 2, O>;
impl<'a, const O: u8> SYSCTL_B_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SYSCTL_B_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SYSCTL_B_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SYSCTL_B_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SYSCTL_B_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `GPIO_INT` reader - GPIO_INT"]
pub type GPIO_INT_R = crate::FieldReader<u8, GPIO_INT_A>;
#[doc = "GPIO_INT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO_INT_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<GPIO_INT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO_INT_A) -> Self {
        variant as _
    }
}
impl GPIO_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT_A {
        match self.bits {
            0 => GPIO_INT_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => GPIO_INT_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => GPIO_INT_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => GPIO_INT_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == GPIO_INT_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == GPIO_INT_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == GPIO_INT_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == GPIO_INT_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `GPIO_INT` writer - GPIO_INT"]
pub type GPIO_INT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE0_SPEC, u8, GPIO_INT_A, 2, O>;
impl<'a, const O: u8> GPIO_INT_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(GPIO_INT_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(GPIO_INT_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(GPIO_INT_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(GPIO_INT_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `PERIPHERAL_MUXES` reader - Peripheral Muxes"]
pub type PERIPHERAL_MUXES_R = crate::FieldReader<u8, PERIPHERAL_MUXES_A>;
#[doc = "Peripheral Muxes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERIPHERAL_MUXES_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<PERIPHERAL_MUXES_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIPHERAL_MUXES_A) -> Self {
        variant as _
    }
}
impl PERIPHERAL_MUXES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPHERAL_MUXES_A {
        match self.bits {
            0 => PERIPHERAL_MUXES_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => PERIPHERAL_MUXES_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => PERIPHERAL_MUXES_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => PERIPHERAL_MUXES_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == PERIPHERAL_MUXES_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == PERIPHERAL_MUXES_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == PERIPHERAL_MUXES_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == PERIPHERAL_MUXES_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `PERIPHERAL_MUXES` writer - Peripheral Muxes"]
pub type PERIPHERAL_MUXES_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE0_SPEC, u8, PERIPHERAL_MUXES_A, 2, O>;
impl<'a, const O: u8> PERIPHERAL_MUXES_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(PERIPHERAL_MUXES_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(PERIPHERAL_MUXES_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(PERIPHERAL_MUXES_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(PERIPHERAL_MUXES_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `SDMA` reader - Smart DMA (SDMA)"]
pub type SDMA_R = crate::FieldReader<u8, SDMA_A>;
#[doc = "Smart DMA (SDMA)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDMA_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<SDMA_A> for u8 {
    #[inline(always)]
    fn from(variant: SDMA_A) -> Self {
        variant as _
    }
}
impl SDMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA_A {
        match self.bits {
            0 => SDMA_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => SDMA_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => SDMA_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => SDMA_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == SDMA_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == SDMA_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == SDMA_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == SDMA_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `SDMA` writer - Smart DMA (SDMA)"]
pub type SDMA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, APB_BRIDGE_PER1_RULE0_SPEC, u8, SDMA_A, 2, O>;
impl<'a, const O: u8> SDMA_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SDMA_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SDMA_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SDMA_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SDMA_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - RSTCTL_B"]
    #[inline(always)]
    pub fn rstctl_b(&self) -> RSTCTL_B_R {
        RSTCTL_B_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - CLKCTL_B"]
    #[inline(always)]
    pub fn clkctl_b(&self) -> CLKCTL_B_R {
        CLKCTL_B_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SYSCTL_B"]
    #[inline(always)]
    pub fn sysctl_b(&self) -> SYSCTL_B_R {
        SYSCTL_B_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIO_INT"]
    #[inline(always)]
    pub fn gpio_int(&self) -> GPIO_INT_R {
        GPIO_INT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Peripheral Muxes"]
    #[inline(always)]
    pub fn peripheral_muxes(&self) -> PERIPHERAL_MUXES_R {
        PERIPHERAL_MUXES_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Smart DMA (SDMA)"]
    #[inline(always)]
    pub fn sdma(&self) -> SDMA_R {
        SDMA_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RSTCTL_B"]
    #[inline(always)]
    #[must_use]
    pub fn rstctl_b(&mut self) -> RSTCTL_B_W<0> {
        RSTCTL_B_W::new(self)
    }
    #[doc = "Bits 4:5 - CLKCTL_B"]
    #[inline(always)]
    #[must_use]
    pub fn clkctl_b(&mut self) -> CLKCTL_B_W<4> {
        CLKCTL_B_W::new(self)
    }
    #[doc = "Bits 8:9 - SYSCTL_B"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_b(&mut self) -> SYSCTL_B_W<8> {
        SYSCTL_B_W::new(self)
    }
    #[doc = "Bits 20:21 - GPIO_INT"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int(&mut self) -> GPIO_INT_W<20> {
        GPIO_INT_W::new(self)
    }
    #[doc = "Bits 24:25 - Peripheral Muxes"]
    #[inline(always)]
    #[must_use]
    pub fn peripheral_muxes(&mut self) -> PERIPHERAL_MUXES_W<24> {
        PERIPHERAL_MUXES_W::new(self)
    }
    #[doc = "Bits 28:29 - Smart DMA (SDMA)"]
    #[inline(always)]
    #[must_use]
    pub fn sdma(&mut self) -> SDMA_W<28> {
        SDMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Bridge Peripheral 1 Rule 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_bridge_per1_rule0](index.html) module"]
pub struct APB_BRIDGE_PER1_RULE0_SPEC;
impl crate::RegisterSpec for APB_BRIDGE_PER1_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_bridge_per1_rule0::R](R) reader structure"]
impl crate::Readable for APB_BRIDGE_PER1_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_bridge_per1_rule0::W](W) writer structure"]
impl crate::Writable for APB_BRIDGE_PER1_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_BRIDGE_PER1_RULE0 to value 0"]
impl crate::Resettable for APB_BRIDGE_PER1_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
