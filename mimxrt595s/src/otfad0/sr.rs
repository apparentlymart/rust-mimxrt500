#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MDPCP` reader - MDPC Present"]
pub type MDPCP_R = crate::BitReader<bool>;
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Operating in Normal mode (NRM)"]
    NORMAL = 0,
    #[doc = "1: Unused (reserved)"]
    RES_01 = 1,
    #[doc = "2: Unused (reserved)"]
    RES_10_SVM = 2,
    #[doc = "3: Operating in Logically Disabled Mode (LDM)"]
    LDM = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::NORMAL,
            1 => MODE_A::RES_01,
            2 => MODE_A::RES_10_SVM,
            3 => MODE_A::LDM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RES_01`"]
    #[inline(always)]
    pub fn is_res_01(&self) -> bool {
        *self == MODE_A::RES_01
    }
    #[doc = "Checks if the value of the field is `RES_10_SVM`"]
    #[inline(always)]
    pub fn is_res_10_svm(&self) -> bool {
        *self == MODE_A::RES_10_SVM
    }
    #[doc = "Checks if the value of the field is `LDM`"]
    #[inline(always)]
    pub fn is_ldm(&self) -> bool {
        *self == MODE_A::LDM
    }
}
#[doc = "Field `NCTX` reader - Number of Contexts"]
pub type NCTX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HRL` reader - Hardware Revision Level"]
pub type HRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RRAM` reader - Restricted Register Access Mode"]
pub type RRAM_R = crate::BitReader<RRAM_A>;
#[doc = "Restricted Register Access Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRAM_A {
    #[doc = "0: Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    NORMAL = 0,
    #[doc = "1: Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    RESTRICTED = 1,
}
impl From<RRAM_A> for bool {
    #[inline(always)]
    fn from(variant: RRAM_A) -> Self {
        variant as u8 != 0
    }
}
impl RRAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRAM_A {
        match self.bits {
            false => RRAM_A::NORMAL,
            true => RRAM_A::RESTRICTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RRAM_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESTRICTED`"]
    #[inline(always)]
    pub fn is_restricted(&self) -> bool {
        *self == RRAM_A::RESTRICTED
    }
}
#[doc = "Field `GEM` reader - Global Enable Mode"]
pub type GEM_R = crate::BitReader<GEM_A>;
#[doc = "Global Enable Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GEM_A {
    #[doc = "0: OTFAD is disabled. All data fetched by the FlexSPI bypasses OTFAD processing."]
    DISABLED = 0,
    #[doc = "1: OTFAD is enabled, and processes data fetched by the FlexSPI as defined by the hardware configuration."]
    ENABLED = 1,
}
impl From<GEM_A> for bool {
    #[inline(always)]
    fn from(variant: GEM_A) -> Self {
        variant as u8 != 0
    }
}
impl GEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEM_A {
        match self.bits {
            false => GEM_A::DISABLED,
            true => GEM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GEM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GEM_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 1 - MDPC Present"]
    #[inline(always)]
    pub fn mdpcp(&self) -> MDPCP_R {
        MDPCP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Number of Contexts"]
    #[inline(always)]
    pub fn nctx(&self) -> NCTX_R {
        NCTX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Hardware Revision Level"]
    #[inline(always)]
    pub fn hrl(&self) -> HRL_R {
        HRL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Restricted Register Access Mode"]
    #[inline(always)]
    pub fn rram(&self) -> RRAM_R {
        RRAM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Global Enable Mode"]
    #[inline(always)]
    pub fn gem(&self) -> GEM_R {
        GEM_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x40"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
