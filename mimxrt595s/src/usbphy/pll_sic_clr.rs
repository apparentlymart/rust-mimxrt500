#[doc = "Register `PLL_SIC_CLR` reader"]
pub struct R(crate::R<PLL_SIC_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_SIC_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_SIC_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_SIC_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_SIC_CLR` writer"]
pub struct W(crate::W<PLL_SIC_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_SIC_CLR_SPEC>;
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
impl From<crate::W<PLL_SIC_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_SIC_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_EN_USB_CLKS` reader - PLL_EN_USB_CLKS"]
pub type PLL_EN_USB_CLKS_R = crate::BitReader<PLL_EN_USB_CLKS_A>;
#[doc = "PLL_EN_USB_CLKS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_EN_USB_CLKS_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<PLL_EN_USB_CLKS_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_EN_USB_CLKS_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_EN_USB_CLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_EN_USB_CLKS_A {
        match self.bits {
            false => PLL_EN_USB_CLKS_A::DISABLE,
            true => PLL_EN_USB_CLKS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_EN_USB_CLKS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_EN_USB_CLKS_A::ENABLE
    }
}
#[doc = "Field `PLL_EN_USB_CLKS` writer - PLL_EN_USB_CLKS"]
pub type PLL_EN_USB_CLKS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, PLL_EN_USB_CLKS_A, O>;
impl<'a, const O: u8> PLL_EN_USB_CLKS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_EN_USB_CLKS_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_EN_USB_CLKS_A::ENABLE)
    }
}
#[doc = "Field `PLL_POWER` reader - POWER"]
pub type PLL_POWER_R = crate::BitReader<PLL_POWER_A>;
#[doc = "POWER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_POWER_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<PLL_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_POWER_A {
        match self.bits {
            false => PLL_POWER_A::DISABLE,
            true => PLL_POWER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_POWER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_POWER_A::ENABLE
    }
}
#[doc = "Field `PLL_POWER` writer - POWER"]
pub type PLL_POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, PLL_POWER_A, O>;
impl<'a, const O: u8> PLL_POWER_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_POWER_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_POWER_A::ENABLE)
    }
}
#[doc = "Field `PLL_ENABLE` reader - ENABLE"]
pub type PLL_ENABLE_R = crate::BitReader<PLL_ENABLE_A>;
#[doc = "ENABLE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_ENABLE_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<PLL_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_ENABLE_A {
        match self.bits {
            false => PLL_ENABLE_A::DISABLE,
            true => PLL_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_ENABLE_A::ENABLE
    }
}
#[doc = "Field `PLL_ENABLE` writer - ENABLE"]
pub type PLL_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, PLL_ENABLE_A, O>;
impl<'a, const O: u8> PLL_ENABLE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_ENABLE_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `PLL_BYPASS` reader - Bypass USB PLL"]
pub type PLL_BYPASS_R = crate::BitReader<PLL_BYPASS_A>;
#[doc = "Bypass USB PLL\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_BYPASS_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<PLL_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_BYPASS_A {
        match self.bits {
            false => PLL_BYPASS_A::DISABLE,
            true => PLL_BYPASS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_BYPASS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_BYPASS_A::ENABLE
    }
}
#[doc = "Field `PLL_BYPASS` writer - Bypass USB PLL"]
pub type PLL_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, PLL_BYPASS_A, O>;
impl<'a, const O: u8> PLL_BYPASS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_BYPASS_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_BYPASS_A::ENABLE)
    }
}
#[doc = "Field `REFBIAS_PWD_SEL` reader - REFBIAS_PWD_SEL"]
pub type REFBIAS_PWD_SEL_R = crate::BitReader<REFBIAS_PWD_SEL_A>;
#[doc = "REFBIAS_PWD_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBIAS_PWD_SEL_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<REFBIAS_PWD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFBIAS_PWD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl REFBIAS_PWD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBIAS_PWD_SEL_A {
        match self.bits {
            false => REFBIAS_PWD_SEL_A::DISABLE,
            true => REFBIAS_PWD_SEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REFBIAS_PWD_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REFBIAS_PWD_SEL_A::ENABLE
    }
}
#[doc = "Field `REFBIAS_PWD_SEL` writer - REFBIAS_PWD_SEL"]
pub type REFBIAS_PWD_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, REFBIAS_PWD_SEL_A, O>;
impl<'a, const O: u8> REFBIAS_PWD_SEL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SEL_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SEL_A::ENABLE)
    }
}
#[doc = "Field `REFBIAS_PWD` reader - REFBIAS_PWD"]
pub type REFBIAS_PWD_R = crate::BitReader<REFBIAS_PWD_A>;
#[doc = "REFBIAS_PWD\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBIAS_PWD_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<REFBIAS_PWD_A> for bool {
    #[inline(always)]
    fn from(variant: REFBIAS_PWD_A) -> Self {
        variant as u8 != 0
    }
}
impl REFBIAS_PWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBIAS_PWD_A {
        match self.bits {
            false => REFBIAS_PWD_A::DISABLE,
            true => REFBIAS_PWD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REFBIAS_PWD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REFBIAS_PWD_A::ENABLE
    }
}
#[doc = "Field `REFBIAS_PWD` writer - REFBIAS_PWD"]
pub type REFBIAS_PWD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, REFBIAS_PWD_A, O>;
impl<'a, const O: u8> REFBIAS_PWD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_A::ENABLE)
    }
}
#[doc = "Field `PLL_REG_ENABLE` reader - PLL_REG_ENABLE"]
pub type PLL_REG_ENABLE_R = crate::BitReader<PLL_REG_ENABLE_A>;
#[doc = "PLL_REG_ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_REG_ENABLE_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<PLL_REG_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_REG_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_REG_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_REG_ENABLE_A {
        match self.bits {
            false => PLL_REG_ENABLE_A::DISABLE,
            true => PLL_REG_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_REG_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_REG_ENABLE_A::ENABLE
    }
}
#[doc = "Field `PLL_REG_ENABLE` writer - PLL_REG_ENABLE"]
pub type PLL_REG_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, PLL_REG_ENABLE_A, O>;
impl<'a, const O: u8> PLL_REG_ENABLE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_REG_ENABLE_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_REG_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `PLL_DIV_SEL` reader - PLL_DIV_SEL"]
pub type PLL_DIV_SEL_R = crate::FieldReader<u8, PLL_DIV_SEL_A>;
#[doc = "PLL_DIV_SEL\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL_DIV_SEL_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<PLL_DIV_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_DIV_SEL_A) -> Self {
        variant as _
    }
}
impl PLL_DIV_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLL_DIV_SEL_A> {
        match self.bits {
            0 => Some(PLL_DIV_SEL_A::DISABLE),
            1 => Some(PLL_DIV_SEL_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_DIV_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_DIV_SEL_A::ENABLE
    }
}
#[doc = "Field `PLL_DIV_SEL` writer - PLL_DIV_SEL"]
pub type PLL_DIV_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_SIC_CLR_SPEC, u8, PLL_DIV_SEL_A, 3, O>;
impl<'a, const O: u8> PLL_DIV_SEL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::ENABLE)
    }
}
#[doc = "Field `PLL_LOCK` reader - PLL_LOCK"]
pub type PLL_LOCK_R = crate::BitReader<PLL_LOCK_A>;
#[doc = "PLL_LOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_LOCK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<PLL_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LOCK_A {
        match self.bits {
            false => PLL_LOCK_A::DISABLE,
            true => PLL_LOCK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_LOCK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_LOCK_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 6 - PLL_EN_USB_CLKS"]
    #[inline(always)]
    pub fn pll_en_usb_clks(&self) -> PLL_EN_USB_CLKS_R {
        PLL_EN_USB_CLKS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - POWER"]
    #[inline(always)]
    pub fn pll_power(&self) -> PLL_POWER_R {
        PLL_POWER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ENABLE"]
    #[inline(always)]
    pub fn pll_enable(&self) -> PLL_ENABLE_R {
        PLL_ENABLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Bypass USB PLL"]
    #[inline(always)]
    pub fn pll_bypass(&self) -> PLL_BYPASS_R {
        PLL_BYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - REFBIAS_PWD_SEL"]
    #[inline(always)]
    pub fn refbias_pwd_sel(&self) -> REFBIAS_PWD_SEL_R {
        REFBIAS_PWD_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - REFBIAS_PWD"]
    #[inline(always)]
    pub fn refbias_pwd(&self) -> REFBIAS_PWD_R {
        REFBIAS_PWD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PLL_REG_ENABLE"]
    #[inline(always)]
    pub fn pll_reg_enable(&self) -> PLL_REG_ENABLE_R {
        PLL_REG_ENABLE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:24 - PLL_DIV_SEL"]
    #[inline(always)]
    pub fn pll_div_sel(&self) -> PLL_DIV_SEL_R {
        PLL_DIV_SEL_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 31 - PLL_LOCK"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PLL_LOCK_R {
        PLL_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - PLL_EN_USB_CLKS"]
    #[inline(always)]
    #[must_use]
    pub fn pll_en_usb_clks(&mut self) -> PLL_EN_USB_CLKS_W<6> {
        PLL_EN_USB_CLKS_W::new(self)
    }
    #[doc = "Bit 12 - POWER"]
    #[inline(always)]
    #[must_use]
    pub fn pll_power(&mut self) -> PLL_POWER_W<12> {
        PLL_POWER_W::new(self)
    }
    #[doc = "Bit 13 - ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn pll_enable(&mut self) -> PLL_ENABLE_W<13> {
        PLL_ENABLE_W::new(self)
    }
    #[doc = "Bit 16 - Bypass USB PLL"]
    #[inline(always)]
    #[must_use]
    pub fn pll_bypass(&mut self) -> PLL_BYPASS_W<16> {
        PLL_BYPASS_W::new(self)
    }
    #[doc = "Bit 19 - REFBIAS_PWD_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn refbias_pwd_sel(&mut self) -> REFBIAS_PWD_SEL_W<19> {
        REFBIAS_PWD_SEL_W::new(self)
    }
    #[doc = "Bit 20 - REFBIAS_PWD"]
    #[inline(always)]
    #[must_use]
    pub fn refbias_pwd(&mut self) -> REFBIAS_PWD_W<20> {
        REFBIAS_PWD_W::new(self)
    }
    #[doc = "Bit 21 - PLL_REG_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn pll_reg_enable(&mut self) -> PLL_REG_ENABLE_W<21> {
        PLL_REG_ENABLE_W::new(self)
    }
    #[doc = "Bits 22:24 - PLL_DIV_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn pll_div_sel(&mut self) -> PLL_DIV_SEL_W<22> {
        PLL_DIV_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Control/Status Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_sic_clr](index.html) module"]
pub struct PLL_SIC_CLR_SPEC;
impl crate::RegisterSpec for PLL_SIC_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_sic_clr::R](R) reader structure"]
impl crate::Readable for PLL_SIC_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_sic_clr::W](W) writer structure"]
impl crate::Writable for PLL_SIC_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_SIC_CLR to value 0x00d1_2000"]
impl crate::Resettable for PLL_SIC_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00d1_2000;
}
