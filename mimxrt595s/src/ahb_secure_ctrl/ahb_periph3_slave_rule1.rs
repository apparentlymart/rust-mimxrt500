#[doc = "Register `AHB_PERIPH3_SLAVE_RULE1` reader"]
pub struct R(crate::R<AHB_PERIPH3_SLAVE_RULE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_PERIPH3_SLAVE_RULE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_PERIPH3_SLAVE_RULE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_PERIPH3_SLAVE_RULE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_PERIPH3_SLAVE_RULE1` writer"]
pub struct W(crate::W<AHB_PERIPH3_SLAVE_RULE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_PERIPH3_SLAVE_RULE1_SPEC>;
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
impl From<crate::W<AHB_PERIPH3_SLAVE_RULE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_PERIPH3_SLAVE_RULE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXCOMM11` reader - FLEXCOMM 11"]
pub type FLEXCOMM11_R = crate::FieldReader<u8, FLEXCOMM11_A>;
#[doc = "FLEXCOMM 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM11_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM11_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM11_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM11_A {
        match self.bits {
            0 => FLEXCOMM11_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM11_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM11_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM11_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM11_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM11_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM11_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM11_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM11` writer - FLEXCOMM 11"]
pub type FLEXCOMM11_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE1_SPEC, u8, FLEXCOMM11_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM11_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM11_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM11_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM11_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM11_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM12` reader - FLEXCOMM 12"]
pub type FLEXCOMM12_R = crate::FieldReader<u8, FLEXCOMM12_A>;
#[doc = "FLEXCOMM 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM12_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM12_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM12_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM12_A {
        match self.bits {
            0 => FLEXCOMM12_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM12_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM12_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM12_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM12_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM12_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM12_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM12_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM12` writer - FLEXCOMM 12"]
pub type FLEXCOMM12_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE1_SPEC, u8, FLEXCOMM12_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM12_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM12_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM12_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM12_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM12_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXCOMM13` reader - FLEXCOMM 13"]
pub type FLEXCOMM13_R = crate::FieldReader<u8, FLEXCOMM13_A>;
#[doc = "FLEXCOMM 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM13_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM13_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM13_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM13_A {
        match self.bits {
            0 => FLEXCOMM13_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM13_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM13_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM13_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM13_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM13_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM13_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM13_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM13` writer - FLEXCOMM 13"]
pub type FLEXCOMM13_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE1_SPEC, u8, FLEXCOMM13_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM13_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM13_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM13_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM13_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM13_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `LCDIF` reader - LCDIF"]
pub type LCDIF_R = crate::FieldReader<u8, LCDIF_A>;
#[doc = "LCDIF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDIF_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<LCDIF_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDIF_A) -> Self {
        variant as _
    }
}
impl LCDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDIF_A {
        match self.bits {
            0 => LCDIF_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => LCDIF_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => LCDIF_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => LCDIF_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == LCDIF_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == LCDIF_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == LCDIF_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == LCDIF_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `LCDIF` writer - LCDIF"]
pub type LCDIF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE1_SPEC, u8, LCDIF_A, 2, O>;
impl<'a, const O: u8> LCDIF_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(LCDIF_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(LCDIF_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(LCDIF_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(LCDIF_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `GPU` reader - GPU"]
pub type GPU_R = crate::FieldReader<u8, GPU_A>;
#[doc = "GPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPU_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<GPU_A> for u8 {
    #[inline(always)]
    fn from(variant: GPU_A) -> Self {
        variant as _
    }
}
impl GPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPU_A {
        match self.bits {
            0 => GPU_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => GPU_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => GPU_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => GPU_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == GPU_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == GPU_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == GPU_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == GPU_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `GPU` writer - GPU"]
pub type GPU_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE1_SPEC, u8, GPU_A, 2, O>;
impl<'a, const O: u8> GPU_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(GPU_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(GPU_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(GPU_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(GPU_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `AXI_SWITCH` reader - AXI Switch"]
pub type AXI_SWITCH_R = crate::FieldReader<u8, AXI_SWITCH_A>;
#[doc = "AXI Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AXI_SWITCH_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<AXI_SWITCH_A> for u8 {
    #[inline(always)]
    fn from(variant: AXI_SWITCH_A) -> Self {
        variant as _
    }
}
impl AXI_SWITCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXI_SWITCH_A {
        match self.bits {
            0 => AXI_SWITCH_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => AXI_SWITCH_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => AXI_SWITCH_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => AXI_SWITCH_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == AXI_SWITCH_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == AXI_SWITCH_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == AXI_SWITCH_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == AXI_SWITCH_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `AXI_SWITCH` writer - AXI Switch"]
pub type AXI_SWITCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH3_SLAVE_RULE1_SPEC, u8, AXI_SWITCH_A, 2, O>;
impl<'a, const O: u8> AXI_SWITCH_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(AXI_SWITCH_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(AXI_SWITCH_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(AXI_SWITCH_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(AXI_SWITCH_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - FLEXCOMM 11"]
    #[inline(always)]
    pub fn flexcomm11(&self) -> FLEXCOMM11_R {
        FLEXCOMM11_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - FLEXCOMM 12"]
    #[inline(always)]
    pub fn flexcomm12(&self) -> FLEXCOMM12_R {
        FLEXCOMM12_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - FLEXCOMM 13"]
    #[inline(always)]
    pub fn flexcomm13(&self) -> FLEXCOMM13_R {
        FLEXCOMM13_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - LCDIF"]
    #[inline(always)]
    pub fn lcdif(&self) -> LCDIF_R {
        LCDIF_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPU"]
    #[inline(always)]
    pub fn gpu(&self) -> GPU_R {
        GPU_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - AXI Switch"]
    #[inline(always)]
    pub fn axi_switch(&self) -> AXI_SWITCH_R {
        AXI_SWITCH_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FLEXCOMM 11"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm11(&mut self) -> FLEXCOMM11_W<0> {
        FLEXCOMM11_W::new(self)
    }
    #[doc = "Bits 4:5 - FLEXCOMM 12"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm12(&mut self) -> FLEXCOMM12_W<4> {
        FLEXCOMM12_W::new(self)
    }
    #[doc = "Bits 8:9 - FLEXCOMM 13"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm13(&mut self) -> FLEXCOMM13_W<8> {
        FLEXCOMM13_W::new(self)
    }
    #[doc = "Bits 12:13 - LCDIF"]
    #[inline(always)]
    #[must_use]
    pub fn lcdif(&mut self) -> LCDIF_W<12> {
        LCDIF_W::new(self)
    }
    #[doc = "Bits 16:17 - GPU"]
    #[inline(always)]
    #[must_use]
    pub fn gpu(&mut self) -> GPU_W<16> {
        GPU_W::new(self)
    }
    #[doc = "Bits 20:21 - AXI Switch"]
    #[inline(always)]
    #[must_use]
    pub fn axi_switch(&mut self) -> AXI_SWITCH_W<20> {
        AXI_SWITCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Peripheral 3 Slave Rule 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_periph3_slave_rule1](index.html) module"]
pub struct AHB_PERIPH3_SLAVE_RULE1_SPEC;
impl crate::RegisterSpec for AHB_PERIPH3_SLAVE_RULE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_periph3_slave_rule1::R](R) reader structure"]
impl crate::Readable for AHB_PERIPH3_SLAVE_RULE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_periph3_slave_rule1::W](W) writer structure"]
impl crate::Writable for AHB_PERIPH3_SLAVE_RULE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH3_SLAVE_RULE1 to value 0"]
impl crate::Resettable for AHB_PERIPH3_SLAVE_RULE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
