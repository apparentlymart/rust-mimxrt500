#[doc = "Register `MCR2` reader"]
pub struct R(crate::R<MCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR2` writer"]
pub struct W(crate::W<MCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR2_SPEC>;
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
impl From<crate::W<MCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRAHBBUFOPT` reader - Clear AHB buffer"]
pub type CLRAHBBUFOPT_R = crate::BitReader<CLRAHBBUFOPT_A>;
#[doc = "Clear AHB buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRAHBBUFOPT_A {
    #[doc = "0: AHB RX/TX Buffer will not be cleared automatically when FlexSPI returns Stop mode ACK."]
    VAL0 = 0,
    #[doc = "1: AHB RX/TX Buffer will be cleared automatically when FlexSPI returns Stop mode ACK."]
    VAL1 = 1,
}
impl From<CLRAHBBUFOPT_A> for bool {
    #[inline(always)]
    fn from(variant: CLRAHBBUFOPT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRAHBBUFOPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRAHBBUFOPT_A {
        match self.bits {
            false => CLRAHBBUFOPT_A::VAL0,
            true => CLRAHBBUFOPT_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == CLRAHBBUFOPT_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == CLRAHBBUFOPT_A::VAL1
    }
}
#[doc = "Field `CLRAHBBUFOPT` writer - Clear AHB buffer"]
pub type CLRAHBBUFOPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR2_SPEC, CLRAHBBUFOPT_A, O>;
impl<'a, const O: u8> CLRAHBBUFOPT_W<'a, O> {
    #[doc = "AHB RX/TX Buffer will not be cleared automatically when FlexSPI returns Stop mode ACK."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(CLRAHBBUFOPT_A::VAL0)
    }
    #[doc = "AHB RX/TX Buffer will be cleared automatically when FlexSPI returns Stop mode ACK."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(CLRAHBBUFOPT_A::VAL1)
    }
}
#[doc = "Field `CLRLEARNPHASE` reader - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
pub type CLRLEARNPHASE_R = crate::BitReader<CLRLEARNPHASE_A>;
#[doc = "The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRLEARNPHASE_A {
    #[doc = "0: No impact"]
    VAL0 = 0,
    #[doc = "1: The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    VAL1 = 1,
}
impl From<CLRLEARNPHASE_A> for bool {
    #[inline(always)]
    fn from(variant: CLRLEARNPHASE_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRLEARNPHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRLEARNPHASE_A {
        match self.bits {
            false => CLRLEARNPHASE_A::VAL0,
            true => CLRLEARNPHASE_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == CLRLEARNPHASE_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == CLRLEARNPHASE_A::VAL1
    }
}
#[doc = "Field `CLRLEARNPHASE` writer - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
pub type CLRLEARNPHASE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MCR2_SPEC, CLRLEARNPHASE_A, O>;
impl<'a, const O: u8> CLRLEARNPHASE_W<'a, O> {
    #[doc = "No impact"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(CLRLEARNPHASE_A::VAL0)
    }
    #[doc = "The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(CLRLEARNPHASE_A::VAL1)
    }
}
#[doc = "Field `SAMEDEVICEEN` reader - All external devices are same devices (both in type and size) for A1/A2/B1/B2."]
pub type SAMEDEVICEEN_R = crate::BitReader<SAMEDEVICEEN_A>;
#[doc = "All external devices are same devices (both in type and size) for A1/A2/B1/B2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMEDEVICEEN_A {
    #[doc = "0: In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 separately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register setting will be ignored."]
    INDIVIDUAL_PARALLEL = 0,
    #[doc = "1: FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register setting will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    ENABLE = 1,
}
impl From<SAMEDEVICEEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAMEDEVICEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMEDEVICEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMEDEVICEEN_A {
        match self.bits {
            false => SAMEDEVICEEN_A::INDIVIDUAL_PARALLEL,
            true => SAMEDEVICEEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL_PARALLEL`"]
    #[inline(always)]
    pub fn is_individual_parallel(&self) -> bool {
        *self == SAMEDEVICEEN_A::INDIVIDUAL_PARALLEL
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SAMEDEVICEEN_A::ENABLE
    }
}
#[doc = "Field `SAMEDEVICEEN` writer - All external devices are same devices (both in type and size) for A1/A2/B1/B2."]
pub type SAMEDEVICEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR2_SPEC, SAMEDEVICEEN_A, O>;
impl<'a, const O: u8> SAMEDEVICEEN_W<'a, O> {
    #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 separately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register setting will be ignored."]
    #[inline(always)]
    pub fn individual_parallel(self) -> &'a mut W {
        self.variant(SAMEDEVICEEN_A::INDIVIDUAL_PARALLEL)
    }
    #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register setting will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SAMEDEVICEEN_A::ENABLE)
    }
}
#[doc = "Field `RESUMEWAIT` reader - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
pub type RESUMEWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESUMEWAIT` writer - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
pub type RESUMEWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 11 - Clear AHB buffer"]
    #[inline(always)]
    pub fn clrahbbufopt(&self) -> CLRAHBBUFOPT_R {
        CLRAHBBUFOPT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    pub fn clrlearnphase(&self) -> CLRLEARNPHASE_R {
        CLRLEARNPHASE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - All external devices are same devices (both in type and size) for A1/A2/B1/B2."]
    #[inline(always)]
    pub fn samedeviceen(&self) -> SAMEDEVICEEN_R {
        SAMEDEVICEEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    pub fn resumewait(&self) -> RESUMEWAIT_R {
        RESUMEWAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - Clear AHB buffer"]
    #[inline(always)]
    #[must_use]
    pub fn clrahbbufopt(&mut self) -> CLRAHBBUFOPT_W<11> {
        CLRAHBBUFOPT_W::new(self)
    }
    #[doc = "Bit 14 - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    #[must_use]
    pub fn clrlearnphase(&mut self) -> CLRLEARNPHASE_W<14> {
        CLRLEARNPHASE_W::new(self)
    }
    #[doc = "Bit 15 - All external devices are same devices (both in type and size) for A1/A2/B1/B2."]
    #[inline(always)]
    #[must_use]
    pub fn samedeviceen(&mut self) -> SAMEDEVICEEN_W<15> {
        SAMEDEVICEEN_W::new(self)
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    #[must_use]
    pub fn resumewait(&mut self) -> RESUMEWAIT_W<24> {
        RESUMEWAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr2](index.html) module"]
pub struct MCR2_SPEC;
impl crate::RegisterSpec for MCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr2::R](R) reader structure"]
impl crate::Readable for MCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr2::W](W) writer structure"]
impl crate::Writable for MCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR2 to value 0x2000_81f7"]
impl crate::Resettable for MCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_81f7;
}
