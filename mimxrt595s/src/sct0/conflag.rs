#[doc = "Register `CONFLAG` reader"]
pub struct R(crate::R<CONFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFLAG` writer"]
pub struct W(crate::W<CONFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFLAG_SPEC>;
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
impl From<crate::W<CONFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCFLAG0` reader - No Change Conflict Event Flag"]
pub type NCFLAG0_R = crate::BitReader<NCFLAG0_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG0_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG0_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG0_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG0_A {
        match self.bits {
            false => NCFLAG0_A::NO_NC_CONFLICT,
            true => NCFLAG0_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG0_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG0_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG0` writer - No Change Conflict Event Flag"]
pub type NCFLAG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG0_A, O>;
impl<'a, const O: u8> NCFLAG0_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG0_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG0_A::NC_CONFLICT)
    }
}
#[doc = "Field `NCFLAG1` reader - No Change Conflict Event Flag"]
pub type NCFLAG1_R = crate::BitReader<NCFLAG1_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG1_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG1_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG1_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG1_A {
        match self.bits {
            false => NCFLAG1_A::NO_NC_CONFLICT,
            true => NCFLAG1_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG1_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG1_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG1` writer - No Change Conflict Event Flag"]
pub type NCFLAG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG1_A, O>;
impl<'a, const O: u8> NCFLAG1_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG1_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG1_A::NC_CONFLICT)
    }
}
#[doc = "Field `NCFLAG2` reader - No Change Conflict Event Flag"]
pub type NCFLAG2_R = crate::BitReader<NCFLAG2_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG2_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG2_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG2_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG2_A {
        match self.bits {
            false => NCFLAG2_A::NO_NC_CONFLICT,
            true => NCFLAG2_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG2_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG2_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG2` writer - No Change Conflict Event Flag"]
pub type NCFLAG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG2_A, O>;
impl<'a, const O: u8> NCFLAG2_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG2_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG2_A::NC_CONFLICT)
    }
}
#[doc = "Field `NCFLAG3` reader - No Change Conflict Event Flag"]
pub type NCFLAG3_R = crate::BitReader<NCFLAG3_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG3_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG3_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG3_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG3_A {
        match self.bits {
            false => NCFLAG3_A::NO_NC_CONFLICT,
            true => NCFLAG3_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG3_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG3_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG3` writer - No Change Conflict Event Flag"]
pub type NCFLAG3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG3_A, O>;
impl<'a, const O: u8> NCFLAG3_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG3_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG3_A::NC_CONFLICT)
    }
}
#[doc = "Field `NCFLAG4` reader - No Change Conflict Event Flag"]
pub type NCFLAG4_R = crate::BitReader<NCFLAG4_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG4_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG4_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG4_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG4_A {
        match self.bits {
            false => NCFLAG4_A::NO_NC_CONFLICT,
            true => NCFLAG4_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG4_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG4_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG4` writer - No Change Conflict Event Flag"]
pub type NCFLAG4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG4_A, O>;
impl<'a, const O: u8> NCFLAG4_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG4_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG4_A::NC_CONFLICT)
    }
}
#[doc = "Field `NCFLAG5` reader - No Change Conflict Event Flag"]
pub type NCFLAG5_R = crate::BitReader<NCFLAG5_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG5_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG5_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG5_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG5_A {
        match self.bits {
            false => NCFLAG5_A::NO_NC_CONFLICT,
            true => NCFLAG5_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG5_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG5_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG5` writer - No Change Conflict Event Flag"]
pub type NCFLAG5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG5_A, O>;
impl<'a, const O: u8> NCFLAG5_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG5_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG5_A::NC_CONFLICT)
    }
}
#[doc = "Field `NCFLAG6` reader - No Change Conflict Event Flag"]
pub type NCFLAG6_R = crate::BitReader<NCFLAG6_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG6_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG6_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG6_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG6_A {
        match self.bits {
            false => NCFLAG6_A::NO_NC_CONFLICT,
            true => NCFLAG6_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG6_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG6_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG6` writer - No Change Conflict Event Flag"]
pub type NCFLAG6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG6_A, O>;
impl<'a, const O: u8> NCFLAG6_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG6_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG6_A::NC_CONFLICT)
    }
}
#[doc = "Field `NCFLAG7` reader - No Change Conflict Event Flag"]
pub type NCFLAG7_R = crate::BitReader<NCFLAG7_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG7_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG7_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG7_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG7_A {
        match self.bits {
            false => NCFLAG7_A::NO_NC_CONFLICT,
            true => NCFLAG7_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG7_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG7_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG7` writer - No Change Conflict Event Flag"]
pub type NCFLAG7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG7_A, O>;
impl<'a, const O: u8> NCFLAG7_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG7_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG7_A::NC_CONFLICT)
    }
}
#[doc = "Field `NCFLAG8` reader - No Change Conflict Event Flag"]
pub type NCFLAG8_R = crate::BitReader<NCFLAG8_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG8_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG8_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG8_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG8_A {
        match self.bits {
            false => NCFLAG8_A::NO_NC_CONFLICT,
            true => NCFLAG8_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG8_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG8_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG8` writer - No Change Conflict Event Flag"]
pub type NCFLAG8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG8_A, O>;
impl<'a, const O: u8> NCFLAG8_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG8_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG8_A::NC_CONFLICT)
    }
}
#[doc = "Field `NCFLAG9` reader - No Change Conflict Event Flag"]
pub type NCFLAG9_R = crate::BitReader<NCFLAG9_A>;
#[doc = "No Change Conflict Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCFLAG9_A {
    #[doc = "0: No Conflict Event"]
    NO_NC_CONFLICT = 0,
    #[doc = "1: A No Change Conflict Event occurred"]
    NC_CONFLICT = 1,
}
impl From<NCFLAG9_A> for bool {
    #[inline(always)]
    fn from(variant: NCFLAG9_A) -> Self {
        variant as u8 != 0
    }
}
impl NCFLAG9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCFLAG9_A {
        match self.bits {
            false => NCFLAG9_A::NO_NC_CONFLICT,
            true => NCFLAG9_A::NC_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_no_nc_conflict(&self) -> bool {
        *self == NCFLAG9_A::NO_NC_CONFLICT
    }
    #[doc = "Checks if the value of the field is `NC_CONFLICT`"]
    #[inline(always)]
    pub fn is_nc_conflict(&self) -> bool {
        *self == NCFLAG9_A::NC_CONFLICT
    }
}
#[doc = "Field `NCFLAG9` writer - No Change Conflict Event Flag"]
pub type NCFLAG9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, NCFLAG9_A, O>;
impl<'a, const O: u8> NCFLAG9_W<'a, O> {
    #[doc = "No Conflict Event"]
    #[inline(always)]
    pub fn no_nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG9_A::NO_NC_CONFLICT)
    }
    #[doc = "A No Change Conflict Event occurred"]
    #[inline(always)]
    pub fn nc_conflict(self) -> &'a mut W {
        self.variant(NCFLAG9_A::NC_CONFLICT)
    }
}
#[doc = "Field `BUSERRL` reader - Bus Error Low/Unified"]
pub type BUSERRL_R = crate::BitReader<bool>;
#[doc = "Field `BUSERRL` writer - Bus Error Low/Unified"]
pub type BUSERRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, bool, O>;
#[doc = "Field `BUSERRH` reader - Bus Error High"]
pub type BUSERRH_R = crate::BitReader<bool>;
#[doc = "Field `BUSERRH` writer - Bus Error High"]
pub type BUSERRH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag0(&self) -> NCFLAG0_R {
        NCFLAG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag1(&self) -> NCFLAG1_R {
        NCFLAG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag2(&self) -> NCFLAG2_R {
        NCFLAG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag3(&self) -> NCFLAG3_R {
        NCFLAG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag4(&self) -> NCFLAG4_R {
        NCFLAG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag5(&self) -> NCFLAG5_R {
        NCFLAG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag6(&self) -> NCFLAG6_R {
        NCFLAG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag7(&self) -> NCFLAG7_R {
        NCFLAG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag8(&self) -> NCFLAG8_R {
        NCFLAG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - No Change Conflict Event Flag"]
    #[inline(always)]
    pub fn ncflag9(&self) -> NCFLAG9_R {
        NCFLAG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 30 - Bus Error Low/Unified"]
    #[inline(always)]
    pub fn buserrl(&self) -> BUSERRL_R {
        BUSERRL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bus Error High"]
    #[inline(always)]
    pub fn buserrh(&self) -> BUSERRH_R {
        BUSERRH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag0(&mut self) -> NCFLAG0_W<0> {
        NCFLAG0_W::new(self)
    }
    #[doc = "Bit 1 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag1(&mut self) -> NCFLAG1_W<1> {
        NCFLAG1_W::new(self)
    }
    #[doc = "Bit 2 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag2(&mut self) -> NCFLAG2_W<2> {
        NCFLAG2_W::new(self)
    }
    #[doc = "Bit 3 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag3(&mut self) -> NCFLAG3_W<3> {
        NCFLAG3_W::new(self)
    }
    #[doc = "Bit 4 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag4(&mut self) -> NCFLAG4_W<4> {
        NCFLAG4_W::new(self)
    }
    #[doc = "Bit 5 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag5(&mut self) -> NCFLAG5_W<5> {
        NCFLAG5_W::new(self)
    }
    #[doc = "Bit 6 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag6(&mut self) -> NCFLAG6_W<6> {
        NCFLAG6_W::new(self)
    }
    #[doc = "Bit 7 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag7(&mut self) -> NCFLAG7_W<7> {
        NCFLAG7_W::new(self)
    }
    #[doc = "Bit 8 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag8(&mut self) -> NCFLAG8_W<8> {
        NCFLAG8_W::new(self)
    }
    #[doc = "Bit 9 - No Change Conflict Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncflag9(&mut self) -> NCFLAG9_W<9> {
        NCFLAG9_W::new(self)
    }
    #[doc = "Bit 30 - Bus Error Low/Unified"]
    #[inline(always)]
    #[must_use]
    pub fn buserrl(&mut self) -> BUSERRL_W<30> {
        BUSERRL_W::new(self)
    }
    #[doc = "Bit 31 - Bus Error High"]
    #[inline(always)]
    #[must_use]
    pub fn buserrh(&mut self) -> BUSERRH_W<31> {
        BUSERRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Conflict Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conflag](index.html) module"]
pub struct CONFLAG_SPEC;
impl crate::RegisterSpec for CONFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conflag::R](R) reader structure"]
impl crate::Readable for CONFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conflag::W](W) writer structure"]
impl crate::Writable for CONFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFLAG to value 0"]
impl crate::Resettable for CONFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
