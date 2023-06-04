#[doc = "Register `PLL_SIC` reader"]
pub struct R(crate::R<PLL_SIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_SIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_SIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_SIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_SIC` writer"]
pub struct W(crate::W<PLL_SIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_SIC_SPEC>;
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
impl From<crate::W<PLL_SIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_SIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_EN_USB_CLKS` reader - PLL clock enable"]
pub type PLL_EN_USB_CLKS_R = crate::BitReader<bool>;
#[doc = "Field `PLL_EN_USB_CLKS` writer - PLL clock enable"]
pub type PLL_EN_USB_CLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_SPEC, bool, O>;
#[doc = "Field `PLL_POWER` reader - Power PLL"]
pub type PLL_POWER_R = crate::BitReader<bool>;
#[doc = "Field `PLL_POWER` writer - Power PLL"]
pub type PLL_POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_SPEC, bool, O>;
#[doc = "Field `PLL_ENABLE` reader - PLL enable"]
pub type PLL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PLL_ENABLE` writer - PLL enable"]
pub type PLL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_SPEC, bool, O>;
#[doc = "Field `PLL_BYPASS` reader - Bypass USB PLL"]
pub type PLL_BYPASS_R = crate::BitReader<PLL_BYPASS_A>;
#[doc = "Bypass USB PLL\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_BYPASS_A {
    #[doc = "0: Use USB PLL"]
    ENABLE = 0,
    #[doc = "1: Bypass USB PLL"]
    DISABLE = 1,
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
            false => PLL_BYPASS_A::ENABLE,
            true => PLL_BYPASS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_BYPASS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_BYPASS_A::DISABLE
    }
}
#[doc = "Field `PLL_BYPASS` writer - Bypass USB PLL"]
pub type PLL_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_SPEC, PLL_BYPASS_A, O>;
impl<'a, const O: u8> PLL_BYPASS_W<'a, O> {
    #[doc = "Use USB PLL"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_BYPASS_A::ENABLE)
    }
    #[doc = "Bypass USB PLL"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_BYPASS_A::DISABLE)
    }
}
#[doc = "Field `REFBIAS_PWD_SEL` reader - Reference bias power control"]
pub type REFBIAS_PWD_SEL_R = crate::BitReader<REFBIAS_PWD_SEL_A>;
#[doc = "Reference bias power control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBIAS_PWD_SEL_A {
    #[doc = "0: Selects PLL_POWER to control the reference bias"]
    PWD_SEL_0 = 0,
    #[doc = "1: Selects REFBIAS_PWD to control the reference bias"]
    PWD_SEL_1 = 1,
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
            false => REFBIAS_PWD_SEL_A::PWD_SEL_0,
            true => REFBIAS_PWD_SEL_A::PWD_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWD_SEL_0`"]
    #[inline(always)]
    pub fn is_pwd_sel_0(&self) -> bool {
        *self == REFBIAS_PWD_SEL_A::PWD_SEL_0
    }
    #[doc = "Checks if the value of the field is `PWD_SEL_1`"]
    #[inline(always)]
    pub fn is_pwd_sel_1(&self) -> bool {
        *self == REFBIAS_PWD_SEL_A::PWD_SEL_1
    }
}
#[doc = "Field `REFBIAS_PWD_SEL` writer - Reference bias power control"]
pub type REFBIAS_PWD_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_SIC_SPEC, REFBIAS_PWD_SEL_A, O>;
impl<'a, const O: u8> REFBIAS_PWD_SEL_W<'a, O> {
    #[doc = "Selects PLL_POWER to control the reference bias"]
    #[inline(always)]
    pub fn pwd_sel_0(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SEL_A::PWD_SEL_0)
    }
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    #[inline(always)]
    pub fn pwd_sel_1(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SEL_A::PWD_SEL_1)
    }
}
#[doc = "Field `REFBIAS_PWD` reader - Power down Reference bias"]
pub type REFBIAS_PWD_R = crate::BitReader<bool>;
#[doc = "Field `REFBIAS_PWD` writer - Power down Reference bias"]
pub type REFBIAS_PWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_SPEC, bool, O>;
#[doc = "Field `PLL_REG_ENABLE` reader - Enable PLL regulator"]
pub type PLL_REG_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PLL_REG_ENABLE` writer - Enable PLL regulator"]
pub type PLL_REG_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_SPEC, bool, O>;
#[doc = "Field `PLL_DIV_SEL` reader - PLL Divider value"]
pub type PLL_DIV_SEL_R = crate::FieldReader<u8, PLL_DIV_SEL_A>;
#[doc = "PLL Divider value\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL_DIV_SEL_A {
    #[doc = "0: Divide by 13"]
    DIV0 = 0,
    #[doc = "1: Divide by 15"]
    DIV1 = 1,
    #[doc = "2: Divide by 16"]
    DIV2 = 2,
    #[doc = "3: Divide by 20"]
    DIV3 = 3,
    #[doc = "4: Divide by 22"]
    DIV4 = 4,
    #[doc = "5: Divide by 25"]
    DIV5 = 5,
    #[doc = "6: Divide by 30"]
    DIV6 = 6,
    #[doc = "7: Divide by 240"]
    DIV7 = 7,
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
    pub fn variant(&self) -> PLL_DIV_SEL_A {
        match self.bits {
            0 => PLL_DIV_SEL_A::DIV0,
            1 => PLL_DIV_SEL_A::DIV1,
            2 => PLL_DIV_SEL_A::DIV2,
            3 => PLL_DIV_SEL_A::DIV3,
            4 => PLL_DIV_SEL_A::DIV4,
            5 => PLL_DIV_SEL_A::DIV5,
            6 => PLL_DIV_SEL_A::DIV6,
            7 => PLL_DIV_SEL_A::DIV7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV0`"]
    #[inline(always)]
    pub fn is_div0(&self) -> bool {
        *self == PLL_DIV_SEL_A::DIV0
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLL_DIV_SEL_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLL_DIV_SEL_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLL_DIV_SEL_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLL_DIV_SEL_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLL_DIV_SEL_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLL_DIV_SEL_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLL_DIV_SEL_A::DIV7
    }
}
#[doc = "Field `PLL_DIV_SEL` writer - PLL Divider value"]
pub type PLL_DIV_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLL_SIC_SPEC, u8, PLL_DIV_SEL_A, 3, O>;
impl<'a, const O: u8> PLL_DIV_SEL_W<'a, O> {
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn div0(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::DIV0)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::DIV1)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::DIV2)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::DIV3)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::DIV4)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::DIV5)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::DIV6)
    }
    #[doc = "Divide by 240"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::DIV7)
    }
}
#[doc = "Field `PLL_LOCK` reader - USB PLL lock status indicator 0 - PLL is not currently locked 1 - PLL is currently locked"]
pub type PLL_LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 6 - PLL clock enable"]
    #[inline(always)]
    pub fn pll_en_usb_clks(&self) -> PLL_EN_USB_CLKS_R {
        PLL_EN_USB_CLKS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Power PLL"]
    #[inline(always)]
    pub fn pll_power(&self) -> PLL_POWER_R {
        PLL_POWER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLL enable"]
    #[inline(always)]
    pub fn pll_enable(&self) -> PLL_ENABLE_R {
        PLL_ENABLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Bypass USB PLL"]
    #[inline(always)]
    pub fn pll_bypass(&self) -> PLL_BYPASS_R {
        PLL_BYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Reference bias power control"]
    #[inline(always)]
    pub fn refbias_pwd_sel(&self) -> REFBIAS_PWD_SEL_R {
        REFBIAS_PWD_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Power down Reference bias"]
    #[inline(always)]
    pub fn refbias_pwd(&self) -> REFBIAS_PWD_R {
        REFBIAS_PWD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable PLL regulator"]
    #[inline(always)]
    pub fn pll_reg_enable(&self) -> PLL_REG_ENABLE_R {
        PLL_REG_ENABLE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:24 - PLL Divider value"]
    #[inline(always)]
    pub fn pll_div_sel(&self) -> PLL_DIV_SEL_R {
        PLL_DIV_SEL_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 31 - USB PLL lock status indicator 0 - PLL is not currently locked 1 - PLL is currently locked"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PLL_LOCK_R {
        PLL_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - PLL clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_en_usb_clks(&mut self) -> PLL_EN_USB_CLKS_W<6> {
        PLL_EN_USB_CLKS_W::new(self)
    }
    #[doc = "Bit 12 - Power PLL"]
    #[inline(always)]
    #[must_use]
    pub fn pll_power(&mut self) -> PLL_POWER_W<12> {
        PLL_POWER_W::new(self)
    }
    #[doc = "Bit 13 - PLL enable"]
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
    #[doc = "Bit 19 - Reference bias power control"]
    #[inline(always)]
    #[must_use]
    pub fn refbias_pwd_sel(&mut self) -> REFBIAS_PWD_SEL_W<19> {
        REFBIAS_PWD_SEL_W::new(self)
    }
    #[doc = "Bit 20 - Power down Reference bias"]
    #[inline(always)]
    #[must_use]
    pub fn refbias_pwd(&mut self) -> REFBIAS_PWD_W<20> {
        REFBIAS_PWD_W::new(self)
    }
    #[doc = "Bit 21 - Enable PLL regulator"]
    #[inline(always)]
    #[must_use]
    pub fn pll_reg_enable(&mut self) -> PLL_REG_ENABLE_W<21> {
        PLL_REG_ENABLE_W::new(self)
    }
    #[doc = "Bits 22:24 - PLL Divider value"]
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
#[doc = "PLL Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_sic](index.html) module"]
pub struct PLL_SIC_SPEC;
impl crate::RegisterSpec for PLL_SIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_sic::R](R) reader structure"]
impl crate::Readable for PLL_SIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_sic::W](W) writer structure"]
impl crate::Writable for PLL_SIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_SIC to value 0x00d1_2000"]
impl crate::Resettable for PLL_SIC_SPEC {
    const RESET_VALUE: Self::Ux = 0x00d1_2000;
}
