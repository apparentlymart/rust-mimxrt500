#[doc = "Register `AUTOCLKGATEOVERRIDE0` reader"]
pub struct R(crate::R<AUTOCLKGATEOVERRIDE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCLKGATEOVERRIDE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCLKGATEOVERRIDE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCLKGATEOVERRIDE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCLKGATEOVERRIDE0` writer"]
pub struct W(crate::W<AUTOCLKGATEOVERRIDE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCLKGATEOVERRIDE0_SPEC>;
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
impl From<crate::W<AUTOCLKGATEOVERRIDE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCLKGATEOVERRIDE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHB2APB0` reader - AHB2APB0"]
pub type AHB2APB0_R = crate::BitReader<AHB2APB0_A>;
#[doc = "AHB2APB0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB2APB0_A {
    #[doc = "0: Enable clock gating"]
    AHB2APB0_0 = 0,
    #[doc = "1: Continuous Clocking"]
    AHB2APB0_1 = 1,
}
impl From<AHB2APB0_A> for bool {
    #[inline(always)]
    fn from(variant: AHB2APB0_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB2APB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB2APB0_A {
        match self.bits {
            false => AHB2APB0_A::AHB2APB0_0,
            true => AHB2APB0_A::AHB2APB0_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB2APB0_0`"]
    #[inline(always)]
    pub fn is_ahb2apb0_0(&self) -> bool {
        *self == AHB2APB0_A::AHB2APB0_0
    }
    #[doc = "Checks if the value of the field is `AHB2APB0_1`"]
    #[inline(always)]
    pub fn is_ahb2apb0_1(&self) -> bool {
        *self == AHB2APB0_A::AHB2APB0_1
    }
}
#[doc = "Field `AHB2APB0` writer - AHB2APB0"]
pub type AHB2APB0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE0_SPEC, AHB2APB0_A, O>;
impl<'a, const O: u8> AHB2APB0_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn ahb2apb0_0(self) -> &'a mut W {
        self.variant(AHB2APB0_A::AHB2APB0_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn ahb2apb0_1(self) -> &'a mut W {
        self.variant(AHB2APB0_A::AHB2APB0_1)
    }
}
#[doc = "Field `AHB2APB1` reader - AHB2APB1"]
pub type AHB2APB1_R = crate::BitReader<AHB2APB1_A>;
#[doc = "AHB2APB1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB2APB1_A {
    #[doc = "0: Enable clock gating"]
    AHB2APB1_0 = 0,
    #[doc = "1: Continuous Clocking"]
    AHB2APB1_1 = 1,
}
impl From<AHB2APB1_A> for bool {
    #[inline(always)]
    fn from(variant: AHB2APB1_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB2APB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB2APB1_A {
        match self.bits {
            false => AHB2APB1_A::AHB2APB1_0,
            true => AHB2APB1_A::AHB2APB1_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB2APB1_0`"]
    #[inline(always)]
    pub fn is_ahb2apb1_0(&self) -> bool {
        *self == AHB2APB1_A::AHB2APB1_0
    }
    #[doc = "Checks if the value of the field is `AHB2APB1_1`"]
    #[inline(always)]
    pub fn is_ahb2apb1_1(&self) -> bool {
        *self == AHB2APB1_A::AHB2APB1_1
    }
}
#[doc = "Field `AHB2APB1` writer - AHB2APB1"]
pub type AHB2APB1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE0_SPEC, AHB2APB1_A, O>;
impl<'a, const O: u8> AHB2APB1_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn ahb2apb1_0(self) -> &'a mut W {
        self.variant(AHB2APB1_A::AHB2APB1_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn ahb2apb1_1(self) -> &'a mut W {
        self.variant(AHB2APB1_A::AHB2APB1_1)
    }
}
#[doc = "Field `CRC_ENGINE` reader - CRC_ENGINE"]
pub type CRC_ENGINE_R = crate::BitReader<CRC_ENGINE_A>;
#[doc = "CRC_ENGINE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC_ENGINE_A {
    #[doc = "0: Enable clock gating"]
    CRC_ENGINE_0 = 0,
    #[doc = "1: Continuous Clocking"]
    CRC_ENGINE_1 = 1,
}
impl From<CRC_ENGINE_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_ENGINE_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_ENGINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_ENGINE_A {
        match self.bits {
            false => CRC_ENGINE_A::CRC_ENGINE_0,
            true => CRC_ENGINE_A::CRC_ENGINE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_ENGINE_0`"]
    #[inline(always)]
    pub fn is_crc_engine_0(&self) -> bool {
        *self == CRC_ENGINE_A::CRC_ENGINE_0
    }
    #[doc = "Checks if the value of the field is `CRC_ENGINE_1`"]
    #[inline(always)]
    pub fn is_crc_engine_1(&self) -> bool {
        *self == CRC_ENGINE_A::CRC_ENGINE_1
    }
}
#[doc = "Field `CRC_ENGINE` writer - CRC_ENGINE"]
pub type CRC_ENGINE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE0_SPEC, CRC_ENGINE_A, O>;
impl<'a, const O: u8> CRC_ENGINE_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn crc_engine_0(self) -> &'a mut W {
        self.variant(CRC_ENGINE_A::CRC_ENGINE_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn crc_engine_1(self) -> &'a mut W {
        self.variant(CRC_ENGINE_A::CRC_ENGINE_1)
    }
}
#[doc = "Field `CASPER` reader - CASPER"]
pub type CASPER_R = crate::BitReader<CASPER_A>;
#[doc = "CASPER\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASPER_A {
    #[doc = "0: Enable clock gating"]
    CASPER_0 = 0,
    #[doc = "1: Continuous Clocking"]
    CASPER_1 = 1,
}
impl From<CASPER_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_A) -> Self {
        variant as u8 != 0
    }
}
impl CASPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_A {
        match self.bits {
            false => CASPER_A::CASPER_0,
            true => CASPER_A::CASPER_1,
        }
    }
    #[doc = "Checks if the value of the field is `CASPER_0`"]
    #[inline(always)]
    pub fn is_casper_0(&self) -> bool {
        *self == CASPER_A::CASPER_0
    }
    #[doc = "Checks if the value of the field is `CASPER_1`"]
    #[inline(always)]
    pub fn is_casper_1(&self) -> bool {
        *self == CASPER_A::CASPER_1
    }
}
#[doc = "Field `CASPER` writer - CASPER"]
pub type CASPER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE0_SPEC, CASPER_A, O>;
impl<'a, const O: u8> CASPER_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn casper_0(self) -> &'a mut W {
        self.variant(CASPER_A::CASPER_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn casper_1(self) -> &'a mut W {
        self.variant(CASPER_A::CASPER_1)
    }
}
#[doc = "Field `DMAC0` reader - DMAC0"]
pub type DMAC0_R = crate::BitReader<DMAC0_A>;
#[doc = "DMAC0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC0_A {
    #[doc = "0: Enable clock gating"]
    DMAC0_0 = 0,
    #[doc = "1: Continuous Clocking"]
    DMAC0_1 = 1,
}
impl From<DMAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC0_A {
        match self.bits {
            false => DMAC0_A::DMAC0_0,
            true => DMAC0_A::DMAC0_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAC0_0`"]
    #[inline(always)]
    pub fn is_dmac0_0(&self) -> bool {
        *self == DMAC0_A::DMAC0_0
    }
    #[doc = "Checks if the value of the field is `DMAC0_1`"]
    #[inline(always)]
    pub fn is_dmac0_1(&self) -> bool {
        *self == DMAC0_A::DMAC0_1
    }
}
#[doc = "Field `DMAC0` writer - DMAC0"]
pub type DMAC0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE0_SPEC, DMAC0_A, O>;
impl<'a, const O: u8> DMAC0_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn dmac0_0(self) -> &'a mut W {
        self.variant(DMAC0_A::DMAC0_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn dmac0_1(self) -> &'a mut W {
        self.variant(DMAC0_A::DMAC0_1)
    }
}
#[doc = "Field `DMAC1` reader - DMAC1"]
pub type DMAC1_R = crate::BitReader<DMAC1_A>;
#[doc = "DMAC1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC1_A {
    #[doc = "0: Enable clock gating"]
    DMAC1_0 = 0,
    #[doc = "1: Continuous Clocking"]
    DMAC1_1 = 1,
}
impl From<DMAC1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC1_A {
        match self.bits {
            false => DMAC1_A::DMAC1_0,
            true => DMAC1_A::DMAC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAC1_0`"]
    #[inline(always)]
    pub fn is_dmac1_0(&self) -> bool {
        *self == DMAC1_A::DMAC1_0
    }
    #[doc = "Checks if the value of the field is `DMAC1_1`"]
    #[inline(always)]
    pub fn is_dmac1_1(&self) -> bool {
        *self == DMAC1_A::DMAC1_1
    }
}
#[doc = "Field `DMAC1` writer - DMAC1"]
pub type DMAC1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE0_SPEC, DMAC1_A, O>;
impl<'a, const O: u8> DMAC1_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn dmac1_0(self) -> &'a mut W {
        self.variant(DMAC1_A::DMAC1_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn dmac1_1(self) -> &'a mut W {
        self.variant(DMAC1_A::DMAC1_1)
    }
}
impl R {
    #[doc = "Bit 0 - AHB2APB0"]
    #[inline(always)]
    pub fn ahb2apb0(&self) -> AHB2APB0_R {
        AHB2APB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB2APB1"]
    #[inline(always)]
    pub fn ahb2apb1(&self) -> AHB2APB1_R {
        AHB2APB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC_ENGINE"]
    #[inline(always)]
    pub fn crc_engine(&self) -> CRC_ENGINE_R {
        CRC_ENGINE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CASPER"]
    #[inline(always)]
    pub fn casper(&self) -> CASPER_R {
        CASPER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC0"]
    #[inline(always)]
    pub fn dmac0(&self) -> DMAC0_R {
        DMAC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC1"]
    #[inline(always)]
    pub fn dmac1(&self) -> DMAC1_R {
        DMAC1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB2APB0"]
    #[inline(always)]
    #[must_use]
    pub fn ahb2apb0(&mut self) -> AHB2APB0_W<0> {
        AHB2APB0_W::new(self)
    }
    #[doc = "Bit 1 - AHB2APB1"]
    #[inline(always)]
    #[must_use]
    pub fn ahb2apb1(&mut self) -> AHB2APB1_W<1> {
        AHB2APB1_W::new(self)
    }
    #[doc = "Bit 2 - CRC_ENGINE"]
    #[inline(always)]
    #[must_use]
    pub fn crc_engine(&mut self) -> CRC_ENGINE_W<2> {
        CRC_ENGINE_W::new(self)
    }
    #[doc = "Bit 3 - CASPER"]
    #[inline(always)]
    #[must_use]
    pub fn casper(&mut self) -> CASPER_W<3> {
        CASPER_W::new(self)
    }
    #[doc = "Bit 4 - DMAC0"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0(&mut self) -> DMAC0_W<4> {
        DMAC0_W::new(self)
    }
    #[doc = "Bit 5 - DMAC1"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1(&mut self) -> DMAC1_W<5> {
        DMAC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto clock gate override 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autoclkgateoverride0](index.html) module"]
pub struct AUTOCLKGATEOVERRIDE0_SPEC;
impl crate::RegisterSpec for AUTOCLKGATEOVERRIDE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autoclkgateoverride0::R](R) reader structure"]
impl crate::Readable for AUTOCLKGATEOVERRIDE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autoclkgateoverride0::W](W) writer structure"]
impl crate::Writable for AUTOCLKGATEOVERRIDE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOCLKGATEOVERRIDE0 to value 0xffff_ffff"]
impl crate::Resettable for AUTOCLKGATEOVERRIDE0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
