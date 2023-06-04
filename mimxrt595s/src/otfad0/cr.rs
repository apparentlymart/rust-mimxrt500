#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLDM` reader - Force Logically Disabled Mode"]
pub type FLDM_R = crate::BitReader<FLDM_A>;
#[doc = "Force Logically Disabled Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLDM_A {
    #[doc = "0: No effect on the operating mode."]
    NO_EFFECT = 0,
    #[doc = "1: Force entry into LDM after a write with this data bit set. SR\\[MODE\\]
signals the operating mode."]
    FORCE_LDM = 1,
}
impl From<FLDM_A> for bool {
    #[inline(always)]
    fn from(variant: FLDM_A) -> Self {
        variant as u8 != 0
    }
}
impl FLDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLDM_A {
        match self.bits {
            false => FLDM_A::NO_EFFECT,
            true => FLDM_A::FORCE_LDM,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FLDM_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `FORCE_LDM`"]
    #[inline(always)]
    pub fn is_force_ldm(&self) -> bool {
        *self == FLDM_A::FORCE_LDM
    }
}
#[doc = "Field `FLDM` writer - Force Logically Disabled Mode"]
pub type FLDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FLDM_A, O>;
impl<'a, const O: u8> FLDM_W<'a, O> {
    #[doc = "No effect on the operating mode."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FLDM_A::NO_EFFECT)
    }
    #[doc = "Force entry into LDM after a write with this data bit set. SR\\[MODE\\]
signals the operating mode."]
    #[inline(always)]
    pub fn force_ldm(self) -> &'a mut W {
        self.variant(FLDM_A::FORCE_LDM)
    }
}
#[doc = "Field `RRAE` reader - Restricted Register Access Enable"]
pub type RRAE_R = crate::BitReader<RRAE_A>;
#[doc = "Restricted Register Access Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRAE_A {
    #[doc = "0: Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    NORMAL = 0,
    #[doc = "1: Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    RESTRICT = 1,
}
impl From<RRAE_A> for bool {
    #[inline(always)]
    fn from(variant: RRAE_A) -> Self {
        variant as u8 != 0
    }
}
impl RRAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRAE_A {
        match self.bits {
            false => RRAE_A::NORMAL,
            true => RRAE_A::RESTRICT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RRAE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESTRICT`"]
    #[inline(always)]
    pub fn is_restrict(&self) -> bool {
        *self == RRAE_A::RESTRICT
    }
}
#[doc = "Field `RRAE` writer - Restricted Register Access Enable"]
pub type RRAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RRAE_A, O>;
impl<'a, const O: u8> RRAE_W<'a, O> {
    #[doc = "Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RRAE_A::NORMAL)
    }
    #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    #[inline(always)]
    pub fn restrict(self) -> &'a mut W {
        self.variant(RRAE_A::RESTRICT)
    }
}
#[doc = "Field `GE` reader - Global OTFAD Enable"]
pub type GE_R = crate::BitReader<GE_A>;
#[doc = "Global OTFAD Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE_A {
    #[doc = "0: OTFAD has decryption disabled. All data fetched by the FlexSPI bypasses OTFAD processing."]
    DISABLE = 0,
    #[doc = "1: OTFAD has decryption enabled, and processes data fetched by the FlexSPI as defined by the hardware configuration."]
    ENABLE = 1,
}
impl From<GE_A> for bool {
    #[inline(always)]
    fn from(variant: GE_A) -> Self {
        variant as u8 != 0
    }
}
impl GE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GE_A {
        match self.bits {
            false => GE_A::DISABLE,
            true => GE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GE_A::ENABLE
    }
}
#[doc = "Field `GE` writer - Global OTFAD Enable"]
pub type GE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, GE_A, O>;
impl<'a, const O: u8> GE_W<'a, O> {
    #[doc = "OTFAD has decryption disabled. All data fetched by the FlexSPI bypasses OTFAD processing."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GE_A::DISABLE)
    }
    #[doc = "OTFAD has decryption enabled, and processes data fetched by the FlexSPI as defined by the hardware configuration."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 3 - Force Logically Disabled Mode"]
    #[inline(always)]
    pub fn fldm(&self) -> FLDM_R {
        FLDM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Restricted Register Access Enable"]
    #[inline(always)]
    pub fn rrae(&self) -> RRAE_R {
        RRAE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Global OTFAD Enable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Force Logically Disabled Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fldm(&mut self) -> FLDM_W<3> {
        FLDM_W::new(self)
    }
    #[doc = "Bit 7 - Restricted Register Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrae(&mut self) -> RRAE_W<7> {
        RRAE_W::new(self)
    }
    #[doc = "Bit 31 - Global OTFAD Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<31> {
        GE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
