#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR0I` reader - Interrupt on MR0"]
pub type MR0I_R = crate::BitReader<MR0I_A>;
#[doc = "Interrupt on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR0I_A {
    #[doc = "0: Disabled"]
    MR0I_0 = 0,
    #[doc = "1: Enabled"]
    MR0I_1 = 1,
}
impl From<MR0I_A> for bool {
    #[inline(always)]
    fn from(variant: MR0I_A) -> Self {
        variant as u8 != 0
    }
}
impl MR0I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0I_A {
        match self.bits {
            false => MR0I_A::MR0I_0,
            true => MR0I_A::MR0I_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR0I_0`"]
    #[inline(always)]
    pub fn is_mr0i_0(&self) -> bool {
        *self == MR0I_A::MR0I_0
    }
    #[doc = "Checks if the value of the field is `MR0I_1`"]
    #[inline(always)]
    pub fn is_mr0i_1(&self) -> bool {
        *self == MR0I_A::MR0I_1
    }
}
#[doc = "Field `MR0I` writer - Interrupt on MR0"]
pub type MR0I_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR0I_A, O>;
impl<'a, const O: u8> MR0I_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr0i_0(self) -> &'a mut W {
        self.variant(MR0I_A::MR0I_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr0i_1(self) -> &'a mut W {
        self.variant(MR0I_A::MR0I_1)
    }
}
#[doc = "Field `MR0R` reader - Reset on MR0"]
pub type MR0R_R = crate::BitReader<MR0R_A>;
#[doc = "Reset on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR0R_A {
    #[doc = "0: Disabled"]
    MR0R_0 = 0,
    #[doc = "1: Enabled"]
    MR0R_1 = 1,
}
impl From<MR0R_A> for bool {
    #[inline(always)]
    fn from(variant: MR0R_A) -> Self {
        variant as u8 != 0
    }
}
impl MR0R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0R_A {
        match self.bits {
            false => MR0R_A::MR0R_0,
            true => MR0R_A::MR0R_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR0R_0`"]
    #[inline(always)]
    pub fn is_mr0r_0(&self) -> bool {
        *self == MR0R_A::MR0R_0
    }
    #[doc = "Checks if the value of the field is `MR0R_1`"]
    #[inline(always)]
    pub fn is_mr0r_1(&self) -> bool {
        *self == MR0R_A::MR0R_1
    }
}
#[doc = "Field `MR0R` writer - Reset on MR0"]
pub type MR0R_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR0R_A, O>;
impl<'a, const O: u8> MR0R_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr0r_0(self) -> &'a mut W {
        self.variant(MR0R_A::MR0R_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr0r_1(self) -> &'a mut W {
        self.variant(MR0R_A::MR0R_1)
    }
}
#[doc = "Field `MR0S` reader - Stop on MR0"]
pub type MR0S_R = crate::BitReader<MR0S_A>;
#[doc = "Stop on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR0S_A {
    #[doc = "0: Disabled"]
    MR0S_0 = 0,
    #[doc = "1: Enabled"]
    MR0S_1 = 1,
}
impl From<MR0S_A> for bool {
    #[inline(always)]
    fn from(variant: MR0S_A) -> Self {
        variant as u8 != 0
    }
}
impl MR0S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0S_A {
        match self.bits {
            false => MR0S_A::MR0S_0,
            true => MR0S_A::MR0S_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR0S_0`"]
    #[inline(always)]
    pub fn is_mr0s_0(&self) -> bool {
        *self == MR0S_A::MR0S_0
    }
    #[doc = "Checks if the value of the field is `MR0S_1`"]
    #[inline(always)]
    pub fn is_mr0s_1(&self) -> bool {
        *self == MR0S_A::MR0S_1
    }
}
#[doc = "Field `MR0S` writer - Stop on MR0"]
pub type MR0S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR0S_A, O>;
impl<'a, const O: u8> MR0S_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr0s_0(self) -> &'a mut W {
        self.variant(MR0S_A::MR0S_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr0s_1(self) -> &'a mut W {
        self.variant(MR0S_A::MR0S_1)
    }
}
#[doc = "Field `MR1I` reader - Interrupt on MR1"]
pub type MR1I_R = crate::BitReader<MR1I_A>;
#[doc = "Interrupt on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR1I_A {
    #[doc = "0: Disabled"]
    MR1I_0 = 0,
    #[doc = "1: Enabled"]
    MR1I_1 = 1,
}
impl From<MR1I_A> for bool {
    #[inline(always)]
    fn from(variant: MR1I_A) -> Self {
        variant as u8 != 0
    }
}
impl MR1I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1I_A {
        match self.bits {
            false => MR1I_A::MR1I_0,
            true => MR1I_A::MR1I_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR1I_0`"]
    #[inline(always)]
    pub fn is_mr1i_0(&self) -> bool {
        *self == MR1I_A::MR1I_0
    }
    #[doc = "Checks if the value of the field is `MR1I_1`"]
    #[inline(always)]
    pub fn is_mr1i_1(&self) -> bool {
        *self == MR1I_A::MR1I_1
    }
}
#[doc = "Field `MR1I` writer - Interrupt on MR1"]
pub type MR1I_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR1I_A, O>;
impl<'a, const O: u8> MR1I_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr1i_0(self) -> &'a mut W {
        self.variant(MR1I_A::MR1I_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr1i_1(self) -> &'a mut W {
        self.variant(MR1I_A::MR1I_1)
    }
}
#[doc = "Field `MR1R` reader - Reset on MR1"]
pub type MR1R_R = crate::BitReader<MR1R_A>;
#[doc = "Reset on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR1R_A {
    #[doc = "0: Disabled"]
    MR1R_0 = 0,
    #[doc = "1: Enabled"]
    MR1R_1 = 1,
}
impl From<MR1R_A> for bool {
    #[inline(always)]
    fn from(variant: MR1R_A) -> Self {
        variant as u8 != 0
    }
}
impl MR1R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1R_A {
        match self.bits {
            false => MR1R_A::MR1R_0,
            true => MR1R_A::MR1R_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR1R_0`"]
    #[inline(always)]
    pub fn is_mr1r_0(&self) -> bool {
        *self == MR1R_A::MR1R_0
    }
    #[doc = "Checks if the value of the field is `MR1R_1`"]
    #[inline(always)]
    pub fn is_mr1r_1(&self) -> bool {
        *self == MR1R_A::MR1R_1
    }
}
#[doc = "Field `MR1R` writer - Reset on MR1"]
pub type MR1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR1R_A, O>;
impl<'a, const O: u8> MR1R_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr1r_0(self) -> &'a mut W {
        self.variant(MR1R_A::MR1R_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr1r_1(self) -> &'a mut W {
        self.variant(MR1R_A::MR1R_1)
    }
}
#[doc = "Field `MR1S` reader - Stop on MR1"]
pub type MR1S_R = crate::BitReader<MR1S_A>;
#[doc = "Stop on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR1S_A {
    #[doc = "0: Disabled"]
    MRIS_0 = 0,
    #[doc = "1: Enabled"]
    MRIS_1 = 1,
}
impl From<MR1S_A> for bool {
    #[inline(always)]
    fn from(variant: MR1S_A) -> Self {
        variant as u8 != 0
    }
}
impl MR1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1S_A {
        match self.bits {
            false => MR1S_A::MRIS_0,
            true => MR1S_A::MRIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MRIS_0`"]
    #[inline(always)]
    pub fn is_mris_0(&self) -> bool {
        *self == MR1S_A::MRIS_0
    }
    #[doc = "Checks if the value of the field is `MRIS_1`"]
    #[inline(always)]
    pub fn is_mris_1(&self) -> bool {
        *self == MR1S_A::MRIS_1
    }
}
#[doc = "Field `MR1S` writer - Stop on MR1"]
pub type MR1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR1S_A, O>;
impl<'a, const O: u8> MR1S_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mris_0(self) -> &'a mut W {
        self.variant(MR1S_A::MRIS_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mris_1(self) -> &'a mut W {
        self.variant(MR1S_A::MRIS_1)
    }
}
#[doc = "Field `MR2I` reader - Interrupt on MR2"]
pub type MR2I_R = crate::BitReader<MR2I_A>;
#[doc = "Interrupt on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR2I_A {
    #[doc = "0: Disabled"]
    MR2I_0 = 0,
    #[doc = "1: Enabled"]
    MR2I_1 = 1,
}
impl From<MR2I_A> for bool {
    #[inline(always)]
    fn from(variant: MR2I_A) -> Self {
        variant as u8 != 0
    }
}
impl MR2I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2I_A {
        match self.bits {
            false => MR2I_A::MR2I_0,
            true => MR2I_A::MR2I_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR2I_0`"]
    #[inline(always)]
    pub fn is_mr2i_0(&self) -> bool {
        *self == MR2I_A::MR2I_0
    }
    #[doc = "Checks if the value of the field is `MR2I_1`"]
    #[inline(always)]
    pub fn is_mr2i_1(&self) -> bool {
        *self == MR2I_A::MR2I_1
    }
}
#[doc = "Field `MR2I` writer - Interrupt on MR2"]
pub type MR2I_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR2I_A, O>;
impl<'a, const O: u8> MR2I_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr2i_0(self) -> &'a mut W {
        self.variant(MR2I_A::MR2I_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr2i_1(self) -> &'a mut W {
        self.variant(MR2I_A::MR2I_1)
    }
}
#[doc = "Field `MR2R` reader - Reset on MR2"]
pub type MR2R_R = crate::BitReader<MR2R_A>;
#[doc = "Reset on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR2R_A {
    #[doc = "0: Disabled"]
    MR2R_0 = 0,
    #[doc = "1: Enabled"]
    MR2R_1 = 1,
}
impl From<MR2R_A> for bool {
    #[inline(always)]
    fn from(variant: MR2R_A) -> Self {
        variant as u8 != 0
    }
}
impl MR2R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2R_A {
        match self.bits {
            false => MR2R_A::MR2R_0,
            true => MR2R_A::MR2R_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR2R_0`"]
    #[inline(always)]
    pub fn is_mr2r_0(&self) -> bool {
        *self == MR2R_A::MR2R_0
    }
    #[doc = "Checks if the value of the field is `MR2R_1`"]
    #[inline(always)]
    pub fn is_mr2r_1(&self) -> bool {
        *self == MR2R_A::MR2R_1
    }
}
#[doc = "Field `MR2R` writer - Reset on MR2"]
pub type MR2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR2R_A, O>;
impl<'a, const O: u8> MR2R_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr2r_0(self) -> &'a mut W {
        self.variant(MR2R_A::MR2R_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr2r_1(self) -> &'a mut W {
        self.variant(MR2R_A::MR2R_1)
    }
}
#[doc = "Field `MR2S` reader - Stop on MR2"]
pub type MR2S_R = crate::BitReader<MR2S_A>;
#[doc = "Stop on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR2S_A {
    #[doc = "0: Disabled"]
    MR2S_0 = 0,
    #[doc = "1: Enabled"]
    MR2S_1 = 1,
}
impl From<MR2S_A> for bool {
    #[inline(always)]
    fn from(variant: MR2S_A) -> Self {
        variant as u8 != 0
    }
}
impl MR2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2S_A {
        match self.bits {
            false => MR2S_A::MR2S_0,
            true => MR2S_A::MR2S_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR2S_0`"]
    #[inline(always)]
    pub fn is_mr2s_0(&self) -> bool {
        *self == MR2S_A::MR2S_0
    }
    #[doc = "Checks if the value of the field is `MR2S_1`"]
    #[inline(always)]
    pub fn is_mr2s_1(&self) -> bool {
        *self == MR2S_A::MR2S_1
    }
}
#[doc = "Field `MR2S` writer - Stop on MR2"]
pub type MR2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR2S_A, O>;
impl<'a, const O: u8> MR2S_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr2s_0(self) -> &'a mut W {
        self.variant(MR2S_A::MR2S_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr2s_1(self) -> &'a mut W {
        self.variant(MR2S_A::MR2S_1)
    }
}
#[doc = "Field `MR3I` reader - Interrupt on MR3"]
pub type MR3I_R = crate::BitReader<MR3I_A>;
#[doc = "Interrupt on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR3I_A {
    #[doc = "0: Disabled"]
    MR3I_0 = 0,
    #[doc = "1: Enabled"]
    MR3I_1 = 1,
}
impl From<MR3I_A> for bool {
    #[inline(always)]
    fn from(variant: MR3I_A) -> Self {
        variant as u8 != 0
    }
}
impl MR3I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3I_A {
        match self.bits {
            false => MR3I_A::MR3I_0,
            true => MR3I_A::MR3I_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR3I_0`"]
    #[inline(always)]
    pub fn is_mr3i_0(&self) -> bool {
        *self == MR3I_A::MR3I_0
    }
    #[doc = "Checks if the value of the field is `MR3I_1`"]
    #[inline(always)]
    pub fn is_mr3i_1(&self) -> bool {
        *self == MR3I_A::MR3I_1
    }
}
#[doc = "Field `MR3I` writer - Interrupt on MR3"]
pub type MR3I_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR3I_A, O>;
impl<'a, const O: u8> MR3I_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr3i_0(self) -> &'a mut W {
        self.variant(MR3I_A::MR3I_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr3i_1(self) -> &'a mut W {
        self.variant(MR3I_A::MR3I_1)
    }
}
#[doc = "Field `MR3R` reader - Reset on MR3"]
pub type MR3R_R = crate::BitReader<MR3R_A>;
#[doc = "Reset on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR3R_A {
    #[doc = "0: Disabled"]
    MR3R_0 = 0,
    #[doc = "1: Enabled"]
    MR3R_1 = 1,
}
impl From<MR3R_A> for bool {
    #[inline(always)]
    fn from(variant: MR3R_A) -> Self {
        variant as u8 != 0
    }
}
impl MR3R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3R_A {
        match self.bits {
            false => MR3R_A::MR3R_0,
            true => MR3R_A::MR3R_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR3R_0`"]
    #[inline(always)]
    pub fn is_mr3r_0(&self) -> bool {
        *self == MR3R_A::MR3R_0
    }
    #[doc = "Checks if the value of the field is `MR3R_1`"]
    #[inline(always)]
    pub fn is_mr3r_1(&self) -> bool {
        *self == MR3R_A::MR3R_1
    }
}
#[doc = "Field `MR3R` writer - Reset on MR3"]
pub type MR3R_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR3R_A, O>;
impl<'a, const O: u8> MR3R_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr3r_0(self) -> &'a mut W {
        self.variant(MR3R_A::MR3R_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr3r_1(self) -> &'a mut W {
        self.variant(MR3R_A::MR3R_1)
    }
}
#[doc = "Field `MR3S` reader - Stop on MR3"]
pub type MR3S_R = crate::BitReader<MR3S_A>;
#[doc = "Stop on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR3S_A {
    #[doc = "0: Disabled"]
    MR3S_0 = 0,
    #[doc = "1: Enabled"]
    MR3S_1 = 1,
}
impl From<MR3S_A> for bool {
    #[inline(always)]
    fn from(variant: MR3S_A) -> Self {
        variant as u8 != 0
    }
}
impl MR3S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3S_A {
        match self.bits {
            false => MR3S_A::MR3S_0,
            true => MR3S_A::MR3S_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR3S_0`"]
    #[inline(always)]
    pub fn is_mr3s_0(&self) -> bool {
        *self == MR3S_A::MR3S_0
    }
    #[doc = "Checks if the value of the field is `MR3S_1`"]
    #[inline(always)]
    pub fn is_mr3s_1(&self) -> bool {
        *self == MR3S_A::MR3S_1
    }
}
#[doc = "Field `MR3S` writer - Stop on MR3"]
pub type MR3S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR3S_A, O>;
impl<'a, const O: u8> MR3S_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr3s_0(self) -> &'a mut W {
        self.variant(MR3S_A::MR3S_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr3s_1(self) -> &'a mut W {
        self.variant(MR3S_A::MR3S_1)
    }
}
#[doc = "Field `MR0RL` reader - Reload MR0"]
pub type MR0RL_R = crate::BitReader<MR0RL_A>;
#[doc = "Reload MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR0RL_A {
    #[doc = "0: Disabled"]
    MR0RL_0 = 0,
    #[doc = "1: Enabled"]
    MR0RL_1 = 1,
}
impl From<MR0RL_A> for bool {
    #[inline(always)]
    fn from(variant: MR0RL_A) -> Self {
        variant as u8 != 0
    }
}
impl MR0RL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0RL_A {
        match self.bits {
            false => MR0RL_A::MR0RL_0,
            true => MR0RL_A::MR0RL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR0RL_0`"]
    #[inline(always)]
    pub fn is_mr0rl_0(&self) -> bool {
        *self == MR0RL_A::MR0RL_0
    }
    #[doc = "Checks if the value of the field is `MR0RL_1`"]
    #[inline(always)]
    pub fn is_mr0rl_1(&self) -> bool {
        *self == MR0RL_A::MR0RL_1
    }
}
#[doc = "Field `MR0RL` writer - Reload MR0"]
pub type MR0RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR0RL_A, O>;
impl<'a, const O: u8> MR0RL_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr0rl_0(self) -> &'a mut W {
        self.variant(MR0RL_A::MR0RL_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr0rl_1(self) -> &'a mut W {
        self.variant(MR0RL_A::MR0RL_1)
    }
}
#[doc = "Field `MR1RL` reader - Reload MR1"]
pub type MR1RL_R = crate::BitReader<MR1RL_A>;
#[doc = "Reload MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR1RL_A {
    #[doc = "0: Disabled"]
    MR1RL_0 = 0,
    #[doc = "1: Enabled"]
    MR1RL_1 = 1,
}
impl From<MR1RL_A> for bool {
    #[inline(always)]
    fn from(variant: MR1RL_A) -> Self {
        variant as u8 != 0
    }
}
impl MR1RL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1RL_A {
        match self.bits {
            false => MR1RL_A::MR1RL_0,
            true => MR1RL_A::MR1RL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR1RL_0`"]
    #[inline(always)]
    pub fn is_mr1rl_0(&self) -> bool {
        *self == MR1RL_A::MR1RL_0
    }
    #[doc = "Checks if the value of the field is `MR1RL_1`"]
    #[inline(always)]
    pub fn is_mr1rl_1(&self) -> bool {
        *self == MR1RL_A::MR1RL_1
    }
}
#[doc = "Field `MR1RL` writer - Reload MR1"]
pub type MR1RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR1RL_A, O>;
impl<'a, const O: u8> MR1RL_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr1rl_0(self) -> &'a mut W {
        self.variant(MR1RL_A::MR1RL_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr1rl_1(self) -> &'a mut W {
        self.variant(MR1RL_A::MR1RL_1)
    }
}
#[doc = "Field `MR2RL` reader - Reload MR2"]
pub type MR2RL_R = crate::BitReader<MR2RL_A>;
#[doc = "Reload MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR2RL_A {
    #[doc = "0: Disabled"]
    MR2RL_0 = 0,
    #[doc = "1: Enabled"]
    MR2RL_1 = 1,
}
impl From<MR2RL_A> for bool {
    #[inline(always)]
    fn from(variant: MR2RL_A) -> Self {
        variant as u8 != 0
    }
}
impl MR2RL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2RL_A {
        match self.bits {
            false => MR2RL_A::MR2RL_0,
            true => MR2RL_A::MR2RL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR2RL_0`"]
    #[inline(always)]
    pub fn is_mr2rl_0(&self) -> bool {
        *self == MR2RL_A::MR2RL_0
    }
    #[doc = "Checks if the value of the field is `MR2RL_1`"]
    #[inline(always)]
    pub fn is_mr2rl_1(&self) -> bool {
        *self == MR2RL_A::MR2RL_1
    }
}
#[doc = "Field `MR2RL` writer - Reload MR2"]
pub type MR2RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR2RL_A, O>;
impl<'a, const O: u8> MR2RL_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr2rl_0(self) -> &'a mut W {
        self.variant(MR2RL_A::MR2RL_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr2rl_1(self) -> &'a mut W {
        self.variant(MR2RL_A::MR2RL_1)
    }
}
#[doc = "Field `MR3RL` reader - Reload MR3"]
pub type MR3RL_R = crate::BitReader<MR3RL_A>;
#[doc = "Reload MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR3RL_A {
    #[doc = "0: Disabled"]
    MR3RL_0 = 0,
    #[doc = "1: Enabled"]
    MR3RL_1 = 1,
}
impl From<MR3RL_A> for bool {
    #[inline(always)]
    fn from(variant: MR3RL_A) -> Self {
        variant as u8 != 0
    }
}
impl MR3RL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3RL_A {
        match self.bits {
            false => MR3RL_A::MR3RL_0,
            true => MR3RL_A::MR3RL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MR3RL_0`"]
    #[inline(always)]
    pub fn is_mr3rl_0(&self) -> bool {
        *self == MR3RL_A::MR3RL_0
    }
    #[doc = "Checks if the value of the field is `MR3RL_1`"]
    #[inline(always)]
    pub fn is_mr3rl_1(&self) -> bool {
        *self == MR3RL_A::MR3RL_1
    }
}
#[doc = "Field `MR3RL` writer - Reload MR3"]
pub type MR3RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MR3RL_A, O>;
impl<'a, const O: u8> MR3RL_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn mr3rl_0(self) -> &'a mut W {
        self.variant(MR3RL_A::MR3RL_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn mr3rl_1(self) -> &'a mut W {
        self.variant(MR3RL_A::MR3RL_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    pub fn mr0i(&self) -> MR0I_R {
        MR0I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    pub fn mr0r(&self) -> MR0R_R {
        MR0R_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    pub fn mr0s(&self) -> MR0S_R {
        MR0S_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    pub fn mr1i(&self) -> MR1I_R {
        MR1I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    pub fn mr1r(&self) -> MR1R_R {
        MR1R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    pub fn mr1s(&self) -> MR1S_R {
        MR1S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    pub fn mr2i(&self) -> MR2I_R {
        MR2I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    pub fn mr2r(&self) -> MR2R_R {
        MR2R_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2"]
    #[inline(always)]
    pub fn mr2s(&self) -> MR2S_R {
        MR2S_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    pub fn mr3i(&self) -> MR3I_R {
        MR3I_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    pub fn mr3r(&self) -> MR3R_R {
        MR3R_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    pub fn mr3s(&self) -> MR3S_R {
        MR3S_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - Reload MR0"]
    #[inline(always)]
    pub fn mr0rl(&self) -> MR0RL_R {
        MR0RL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reload MR1"]
    #[inline(always)]
    pub fn mr1rl(&self) -> MR1RL_R {
        MR1RL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reload MR2"]
    #[inline(always)]
    pub fn mr2rl(&self) -> MR2RL_R {
        MR2RL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reload MR3"]
    #[inline(always)]
    pub fn mr3rl(&self) -> MR3RL_R {
        MR3RL_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0i(&mut self) -> MR0I_W<0> {
        MR0I_W::new(self)
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0r(&mut self) -> MR0R_W<1> {
        MR0R_W::new(self)
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0s(&mut self) -> MR0S_W<2> {
        MR0S_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    #[must_use]
    pub fn mr1i(&mut self) -> MR1I_W<3> {
        MR1I_W::new(self)
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    #[must_use]
    pub fn mr1r(&mut self) -> MR1R_W<4> {
        MR1R_W::new(self)
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    #[must_use]
    pub fn mr1s(&mut self) -> MR1S_W<5> {
        MR1S_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    #[must_use]
    pub fn mr2i(&mut self) -> MR2I_W<6> {
        MR2I_W::new(self)
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    #[must_use]
    pub fn mr2r(&mut self) -> MR2R_W<7> {
        MR2R_W::new(self)
    }
    #[doc = "Bit 8 - Stop on MR2"]
    #[inline(always)]
    #[must_use]
    pub fn mr2s(&mut self) -> MR2S_W<8> {
        MR2S_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    #[must_use]
    pub fn mr3i(&mut self) -> MR3I_W<9> {
        MR3I_W::new(self)
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    #[must_use]
    pub fn mr3r(&mut self) -> MR3R_W<10> {
        MR3R_W::new(self)
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    #[must_use]
    pub fn mr3s(&mut self) -> MR3S_W<11> {
        MR3S_W::new(self)
    }
    #[doc = "Bit 24 - Reload MR0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0rl(&mut self) -> MR0RL_W<24> {
        MR0RL_W::new(self)
    }
    #[doc = "Bit 25 - Reload MR1"]
    #[inline(always)]
    #[must_use]
    pub fn mr1rl(&mut self) -> MR1RL_W<25> {
        MR1RL_W::new(self)
    }
    #[doc = "Bit 26 - Reload MR2"]
    #[inline(always)]
    #[must_use]
    pub fn mr2rl(&mut self) -> MR2RL_W<26> {
        MR2RL_W::new(self)
    }
    #[doc = "Bit 27 - Reload MR3"]
    #[inline(always)]
    #[must_use]
    pub fn mr3rl(&mut self) -> MR3RL_W<27> {
        MR3RL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
