#[doc = "Register `PSCCTL0_SET` reader"]
pub struct R(crate::R<PSCCTL0_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSCCTL0_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSCCTL0_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSCCTL0_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSCCTL0_SET` writer"]
pub struct W(crate::W<PSCCTL0_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCCTL0_SET_SPEC>;
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
impl From<crate::W<PSCCTL0_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCCTL0_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC0_CLK` reader - Flexcomm Interface 0 clock set"]
pub type FC0_CLK_R = crate::BitReader<FC0_CLK_A>;
#[doc = "Flexcomm Interface 0 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC0_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC0_CLK_A {
        match self.bits {
            false => FC0_CLK_A::DISABLE,
            true => FC0_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC0_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC0_CLK_A::ENABLE
    }
}
#[doc = "Field `FC0_CLK` writer - Flexcomm Interface 0 clock set"]
pub type FC0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC0_CLK_A, O>;
impl<'a, const O: u8> FC0_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC0_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC0_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC1_CLK` reader - Flexcomm Interface 1 clock set"]
pub type FC1_CLK_R = crate::BitReader<FC1_CLK_A>;
#[doc = "Flexcomm Interface 1 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC1_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC1_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC1_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC1_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC1_CLK_A {
        match self.bits {
            false => FC1_CLK_A::DISABLE,
            true => FC1_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC1_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC1_CLK_A::ENABLE
    }
}
#[doc = "Field `FC1_CLK` writer - Flexcomm Interface 1 clock set"]
pub type FC1_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC1_CLK_A, O>;
impl<'a, const O: u8> FC1_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC1_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC1_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC2_CLK` reader - Flexcomm Interface 2 clock set"]
pub type FC2_CLK_R = crate::BitReader<FC2_CLK_A>;
#[doc = "Flexcomm Interface 2 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC2_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC2_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC2_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC2_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC2_CLK_A {
        match self.bits {
            false => FC2_CLK_A::DISABLE,
            true => FC2_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC2_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC2_CLK_A::ENABLE
    }
}
#[doc = "Field `FC2_CLK` writer - Flexcomm Interface 2 clock set"]
pub type FC2_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC2_CLK_A, O>;
impl<'a, const O: u8> FC2_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC2_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC2_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC3_CLK` reader - Flexcomm Interface 3 clock set"]
pub type FC3_CLK_R = crate::BitReader<FC3_CLK_A>;
#[doc = "Flexcomm Interface 3 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC3_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC3_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC3_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC3_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC3_CLK_A {
        match self.bits {
            false => FC3_CLK_A::DISABLE,
            true => FC3_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC3_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC3_CLK_A::ENABLE
    }
}
#[doc = "Field `FC3_CLK` writer - Flexcomm Interface 3 clock set"]
pub type FC3_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC3_CLK_A, O>;
impl<'a, const O: u8> FC3_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC3_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC3_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC4_CLK` reader - Flexcomm Interface 4 clock set"]
pub type FC4_CLK_R = crate::BitReader<FC4_CLK_A>;
#[doc = "Flexcomm Interface 4 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC4_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC4_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC4_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC4_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC4_CLK_A {
        match self.bits {
            false => FC4_CLK_A::DISABLE,
            true => FC4_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC4_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC4_CLK_A::ENABLE
    }
}
#[doc = "Field `FC4_CLK` writer - Flexcomm Interface 4 clock set"]
pub type FC4_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC4_CLK_A, O>;
impl<'a, const O: u8> FC4_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC4_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC4_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC5_CLK` reader - Flexcomm Interface 5 clock set"]
pub type FC5_CLK_R = crate::BitReader<FC5_CLK_A>;
#[doc = "Flexcomm Interface 5 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC5_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC5_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC5_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC5_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC5_CLK_A {
        match self.bits {
            false => FC5_CLK_A::DISABLE,
            true => FC5_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC5_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC5_CLK_A::ENABLE
    }
}
#[doc = "Field `FC5_CLK` writer - Flexcomm Interface 5 clock set"]
pub type FC5_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC5_CLK_A, O>;
impl<'a, const O: u8> FC5_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC5_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC5_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC6_CLK` reader - Flexcomm Interface 6 clock set"]
pub type FC6_CLK_R = crate::BitReader<FC6_CLK_A>;
#[doc = "Flexcomm Interface 6 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC6_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC6_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC6_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC6_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC6_CLK_A {
        match self.bits {
            false => FC6_CLK_A::DISABLE,
            true => FC6_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC6_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC6_CLK_A::ENABLE
    }
}
#[doc = "Field `FC6_CLK` writer - Flexcomm Interface 6 clock set"]
pub type FC6_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC6_CLK_A, O>;
impl<'a, const O: u8> FC6_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC6_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC6_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC7_CLK` reader - Flexcomm Interface 7 clock set"]
pub type FC7_CLK_R = crate::BitReader<FC7_CLK_A>;
#[doc = "Flexcomm Interface 7 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC7_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC7_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC7_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC7_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC7_CLK_A {
        match self.bits {
            false => FC7_CLK_A::DISABLE,
            true => FC7_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC7_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC7_CLK_A::ENABLE
    }
}
#[doc = "Field `FC7_CLK` writer - Flexcomm Interface 7 clock set"]
pub type FC7_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC7_CLK_A, O>;
impl<'a, const O: u8> FC7_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC7_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC7_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC8_CLK` reader - Flexcomm Interface 8 clock set"]
pub type FC8_CLK_R = crate::BitReader<FC8_CLK_A>;
#[doc = "Flexcomm Interface 8 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC8_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC8_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC8_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC8_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC8_CLK_A {
        match self.bits {
            false => FC8_CLK_A::DISABLE,
            true => FC8_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC8_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC8_CLK_A::ENABLE
    }
}
#[doc = "Field `FC8_CLK` writer - Flexcomm Interface 8 clock set"]
pub type FC8_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC8_CLK_A, O>;
impl<'a, const O: u8> FC8_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC8_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC8_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC9_CLK` reader - Flexcomm Interface 9 clock set"]
pub type FC9_CLK_R = crate::BitReader<FC9_CLK_A>;
#[doc = "Flexcomm Interface 9 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC9_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC9_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC9_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC9_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC9_CLK_A {
        match self.bits {
            false => FC9_CLK_A::DISABLE,
            true => FC9_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC9_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC9_CLK_A::ENABLE
    }
}
#[doc = "Field `FC9_CLK` writer - Flexcomm Interface 9 clock set"]
pub type FC9_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC9_CLK_A, O>;
impl<'a, const O: u8> FC9_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC9_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC9_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC10_CLK` reader - Flexcomm Interface 10 clock set"]
pub type FC10_CLK_R = crate::BitReader<FC10_CLK_A>;
#[doc = "Flexcomm Interface 10 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC10_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC10_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC10_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC10_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC10_CLK_A {
        match self.bits {
            false => FC10_CLK_A::DISABLE,
            true => FC10_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC10_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC10_CLK_A::ENABLE
    }
}
#[doc = "Field `FC10_CLK` writer - Flexcomm Interface 10 clock set"]
pub type FC10_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC10_CLK_A, O>;
impl<'a, const O: u8> FC10_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC10_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC10_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC11_CLK` reader - Flexcomm Interface 11 clock set"]
pub type FC11_CLK_R = crate::BitReader<FC11_CLK_A>;
#[doc = "Flexcomm Interface 11 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC11_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC11_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC11_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC11_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC11_CLK_A {
        match self.bits {
            false => FC11_CLK_A::DISABLE,
            true => FC11_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC11_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC11_CLK_A::ENABLE
    }
}
#[doc = "Field `FC11_CLK` writer - Flexcomm Interface 11 clock set"]
pub type FC11_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC11_CLK_A, O>;
impl<'a, const O: u8> FC11_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC11_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC11_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC12_CLK` reader - Flexcomm Interface 12 clock set"]
pub type FC12_CLK_R = crate::BitReader<FC12_CLK_A>;
#[doc = "Flexcomm Interface 12 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC12_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC12_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC12_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC12_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC12_CLK_A {
        match self.bits {
            false => FC12_CLK_A::DISABLE,
            true => FC12_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC12_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC12_CLK_A::ENABLE
    }
}
#[doc = "Field `FC12_CLK` writer - Flexcomm Interface 12 clock set"]
pub type FC12_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC12_CLK_A, O>;
impl<'a, const O: u8> FC12_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC12_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC12_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC13_CLK` reader - Flexcomm Interface 13 clock set"]
pub type FC13_CLK_R = crate::BitReader<FC13_CLK_A>;
#[doc = "Flexcomm Interface 13 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC13_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC13_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC13_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC13_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC13_CLK_A {
        match self.bits {
            false => FC13_CLK_A::DISABLE,
            true => FC13_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC13_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC13_CLK_A::ENABLE
    }
}
#[doc = "Field `FC13_CLK` writer - Flexcomm Interface 13 clock set"]
pub type FC13_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC13_CLK_A, O>;
impl<'a, const O: u8> FC13_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC13_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC13_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC14_SPI_CLK` reader - Flexcomm Interface 14 SPI clock control"]
pub type FC14_SPI_CLK_R = crate::BitReader<FC14_SPI_CLK_A>;
#[doc = "Flexcomm Interface 14 SPI clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC14_SPI_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC14_SPI_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC14_SPI_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC14_SPI_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC14_SPI_CLK_A {
        match self.bits {
            false => FC14_SPI_CLK_A::DISABLE,
            true => FC14_SPI_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC14_SPI_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC14_SPI_CLK_A::ENABLE
    }
}
#[doc = "Field `FC14_SPI_CLK` writer - Flexcomm Interface 14 SPI clock control"]
pub type FC14_SPI_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC14_SPI_CLK_A, O>;
impl<'a, const O: u8> FC14_SPI_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC14_SPI_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC14_SPI_CLK_A::ENABLE)
    }
}
#[doc = "Field `FC15_I2C_CLK` reader - Flexcomm Interface 15 I2C clock control"]
pub type FC15_I2C_CLK_R = crate::BitReader<FC15_I2C_CLK_A>;
#[doc = "Flexcomm Interface 15 I2C clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC15_I2C_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC15_I2C_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC15_I2C_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC15_I2C_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC15_I2C_CLK_A {
        match self.bits {
            false => FC15_I2C_CLK_A::DISABLE,
            true => FC15_I2C_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC15_I2C_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC15_I2C_CLK_A::ENABLE
    }
}
#[doc = "Field `FC15_I2C_CLK` writer - Flexcomm Interface 15 I2C clock control"]
pub type FC15_I2C_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC15_I2C_CLK_A, O>;
impl<'a, const O: u8> FC15_I2C_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC15_I2C_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC15_I2C_CLK_A::ENABLE)
    }
}
#[doc = "Field `DMIC0` reader - DMIC0 clock control"]
pub type DMIC0_R = crate::BitReader<DMIC0_A>;
#[doc = "DMIC0 clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<DMIC0_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_A {
        match self.bits {
            false => DMIC0_A::DISABLE,
            true => DMIC0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMIC0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMIC0_A::ENABLE
    }
}
#[doc = "Field `DMIC0` writer - DMIC0 clock control"]
pub type DMIC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, DMIC0_A, O>;
impl<'a, const O: u8> DMIC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMIC0_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMIC0_A::ENABLE)
    }
}
#[doc = "Field `FC16_SPI_CLK` reader - Flexcomm Interface 16 SPI clock control"]
pub type FC16_SPI_CLK_R = crate::BitReader<FC16_SPI_CLK_A>;
#[doc = "Flexcomm Interface 16 SPI clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC16_SPI_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FC16_SPI_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FC16_SPI_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FC16_SPI_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC16_SPI_CLK_A {
        match self.bits {
            false => FC16_SPI_CLK_A::DISABLE,
            true => FC16_SPI_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC16_SPI_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC16_SPI_CLK_A::ENABLE
    }
}
#[doc = "Field `FC16_SPI_CLK` writer - Flexcomm Interface 16 SPI clock control"]
pub type FC16_SPI_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FC16_SPI_CLK_A, O>;
impl<'a, const O: u8> FC16_SPI_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC16_SPI_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC16_SPI_CLK_A::ENABLE)
    }
}
#[doc = "Field `OSEVENT_TIMER` reader - OS event timer bus clock control"]
pub type OSEVENT_TIMER_R = crate::BitReader<OSEVENT_TIMER_A>;
#[doc = "OS event timer bus clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSEVENT_TIMER_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<OSEVENT_TIMER_A> for bool {
    #[inline(always)]
    fn from(variant: OSEVENT_TIMER_A) -> Self {
        variant as u8 != 0
    }
}
impl OSEVENT_TIMER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSEVENT_TIMER_A {
        match self.bits {
            false => OSEVENT_TIMER_A::DISABLE,
            true => OSEVENT_TIMER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OSEVENT_TIMER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OSEVENT_TIMER_A::ENABLE
    }
}
#[doc = "Field `OSEVENT_TIMER` writer - OS event timer bus clock control"]
pub type OSEVENT_TIMER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, OSEVENT_TIMER_A, O>;
impl<'a, const O: u8> OSEVENT_TIMER_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OSEVENT_TIMER_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OSEVENT_TIMER_A::ENABLE)
    }
}
#[doc = "Field `FlexIO` reader - FlexIO clock control"]
pub type FLEX_IO_R = crate::BitReader<FLEX_IO_A>;
#[doc = "FlexIO clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEX_IO_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL0 bit"]
    ENABLE = 1,
}
impl From<FLEX_IO_A> for bool {
    #[inline(always)]
    fn from(variant: FLEX_IO_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEX_IO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEX_IO_A {
        match self.bits {
            false => FLEX_IO_A::DISABLE,
            true => FLEX_IO_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEX_IO_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEX_IO_A::ENABLE
    }
}
#[doc = "Field `FlexIO` writer - FlexIO clock control"]
pub type FLEX_IO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FLEX_IO_A, O>;
impl<'a, const O: u8> FLEX_IO_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEX_IO_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL0 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEX_IO_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 8 - Flexcomm Interface 0 clock set"]
    #[inline(always)]
    pub fn fc0_clk(&self) -> FC0_CLK_R {
        FC0_CLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flexcomm Interface 1 clock set"]
    #[inline(always)]
    pub fn fc1_clk(&self) -> FC1_CLK_R {
        FC1_CLK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Flexcomm Interface 2 clock set"]
    #[inline(always)]
    pub fn fc2_clk(&self) -> FC2_CLK_R {
        FC2_CLK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Flexcomm Interface 3 clock set"]
    #[inline(always)]
    pub fn fc3_clk(&self) -> FC3_CLK_R {
        FC3_CLK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Flexcomm Interface 4 clock set"]
    #[inline(always)]
    pub fn fc4_clk(&self) -> FC4_CLK_R {
        FC4_CLK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flexcomm Interface 5 clock set"]
    #[inline(always)]
    pub fn fc5_clk(&self) -> FC5_CLK_R {
        FC5_CLK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flexcomm Interface 6 clock set"]
    #[inline(always)]
    pub fn fc6_clk(&self) -> FC6_CLK_R {
        FC6_CLK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flexcomm Interface 7 clock set"]
    #[inline(always)]
    pub fn fc7_clk(&self) -> FC7_CLK_R {
        FC7_CLK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flexcomm Interface 8 clock set"]
    #[inline(always)]
    pub fn fc8_clk(&self) -> FC8_CLK_R {
        FC8_CLK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flexcomm Interface 9 clock set"]
    #[inline(always)]
    pub fn fc9_clk(&self) -> FC9_CLK_R {
        FC9_CLK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flexcomm Interface 10 clock set"]
    #[inline(always)]
    pub fn fc10_clk(&self) -> FC10_CLK_R {
        FC10_CLK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Flexcomm Interface 11 clock set"]
    #[inline(always)]
    pub fn fc11_clk(&self) -> FC11_CLK_R {
        FC11_CLK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Flexcomm Interface 12 clock set"]
    #[inline(always)]
    pub fn fc12_clk(&self) -> FC12_CLK_R {
        FC12_CLK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Flexcomm Interface 13 clock set"]
    #[inline(always)]
    pub fn fc13_clk(&self) -> FC13_CLK_R {
        FC13_CLK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Flexcomm Interface 14 SPI clock control"]
    #[inline(always)]
    pub fn fc14_spi_clk(&self) -> FC14_SPI_CLK_R {
        FC14_SPI_CLK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Flexcomm Interface 15 I2C clock control"]
    #[inline(always)]
    pub fn fc15_i2c_clk(&self) -> FC15_I2C_CLK_R {
        FC15_I2C_CLK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMIC0 clock control"]
    #[inline(always)]
    pub fn dmic0(&self) -> DMIC0_R {
        DMIC0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Flexcomm Interface 16 SPI clock control"]
    #[inline(always)]
    pub fn fc16_spi_clk(&self) -> FC16_SPI_CLK_R {
        FC16_SPI_CLK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - OS event timer bus clock control"]
    #[inline(always)]
    pub fn osevent_timer(&self) -> OSEVENT_TIMER_R {
        OSEVENT_TIMER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - FlexIO clock control"]
    #[inline(always)]
    pub fn flex_io(&self) -> FLEX_IO_R {
        FLEX_IO_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Flexcomm Interface 0 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_clk(&mut self) -> FC0_CLK_W<8> {
        FC0_CLK_W::new(self)
    }
    #[doc = "Bit 9 - Flexcomm Interface 1 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc1_clk(&mut self) -> FC1_CLK_W<9> {
        FC1_CLK_W::new(self)
    }
    #[doc = "Bit 10 - Flexcomm Interface 2 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc2_clk(&mut self) -> FC2_CLK_W<10> {
        FC2_CLK_W::new(self)
    }
    #[doc = "Bit 11 - Flexcomm Interface 3 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc3_clk(&mut self) -> FC3_CLK_W<11> {
        FC3_CLK_W::new(self)
    }
    #[doc = "Bit 12 - Flexcomm Interface 4 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc4_clk(&mut self) -> FC4_CLK_W<12> {
        FC4_CLK_W::new(self)
    }
    #[doc = "Bit 13 - Flexcomm Interface 5 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc5_clk(&mut self) -> FC5_CLK_W<13> {
        FC5_CLK_W::new(self)
    }
    #[doc = "Bit 14 - Flexcomm Interface 6 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc6_clk(&mut self) -> FC6_CLK_W<14> {
        FC6_CLK_W::new(self)
    }
    #[doc = "Bit 15 - Flexcomm Interface 7 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc7_clk(&mut self) -> FC7_CLK_W<15> {
        FC7_CLK_W::new(self)
    }
    #[doc = "Bit 16 - Flexcomm Interface 8 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc8_clk(&mut self) -> FC8_CLK_W<16> {
        FC8_CLK_W::new(self)
    }
    #[doc = "Bit 17 - Flexcomm Interface 9 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc9_clk(&mut self) -> FC9_CLK_W<17> {
        FC9_CLK_W::new(self)
    }
    #[doc = "Bit 18 - Flexcomm Interface 10 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc10_clk(&mut self) -> FC10_CLK_W<18> {
        FC10_CLK_W::new(self)
    }
    #[doc = "Bit 19 - Flexcomm Interface 11 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc11_clk(&mut self) -> FC11_CLK_W<19> {
        FC11_CLK_W::new(self)
    }
    #[doc = "Bit 20 - Flexcomm Interface 12 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc12_clk(&mut self) -> FC12_CLK_W<20> {
        FC12_CLK_W::new(self)
    }
    #[doc = "Bit 21 - Flexcomm Interface 13 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn fc13_clk(&mut self) -> FC13_CLK_W<21> {
        FC13_CLK_W::new(self)
    }
    #[doc = "Bit 22 - Flexcomm Interface 14 SPI clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc14_spi_clk(&mut self) -> FC14_SPI_CLK_W<22> {
        FC14_SPI_CLK_W::new(self)
    }
    #[doc = "Bit 23 - Flexcomm Interface 15 I2C clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc15_i2c_clk(&mut self) -> FC15_I2C_CLK_W<23> {
        FC15_I2C_CLK_W::new(self)
    }
    #[doc = "Bit 24 - DMIC0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0(&mut self) -> DMIC0_W<24> {
        DMIC0_W::new(self)
    }
    #[doc = "Bit 25 - Flexcomm Interface 16 SPI clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc16_spi_clk(&mut self) -> FC16_SPI_CLK_W<25> {
        FC16_SPI_CLK_W::new(self)
    }
    #[doc = "Bit 27 - OS event timer bus clock control"]
    #[inline(always)]
    #[must_use]
    pub fn osevent_timer(&mut self) -> OSEVENT_TIMER_W<27> {
        OSEVENT_TIMER_W::new(self)
    }
    #[doc = "Bit 29 - FlexIO clock control"]
    #[inline(always)]
    #[must_use]
    pub fn flex_io(&mut self) -> FLEX_IO_W<29> {
        FLEX_IO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Set 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscctl0_set](index.html) module"]
pub struct PSCCTL0_SET_SPEC;
impl crate::RegisterSpec for PSCCTL0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pscctl0_set::R](R) reader structure"]
impl crate::Readable for PSCCTL0_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pscctl0_set::W](W) writer structure"]
impl crate::Writable for PSCCTL0_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCCTL0_SET to value 0"]
impl crate::Resettable for PSCCTL0_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
