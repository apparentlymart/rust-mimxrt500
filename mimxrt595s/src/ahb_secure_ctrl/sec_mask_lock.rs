#[doc = "Register `SEC_MASK_LOCK` reader"]
pub struct R(crate::R<SEC_MASK_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_MASK_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_MASK_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_MASK_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_MASK_LOCK` writer"]
pub struct W(crate::W<SEC_MASK_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_MASK_LOCK_SPEC>;
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
impl From<crate::W<SEC_MASK_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_MASK_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC_GPIO_MASK0_LOCK` reader - Secure GPIO _MASK0 Lock"]
pub type SEC_GPIO_MASK0_LOCK_R = crate::FieldReader<u8, SEC_GPIO_MASK0_LOCK_A>;
#[doc = "Secure GPIO _MASK0 Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK0_LOCK_A {
    #[doc = "1: SEC_GPIO_MASK0 cannot be written"]
    CANNOT_BE_WRITTEN = 1,
    #[doc = "2: SEC_GPIO_MASK0 can be written"]
    CAN_BE_WRITTEN = 2,
}
impl From<SEC_GPIO_MASK0_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK0_LOCK_A) -> Self {
        variant as _
    }
}
impl SEC_GPIO_MASK0_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK0_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK0_LOCK_A::CANNOT_BE_WRITTEN),
            2 => Some(SEC_GPIO_MASK0_LOCK_A::CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_cannot_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK0_LOCK_A::CANNOT_BE_WRITTEN
    }
    #[doc = "Checks if the value of the field is `CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_can_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK0_LOCK_A::CAN_BE_WRITTEN
    }
}
#[doc = "Field `SEC_GPIO_MASK0_LOCK` writer - Secure GPIO _MASK0 Lock"]
pub type SEC_GPIO_MASK0_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEC_MASK_LOCK_SPEC, u8, SEC_GPIO_MASK0_LOCK_A, 2, O>;
impl<'a, const O: u8> SEC_GPIO_MASK0_LOCK_W<'a, O> {
    #[doc = "SEC_GPIO_MASK0 cannot be written"]
    #[inline(always)]
    pub fn cannot_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK0_LOCK_A::CANNOT_BE_WRITTEN)
    }
    #[doc = "SEC_GPIO_MASK0 can be written"]
    #[inline(always)]
    pub fn can_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK0_LOCK_A::CAN_BE_WRITTEN)
    }
}
#[doc = "Field `SEC_GPIO_MASK1_LOCK` reader - Secure GPIO _MASK1 Lock"]
pub type SEC_GPIO_MASK1_LOCK_R = crate::FieldReader<u8, SEC_GPIO_MASK1_LOCK_A>;
#[doc = "Secure GPIO _MASK1 Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK1_LOCK_A {
    #[doc = "1: SEC_GPIO_MASK1 cannot be written"]
    CANNOT_BE_WRITTEN = 1,
    #[doc = "2: SEC_GPIO_MASK1 can be written"]
    CAN_BE_WRITTEN = 2,
}
impl From<SEC_GPIO_MASK1_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK1_LOCK_A) -> Self {
        variant as _
    }
}
impl SEC_GPIO_MASK1_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK1_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK1_LOCK_A::CANNOT_BE_WRITTEN),
            2 => Some(SEC_GPIO_MASK1_LOCK_A::CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_cannot_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK1_LOCK_A::CANNOT_BE_WRITTEN
    }
    #[doc = "Checks if the value of the field is `CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_can_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK1_LOCK_A::CAN_BE_WRITTEN
    }
}
#[doc = "Field `SEC_GPIO_MASK1_LOCK` writer - Secure GPIO _MASK1 Lock"]
pub type SEC_GPIO_MASK1_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEC_MASK_LOCK_SPEC, u8, SEC_GPIO_MASK1_LOCK_A, 2, O>;
impl<'a, const O: u8> SEC_GPIO_MASK1_LOCK_W<'a, O> {
    #[doc = "SEC_GPIO_MASK1 cannot be written"]
    #[inline(always)]
    pub fn cannot_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK1_LOCK_A::CANNOT_BE_WRITTEN)
    }
    #[doc = "SEC_GPIO_MASK1 can be written"]
    #[inline(always)]
    pub fn can_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK1_LOCK_A::CAN_BE_WRITTEN)
    }
}
#[doc = "Field `SEC_GPIO_MASK2_LOCK` reader - Secure GPIO _MASK2 Lock"]
pub type SEC_GPIO_MASK2_LOCK_R = crate::FieldReader<u8, SEC_GPIO_MASK2_LOCK_A>;
#[doc = "Secure GPIO _MASK2 Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK2_LOCK_A {
    #[doc = "1: SEC_GPIO_MASK2 cannot be written"]
    CANNOT_BE_WRITTEN = 1,
    #[doc = "2: SEC_GPIO_MASK2 can be written"]
    CAN_BE_WRITTEN = 2,
}
impl From<SEC_GPIO_MASK2_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK2_LOCK_A) -> Self {
        variant as _
    }
}
impl SEC_GPIO_MASK2_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK2_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK2_LOCK_A::CANNOT_BE_WRITTEN),
            2 => Some(SEC_GPIO_MASK2_LOCK_A::CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_cannot_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK2_LOCK_A::CANNOT_BE_WRITTEN
    }
    #[doc = "Checks if the value of the field is `CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_can_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK2_LOCK_A::CAN_BE_WRITTEN
    }
}
#[doc = "Field `SEC_GPIO_MASK2_LOCK` writer - Secure GPIO _MASK2 Lock"]
pub type SEC_GPIO_MASK2_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEC_MASK_LOCK_SPEC, u8, SEC_GPIO_MASK2_LOCK_A, 2, O>;
impl<'a, const O: u8> SEC_GPIO_MASK2_LOCK_W<'a, O> {
    #[doc = "SEC_GPIO_MASK2 cannot be written"]
    #[inline(always)]
    pub fn cannot_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK2_LOCK_A::CANNOT_BE_WRITTEN)
    }
    #[doc = "SEC_GPIO_MASK2 can be written"]
    #[inline(always)]
    pub fn can_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK2_LOCK_A::CAN_BE_WRITTEN)
    }
}
#[doc = "Field `SEC_GPIO_MASK3_LOCK` reader - Secure GPIO _MASK3 Lock"]
pub type SEC_GPIO_MASK3_LOCK_R = crate::FieldReader<u8, SEC_GPIO_MASK3_LOCK_A>;
#[doc = "Secure GPIO _MASK3 Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK3_LOCK_A {
    #[doc = "1: SEC_GPIO_MASK3 cannot be written"]
    SCANNOT_BE_WRITTEN = 1,
    #[doc = "2: SEC_GPIO_MASK3 can be written"]
    CAN_BE_WRITTEN = 2,
}
impl From<SEC_GPIO_MASK3_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK3_LOCK_A) -> Self {
        variant as _
    }
}
impl SEC_GPIO_MASK3_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK3_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK3_LOCK_A::SCANNOT_BE_WRITTEN),
            2 => Some(SEC_GPIO_MASK3_LOCK_A::CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCANNOT_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_scannot_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK3_LOCK_A::SCANNOT_BE_WRITTEN
    }
    #[doc = "Checks if the value of the field is `CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_can_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK3_LOCK_A::CAN_BE_WRITTEN
    }
}
#[doc = "Field `SEC_GPIO_MASK3_LOCK` writer - Secure GPIO _MASK3 Lock"]
pub type SEC_GPIO_MASK3_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEC_MASK_LOCK_SPEC, u8, SEC_GPIO_MASK3_LOCK_A, 2, O>;
impl<'a, const O: u8> SEC_GPIO_MASK3_LOCK_W<'a, O> {
    #[doc = "SEC_GPIO_MASK3 cannot be written"]
    #[inline(always)]
    pub fn scannot_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK3_LOCK_A::SCANNOT_BE_WRITTEN)
    }
    #[doc = "SEC_GPIO_MASK3 can be written"]
    #[inline(always)]
    pub fn can_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK3_LOCK_A::CAN_BE_WRITTEN)
    }
}
#[doc = "Field `SEC_GPIO_MASK4_LOCK` reader - SEC_GPIO_MASK4 Lock"]
pub type SEC_GPIO_MASK4_LOCK_R = crate::FieldReader<u8, SEC_GPIO_MASK4_LOCK_A>;
#[doc = "SEC_GPIO_MASK4 Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK4_LOCK_A {
    #[doc = "1: SEC_GPIO_MASK4_LOCK cannot be written"]
    CANNOT_BE_WRITTEN = 1,
    #[doc = "2: SEC_GPIO_MASK4_LOCK can be written"]
    CAN_BE_WRITTEN = 2,
}
impl From<SEC_GPIO_MASK4_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK4_LOCK_A) -> Self {
        variant as _
    }
}
impl SEC_GPIO_MASK4_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK4_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK4_LOCK_A::CANNOT_BE_WRITTEN),
            2 => Some(SEC_GPIO_MASK4_LOCK_A::CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_cannot_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK4_LOCK_A::CANNOT_BE_WRITTEN
    }
    #[doc = "Checks if the value of the field is `CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_can_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK4_LOCK_A::CAN_BE_WRITTEN
    }
}
#[doc = "Field `SEC_GPIO_MASK4_LOCK` writer - SEC_GPIO_MASK4 Lock"]
pub type SEC_GPIO_MASK4_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEC_MASK_LOCK_SPEC, u8, SEC_GPIO_MASK4_LOCK_A, 2, O>;
impl<'a, const O: u8> SEC_GPIO_MASK4_LOCK_W<'a, O> {
    #[doc = "SEC_GPIO_MASK4_LOCK cannot be written"]
    #[inline(always)]
    pub fn cannot_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK4_LOCK_A::CANNOT_BE_WRITTEN)
    }
    #[doc = "SEC_GPIO_MASK4_LOCK can be written"]
    #[inline(always)]
    pub fn can_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK4_LOCK_A::CAN_BE_WRITTEN)
    }
}
#[doc = "Field `SEC_GPIO_MASK5_LOCK` reader - SEC_GPIO_MASK5 Lock"]
pub type SEC_GPIO_MASK5_LOCK_R = crate::FieldReader<u8, SEC_GPIO_MASK5_LOCK_A>;
#[doc = "SEC_GPIO_MASK5 Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK5_LOCK_A {
    #[doc = "1: SEC_GPIO_MASK5 cannot be written"]
    CANNOT_BE_WRITTEN = 1,
    #[doc = "2: SEC_GPIO_MASK5 can be written"]
    CAN_BE_WRITTEN = 2,
}
impl From<SEC_GPIO_MASK5_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK5_LOCK_A) -> Self {
        variant as _
    }
}
impl SEC_GPIO_MASK5_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK5_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK5_LOCK_A::CANNOT_BE_WRITTEN),
            2 => Some(SEC_GPIO_MASK5_LOCK_A::CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_cannot_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK5_LOCK_A::CANNOT_BE_WRITTEN
    }
    #[doc = "Checks if the value of the field is `CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_can_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK5_LOCK_A::CAN_BE_WRITTEN
    }
}
#[doc = "Field `SEC_GPIO_MASK5_LOCK` writer - SEC_GPIO_MASK5 Lock"]
pub type SEC_GPIO_MASK5_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEC_MASK_LOCK_SPEC, u8, SEC_GPIO_MASK5_LOCK_A, 2, O>;
impl<'a, const O: u8> SEC_GPIO_MASK5_LOCK_W<'a, O> {
    #[doc = "SEC_GPIO_MASK5 cannot be written"]
    #[inline(always)]
    pub fn cannot_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK5_LOCK_A::CANNOT_BE_WRITTEN)
    }
    #[doc = "SEC_GPIO_MASK5 can be written"]
    #[inline(always)]
    pub fn can_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK5_LOCK_A::CAN_BE_WRITTEN)
    }
}
#[doc = "Field `SEC_GPIO_MASK6_LOCK` reader - SEC_GPIO_MASK6 Lock"]
pub type SEC_GPIO_MASK6_LOCK_R = crate::FieldReader<u8, SEC_GPIO_MASK6_LOCK_A>;
#[doc = "SEC_GPIO_MASK6 Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK6_LOCK_A {
    #[doc = "1: SEC_GPIO_MASK6 cannot be written"]
    CANNOT_BE_WRITTEN = 1,
    #[doc = "2: SEC_GPIO_MASK6 can be written"]
    CAN_BE_WRITTEN = 2,
}
impl From<SEC_GPIO_MASK6_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK6_LOCK_A) -> Self {
        variant as _
    }
}
impl SEC_GPIO_MASK6_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK6_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK6_LOCK_A::CANNOT_BE_WRITTEN),
            2 => Some(SEC_GPIO_MASK6_LOCK_A::CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_cannot_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK6_LOCK_A::CANNOT_BE_WRITTEN
    }
    #[doc = "Checks if the value of the field is `CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_can_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK6_LOCK_A::CAN_BE_WRITTEN
    }
}
#[doc = "Field `SEC_GPIO_MASK6_LOCK` writer - SEC_GPIO_MASK6 Lock"]
pub type SEC_GPIO_MASK6_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEC_MASK_LOCK_SPEC, u8, SEC_GPIO_MASK6_LOCK_A, 2, O>;
impl<'a, const O: u8> SEC_GPIO_MASK6_LOCK_W<'a, O> {
    #[doc = "SEC_GPIO_MASK6 cannot be written"]
    #[inline(always)]
    pub fn cannot_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK6_LOCK_A::CANNOT_BE_WRITTEN)
    }
    #[doc = "SEC_GPIO_MASK6 can be written"]
    #[inline(always)]
    pub fn can_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK6_LOCK_A::CAN_BE_WRITTEN)
    }
}
#[doc = "Field `SEC_GPIO_MASK7_LOCK` reader - SEC_GPIO_MASK7 Lock"]
pub type SEC_GPIO_MASK7_LOCK_R = crate::FieldReader<u8, SEC_GPIO_MASK7_LOCK_A>;
#[doc = "SEC_GPIO_MASK7 Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK7_LOCK_A {
    #[doc = "1: SEC_GPIO_MASK7 cannot be written"]
    CANNOT_BE_WRITTEN = 1,
    #[doc = "2: SEC_GPIO_MASK7 can be written"]
    CAN_BE_WRITTEN = 2,
}
impl From<SEC_GPIO_MASK7_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK7_LOCK_A) -> Self {
        variant as _
    }
}
impl SEC_GPIO_MASK7_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK7_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK7_LOCK_A::CANNOT_BE_WRITTEN),
            2 => Some(SEC_GPIO_MASK7_LOCK_A::CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_cannot_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK7_LOCK_A::CANNOT_BE_WRITTEN
    }
    #[doc = "Checks if the value of the field is `CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_can_be_written(&self) -> bool {
        *self == SEC_GPIO_MASK7_LOCK_A::CAN_BE_WRITTEN
    }
}
#[doc = "Field `SEC_GPIO_MASK7_LOCK` writer - SEC_GPIO_MASK7 Lock"]
pub type SEC_GPIO_MASK7_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEC_MASK_LOCK_SPEC, u8, SEC_GPIO_MASK7_LOCK_A, 2, O>;
impl<'a, const O: u8> SEC_GPIO_MASK7_LOCK_W<'a, O> {
    #[doc = "SEC_GPIO_MASK7 cannot be written"]
    #[inline(always)]
    pub fn cannot_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK7_LOCK_A::CANNOT_BE_WRITTEN)
    }
    #[doc = "SEC_GPIO_MASK7 can be written"]
    #[inline(always)]
    pub fn can_be_written(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK7_LOCK_A::CAN_BE_WRITTEN)
    }
}
#[doc = "Field `SEC_DSP_INT_MASK_LOCK` reader - SEC_DSP_INT_MASK Lock"]
pub type SEC_DSP_INT_MASK_LOCK_R = crate::FieldReader<u8, SEC_DSP_INT_MASK_LOCK_A>;
#[doc = "SEC_DSP_INT_MASK Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_DSP_INT_MASK_LOCK_A {
    #[doc = "1: SEC_DSP_INT_MASK cannot be written"]
    CANNOT_BE_WRITTEN = 1,
    #[doc = "2: SEC_DSP_INT_MASK can be written"]
    CAN_BE_WRITTEN = 2,
}
impl From<SEC_DSP_INT_MASK_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_DSP_INT_MASK_LOCK_A) -> Self {
        variant as _
    }
}
impl SEC_DSP_INT_MASK_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_DSP_INT_MASK_LOCK_A> {
        match self.bits {
            1 => Some(SEC_DSP_INT_MASK_LOCK_A::CANNOT_BE_WRITTEN),
            2 => Some(SEC_DSP_INT_MASK_LOCK_A::CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_cannot_be_written(&self) -> bool {
        *self == SEC_DSP_INT_MASK_LOCK_A::CANNOT_BE_WRITTEN
    }
    #[doc = "Checks if the value of the field is `CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_can_be_written(&self) -> bool {
        *self == SEC_DSP_INT_MASK_LOCK_A::CAN_BE_WRITTEN
    }
}
#[doc = "Field `SEC_DSP_INT_MASK_LOCK` writer - SEC_DSP_INT_MASK Lock"]
pub type SEC_DSP_INT_MASK_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEC_MASK_LOCK_SPEC, u8, SEC_DSP_INT_MASK_LOCK_A, 2, O>;
impl<'a, const O: u8> SEC_DSP_INT_MASK_LOCK_W<'a, O> {
    #[doc = "SEC_DSP_INT_MASK cannot be written"]
    #[inline(always)]
    pub fn cannot_be_written(self) -> &'a mut W {
        self.variant(SEC_DSP_INT_MASK_LOCK_A::CANNOT_BE_WRITTEN)
    }
    #[doc = "SEC_DSP_INT_MASK can be written"]
    #[inline(always)]
    pub fn can_be_written(self) -> &'a mut W {
        self.variant(SEC_DSP_INT_MASK_LOCK_A::CAN_BE_WRITTEN)
    }
}
impl R {
    #[doc = "Bits 0:1 - Secure GPIO _MASK0 Lock"]
    #[inline(always)]
    pub fn sec_gpio_mask0_lock(&self) -> SEC_GPIO_MASK0_LOCK_R {
        SEC_GPIO_MASK0_LOCK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Secure GPIO _MASK1 Lock"]
    #[inline(always)]
    pub fn sec_gpio_mask1_lock(&self) -> SEC_GPIO_MASK1_LOCK_R {
        SEC_GPIO_MASK1_LOCK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Secure GPIO _MASK2 Lock"]
    #[inline(always)]
    pub fn sec_gpio_mask2_lock(&self) -> SEC_GPIO_MASK2_LOCK_R {
        SEC_GPIO_MASK2_LOCK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Secure GPIO _MASK3 Lock"]
    #[inline(always)]
    pub fn sec_gpio_mask3_lock(&self) -> SEC_GPIO_MASK3_LOCK_R {
        SEC_GPIO_MASK3_LOCK_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SEC_GPIO_MASK4 Lock"]
    #[inline(always)]
    pub fn sec_gpio_mask4_lock(&self) -> SEC_GPIO_MASK4_LOCK_R {
        SEC_GPIO_MASK4_LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SEC_GPIO_MASK5 Lock"]
    #[inline(always)]
    pub fn sec_gpio_mask5_lock(&self) -> SEC_GPIO_MASK5_LOCK_R {
        SEC_GPIO_MASK5_LOCK_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SEC_GPIO_MASK6 Lock"]
    #[inline(always)]
    pub fn sec_gpio_mask6_lock(&self) -> SEC_GPIO_MASK6_LOCK_R {
        SEC_GPIO_MASK6_LOCK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SEC_GPIO_MASK7 Lock"]
    #[inline(always)]
    pub fn sec_gpio_mask7_lock(&self) -> SEC_GPIO_MASK7_LOCK_R {
        SEC_GPIO_MASK7_LOCK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SEC_DSP_INT_MASK Lock"]
    #[inline(always)]
    pub fn sec_dsp_int_mask_lock(&self) -> SEC_DSP_INT_MASK_LOCK_R {
        SEC_DSP_INT_MASK_LOCK_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Secure GPIO _MASK0 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_mask0_lock(&mut self) -> SEC_GPIO_MASK0_LOCK_W<0> {
        SEC_GPIO_MASK0_LOCK_W::new(self)
    }
    #[doc = "Bits 2:3 - Secure GPIO _MASK1 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_mask1_lock(&mut self) -> SEC_GPIO_MASK1_LOCK_W<2> {
        SEC_GPIO_MASK1_LOCK_W::new(self)
    }
    #[doc = "Bits 4:5 - Secure GPIO _MASK2 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_mask2_lock(&mut self) -> SEC_GPIO_MASK2_LOCK_W<4> {
        SEC_GPIO_MASK2_LOCK_W::new(self)
    }
    #[doc = "Bits 6:7 - Secure GPIO _MASK3 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_mask3_lock(&mut self) -> SEC_GPIO_MASK3_LOCK_W<6> {
        SEC_GPIO_MASK3_LOCK_W::new(self)
    }
    #[doc = "Bits 8:9 - SEC_GPIO_MASK4 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_mask4_lock(&mut self) -> SEC_GPIO_MASK4_LOCK_W<8> {
        SEC_GPIO_MASK4_LOCK_W::new(self)
    }
    #[doc = "Bits 10:11 - SEC_GPIO_MASK5 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_mask5_lock(&mut self) -> SEC_GPIO_MASK5_LOCK_W<10> {
        SEC_GPIO_MASK5_LOCK_W::new(self)
    }
    #[doc = "Bits 12:13 - SEC_GPIO_MASK6 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_mask6_lock(&mut self) -> SEC_GPIO_MASK6_LOCK_W<12> {
        SEC_GPIO_MASK6_LOCK_W::new(self)
    }
    #[doc = "Bits 14:15 - SEC_GPIO_MASK7 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_mask7_lock(&mut self) -> SEC_GPIO_MASK7_LOCK_W<14> {
        SEC_GPIO_MASK7_LOCK_W::new(self)
    }
    #[doc = "Bits 16:17 - SEC_DSP_INT_MASK Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sec_dsp_int_mask_lock(&mut self) -> SEC_DSP_INT_MASK_LOCK_W<16> {
        SEC_DSP_INT_MASK_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Mask Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_mask_lock](index.html) module"]
pub struct SEC_MASK_LOCK_SPEC;
impl crate::RegisterSpec for SEC_MASK_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_mask_lock::R](R) reader structure"]
impl crate::Readable for SEC_MASK_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_mask_lock::W](W) writer structure"]
impl crate::Writable for SEC_MASK_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_MASK_LOCK to value 0x0002_aaaa"]
impl crate::Resettable for SEC_MASK_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_aaaa;
}
