#[doc = "Register `CLKGATEOVERRIDE0` reader"]
pub struct R(crate::R<CLKGATEOVERRIDE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKGATEOVERRIDE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKGATEOVERRIDE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKGATEOVERRIDE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKGATEOVERRIDE0` writer"]
pub struct W(crate::W<CLKGATEOVERRIDE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKGATEOVERRIDE0_SPEC>;
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
impl From<crate::W<CLKGATEOVERRIDE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKGATEOVERRIDE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO0` reader - SDIO0"]
pub type SDIO0_R = crate::BitReader<SDIO0_A>;
#[doc = "SDIO0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO0_A {
    #[doc = "0: Enable clock gating"]
    SDIO0_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SDIO0_1 = 1,
}
impl From<SDIO0_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO0_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO0_A {
        match self.bits {
            false => SDIO0_A::SDIO0_0,
            true => SDIO0_A::SDIO0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO0_0`"]
    #[inline(always)]
    pub fn is_sdio0_0(&self) -> bool {
        *self == SDIO0_A::SDIO0_0
    }
    #[doc = "Checks if the value of the field is `SDIO0_1`"]
    #[inline(always)]
    pub fn is_sdio0_1(&self) -> bool {
        *self == SDIO0_A::SDIO0_1
    }
}
#[doc = "Field `SDIO0` writer - SDIO0"]
pub type SDIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKGATEOVERRIDE0_SPEC, SDIO0_A, O>;
impl<'a, const O: u8> SDIO0_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sdio0_0(self) -> &'a mut W {
        self.variant(SDIO0_A::SDIO0_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sdio0_1(self) -> &'a mut W {
        self.variant(SDIO0_A::SDIO0_1)
    }
}
#[doc = "Field `SDIO1` reader - SDIO1"]
pub type SDIO1_R = crate::BitReader<SDIO1_A>;
#[doc = "SDIO1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO1_A {
    #[doc = "0: Enable clock gating"]
    SDIO1_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SDIO1_1 = 1,
}
impl From<SDIO1_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO1_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO1_A {
        match self.bits {
            false => SDIO1_A::SDIO1_0,
            true => SDIO1_A::SDIO1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO1_0`"]
    #[inline(always)]
    pub fn is_sdio1_0(&self) -> bool {
        *self == SDIO1_A::SDIO1_0
    }
    #[doc = "Checks if the value of the field is `SDIO1_1`"]
    #[inline(always)]
    pub fn is_sdio1_1(&self) -> bool {
        *self == SDIO1_A::SDIO1_1
    }
}
#[doc = "Field `SDIO1` writer - SDIO1"]
pub type SDIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKGATEOVERRIDE0_SPEC, SDIO1_A, O>;
impl<'a, const O: u8> SDIO1_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sdio1_0(self) -> &'a mut W {
        self.variant(SDIO1_A::SDIO1_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sdio1_1(self) -> &'a mut W {
        self.variant(SDIO1_A::SDIO1_1)
    }
}
#[doc = "Field `USBPHY` reader - USBPHY"]
pub type USBPHY_R = crate::BitReader<USBPHY_A>;
#[doc = "USBPHY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPHY_A {
    #[doc = "0: Enable clock gating"]
    USBPHY_0 = 0,
    #[doc = "1: Continuous Clocking"]
    USBPHY_1 = 1,
}
impl From<USBPHY_A> for bool {
    #[inline(always)]
    fn from(variant: USBPHY_A) -> Self {
        variant as u8 != 0
    }
}
impl USBPHY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPHY_A {
        match self.bits {
            false => USBPHY_A::USBPHY_0,
            true => USBPHY_A::USBPHY_1,
        }
    }
    #[doc = "Checks if the value of the field is `USBPHY_0`"]
    #[inline(always)]
    pub fn is_usbphy_0(&self) -> bool {
        *self == USBPHY_A::USBPHY_0
    }
    #[doc = "Checks if the value of the field is `USBPHY_1`"]
    #[inline(always)]
    pub fn is_usbphy_1(&self) -> bool {
        *self == USBPHY_A::USBPHY_1
    }
}
#[doc = "Field `USBPHY` writer - USBPHY"]
pub type USBPHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKGATEOVERRIDE0_SPEC, USBPHY_A, O>;
impl<'a, const O: u8> USBPHY_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn usbphy_0(self) -> &'a mut W {
        self.variant(USBPHY_A::USBPHY_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn usbphy_1(self) -> &'a mut W {
        self.variant(USBPHY_A::USBPHY_1)
    }
}
#[doc = "Field `ADC` reader - ADC"]
pub type ADC_R = crate::BitReader<ADC_A>;
#[doc = "ADC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_A {
    #[doc = "0: Enable clock gating"]
    ADC_0 = 0,
    #[doc = "1: Continuous Clocking"]
    ADC_1 = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::ADC_0,
            true => ADC_A::ADC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_0`"]
    #[inline(always)]
    pub fn is_adc_0(&self) -> bool {
        *self == ADC_A::ADC_0
    }
    #[doc = "Checks if the value of the field is `ADC_1`"]
    #[inline(always)]
    pub fn is_adc_1(&self) -> bool {
        *self == ADC_A::ADC_1
    }
}
#[doc = "Field `ADC` writer - ADC"]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKGATEOVERRIDE0_SPEC, ADC_A, O>;
impl<'a, const O: u8> ADC_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn adc_0(self) -> &'a mut W {
        self.variant(ADC_A::ADC_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn adc_1(self) -> &'a mut W {
        self.variant(ADC_A::ADC_1)
    }
}
#[doc = "Field `MU` reader - MU"]
pub type MU_R = crate::BitReader<MU_A>;
#[doc = "MU\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MU_A {
    #[doc = "0: Enable clock gating"]
    MU_0 = 0,
    #[doc = "1: Continuous Clocking"]
    MU_1 = 1,
}
impl From<MU_A> for bool {
    #[inline(always)]
    fn from(variant: MU_A) -> Self {
        variant as u8 != 0
    }
}
impl MU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MU_A {
        match self.bits {
            false => MU_A::MU_0,
            true => MU_A::MU_1,
        }
    }
    #[doc = "Checks if the value of the field is `MU_0`"]
    #[inline(always)]
    pub fn is_mu_0(&self) -> bool {
        *self == MU_A::MU_0
    }
    #[doc = "Checks if the value of the field is `MU_1`"]
    #[inline(always)]
    pub fn is_mu_1(&self) -> bool {
        *self == MU_A::MU_1
    }
}
#[doc = "Field `MU` writer - MU"]
pub type MU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKGATEOVERRIDE0_SPEC, MU_A, O>;
impl<'a, const O: u8> MU_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn mu_0(self) -> &'a mut W {
        self.variant(MU_A::MU_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn mu_1(self) -> &'a mut W {
        self.variant(MU_A::MU_1)
    }
}
#[doc = "Field `ACMP` reader - ACMP"]
pub type ACMP_R = crate::BitReader<ACMP_A>;
#[doc = "ACMP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP_A {
    #[doc = "0: Enable clock gating"]
    ACMP_0 = 0,
    #[doc = "1: Continuous Clocking"]
    ACMP_1 = 1,
}
impl From<ACMP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_A {
        match self.bits {
            false => ACMP_A::ACMP_0,
            true => ACMP_A::ACMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_0`"]
    #[inline(always)]
    pub fn is_acmp_0(&self) -> bool {
        *self == ACMP_A::ACMP_0
    }
    #[doc = "Checks if the value of the field is `ACMP_1`"]
    #[inline(always)]
    pub fn is_acmp_1(&self) -> bool {
        *self == ACMP_A::ACMP_1
    }
}
#[doc = "Field `ACMP` writer - ACMP"]
pub type ACMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKGATEOVERRIDE0_SPEC, ACMP_A, O>;
impl<'a, const O: u8> ACMP_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn acmp_0(self) -> &'a mut W {
        self.variant(ACMP_A::ACMP_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn acmp_1(self) -> &'a mut W {
        self.variant(ACMP_A::ACMP_1)
    }
}
#[doc = "Field `PMC` reader - PMC"]
pub type PMC_R = crate::BitReader<PMC_A>;
#[doc = "PMC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMC_A {
    #[doc = "0: Enable clock gating"]
    PMC_0 = 0,
    #[doc = "1: Continuous Clocking"]
    PMC_1 = 1,
}
impl From<PMC_A> for bool {
    #[inline(always)]
    fn from(variant: PMC_A) -> Self {
        variant as u8 != 0
    }
}
impl PMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMC_A {
        match self.bits {
            false => PMC_A::PMC_0,
            true => PMC_A::PMC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMC_0`"]
    #[inline(always)]
    pub fn is_pmc_0(&self) -> bool {
        *self == PMC_A::PMC_0
    }
    #[doc = "Checks if the value of the field is `PMC_1`"]
    #[inline(always)]
    pub fn is_pmc_1(&self) -> bool {
        *self == PMC_A::PMC_1
    }
}
#[doc = "Field `PMC` writer - PMC"]
pub type PMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKGATEOVERRIDE0_SPEC, PMC_A, O>;
impl<'a, const O: u8> PMC_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn pmc_0(self) -> &'a mut W {
        self.variant(PMC_A::PMC_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn pmc_1(self) -> &'a mut W {
        self.variant(PMC_A::PMC_1)
    }
}
impl R {
    #[doc = "Bit 0 - SDIO0"]
    #[inline(always)]
    pub fn sdio0(&self) -> SDIO0_R {
        SDIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDIO1"]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USBPHY"]
    #[inline(always)]
    pub fn usbphy(&self) -> USBPHY_R {
        USBPHY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MU"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACMP"]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PMC"]
    #[inline(always)]
    pub fn pmc(&self) -> PMC_R {
        PMC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDIO0"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0(&mut self) -> SDIO0_W<0> {
        SDIO0_W::new(self)
    }
    #[doc = "Bit 1 - SDIO1"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1(&mut self) -> SDIO1_W<1> {
        SDIO1_W::new(self)
    }
    #[doc = "Bit 2 - USBPHY"]
    #[inline(always)]
    #[must_use]
    pub fn usbphy(&mut self) -> USBPHY_W<2> {
        USBPHY_W::new(self)
    }
    #[doc = "Bit 3 - ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<3> {
        ADC_W::new(self)
    }
    #[doc = "Bit 4 - MU"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<4> {
        MU_W::new(self)
    }
    #[doc = "Bit 5 - ACMP"]
    #[inline(always)]
    #[must_use]
    pub fn acmp(&mut self) -> ACMP_W<5> {
        ACMP_W::new(self)
    }
    #[doc = "Bit 6 - PMC"]
    #[inline(always)]
    #[must_use]
    pub fn pmc(&mut self) -> PMC_W<6> {
        PMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock gate override 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkgateoverride0](index.html) module"]
pub struct CLKGATEOVERRIDE0_SPEC;
impl crate::RegisterSpec for CLKGATEOVERRIDE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkgateoverride0::R](R) reader structure"]
impl crate::Readable for CLKGATEOVERRIDE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkgateoverride0::W](W) writer structure"]
impl crate::Writable for CLKGATEOVERRIDE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKGATEOVERRIDE0 to value 0xffff_ffff"]
impl crate::Resettable for CLKGATEOVERRIDE0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
