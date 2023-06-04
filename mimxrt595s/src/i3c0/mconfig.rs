#[doc = "Register `MCONFIG` reader"]
pub struct R(crate::R<MCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCONFIG` writer"]
pub struct W(crate::W<MCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCONFIG_SPEC>;
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
impl From<crate::W<MCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTENA` reader - Master enable"]
pub type MSTENA_R = crate::FieldReader<u8, MSTENA_A>;
#[doc = "Master enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSTENA_A {
    #[doc = "0: MASTER_OFF"]
    MASTER_OFF = 0,
    #[doc = "1: MASTER_ON"]
    MASTER_ON = 1,
    #[doc = "2: MASTER_CAPABLE"]
    MASTER_CAPABLE = 2,
}
impl From<MSTENA_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTENA_A) -> Self {
        variant as _
    }
}
impl MSTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSTENA_A> {
        match self.bits {
            0 => Some(MSTENA_A::MASTER_OFF),
            1 => Some(MSTENA_A::MASTER_ON),
            2 => Some(MSTENA_A::MASTER_CAPABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_OFF`"]
    #[inline(always)]
    pub fn is_master_off(&self) -> bool {
        *self == MSTENA_A::MASTER_OFF
    }
    #[doc = "Checks if the value of the field is `MASTER_ON`"]
    #[inline(always)]
    pub fn is_master_on(&self) -> bool {
        *self == MSTENA_A::MASTER_ON
    }
    #[doc = "Checks if the value of the field is `MASTER_CAPABLE`"]
    #[inline(always)]
    pub fn is_master_capable(&self) -> bool {
        *self == MSTENA_A::MASTER_CAPABLE
    }
}
#[doc = "Field `MSTENA` writer - Master enable"]
pub type MSTENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCONFIG_SPEC, u8, MSTENA_A, 2, O>;
impl<'a, const O: u8> MSTENA_W<'a, O> {
    #[doc = "MASTER_OFF"]
    #[inline(always)]
    pub fn master_off(self) -> &'a mut W {
        self.variant(MSTENA_A::MASTER_OFF)
    }
    #[doc = "MASTER_ON"]
    #[inline(always)]
    pub fn master_on(self) -> &'a mut W {
        self.variant(MSTENA_A::MASTER_ON)
    }
    #[doc = "MASTER_CAPABLE"]
    #[inline(always)]
    pub fn master_capable(self) -> &'a mut W {
        self.variant(MSTENA_A::MASTER_CAPABLE)
    }
}
#[doc = "Field `DISTO` reader - Disable Timeout"]
pub type DISTO_R = crate::BitReader<bool>;
#[doc = "Field `DISTO` writer - Disable Timeout"]
pub type DISTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCONFIG_SPEC, bool, O>;
#[doc = "Field `HKEEP` reader - High-Keeper"]
pub type HKEEP_R = crate::FieldReader<u8, HKEEP_A>;
#[doc = "High-Keeper\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HKEEP_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: WIRED_IN"]
    WIRED_IN = 1,
    #[doc = "2: PASSIVE_SDA"]
    PASSIVE_SDA = 2,
    #[doc = "3: PASSIVE_ON_SDA_SCL"]
    PASSIVE_ON_SDA_SCL = 3,
}
impl From<HKEEP_A> for u8 {
    #[inline(always)]
    fn from(variant: HKEEP_A) -> Self {
        variant as _
    }
}
impl HKEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HKEEP_A {
        match self.bits {
            0 => HKEEP_A::NONE,
            1 => HKEEP_A::WIRED_IN,
            2 => HKEEP_A::PASSIVE_SDA,
            3 => HKEEP_A::PASSIVE_ON_SDA_SCL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == HKEEP_A::NONE
    }
    #[doc = "Checks if the value of the field is `WIRED_IN`"]
    #[inline(always)]
    pub fn is_wired_in(&self) -> bool {
        *self == HKEEP_A::WIRED_IN
    }
    #[doc = "Checks if the value of the field is `PASSIVE_SDA`"]
    #[inline(always)]
    pub fn is_passive_sda(&self) -> bool {
        *self == HKEEP_A::PASSIVE_SDA
    }
    #[doc = "Checks if the value of the field is `PASSIVE_ON_SDA_SCL`"]
    #[inline(always)]
    pub fn is_passive_on_sda_scl(&self) -> bool {
        *self == HKEEP_A::PASSIVE_ON_SDA_SCL
    }
}
#[doc = "Field `HKEEP` writer - High-Keeper"]
pub type HKEEP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCONFIG_SPEC, u8, HKEEP_A, 2, O>;
impl<'a, const O: u8> HKEEP_W<'a, O> {
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(HKEEP_A::NONE)
    }
    #[doc = "WIRED_IN"]
    #[inline(always)]
    pub fn wired_in(self) -> &'a mut W {
        self.variant(HKEEP_A::WIRED_IN)
    }
    #[doc = "PASSIVE_SDA"]
    #[inline(always)]
    pub fn passive_sda(self) -> &'a mut W {
        self.variant(HKEEP_A::PASSIVE_SDA)
    }
    #[doc = "PASSIVE_ON_SDA_SCL"]
    #[inline(always)]
    pub fn passive_on_sda_scl(self) -> &'a mut W {
        self.variant(HKEEP_A::PASSIVE_ON_SDA_SCL)
    }
}
#[doc = "Field `ODSTOP` reader - Open drain stop"]
pub type ODSTOP_R = crate::BitReader<bool>;
#[doc = "Field `ODSTOP` writer - Open drain stop"]
pub type ODSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCONFIG_SPEC, bool, O>;
#[doc = "Field `PPBAUD` reader - Push-pull baud rate"]
pub type PPBAUD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPBAUD` writer - Push-pull baud rate"]
pub type PPBAUD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `PPLOW` reader - Push-Pull low"]
pub type PPLOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPLOW` writer - Push-Pull low"]
pub type PPLOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `ODBAUD` reader - Open drain baud rate"]
pub type ODBAUD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ODBAUD` writer - Open drain baud rate"]
pub type ODBAUD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `ODHPP` reader - Open drain high push-pull"]
pub type ODHPP_R = crate::BitReader<bool>;
#[doc = "Field `ODHPP` writer - Open drain high push-pull"]
pub type ODHPP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCONFIG_SPEC, bool, O>;
#[doc = "Field `SKEW` reader - Skew"]
pub type SKEW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SKEW` writer - Skew"]
pub type SKEW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `I2CBAUD` reader - I2C baud rate"]
pub type I2CBAUD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2CBAUD` writer - I2C baud rate"]
pub type I2CBAUD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCONFIG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Master enable"]
    #[inline(always)]
    pub fn mstena(&self) -> MSTENA_R {
        MSTENA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Disable Timeout"]
    #[inline(always)]
    pub fn disto(&self) -> DISTO_R {
        DISTO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - High-Keeper"]
    #[inline(always)]
    pub fn hkeep(&self) -> HKEEP_R {
        HKEEP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Open drain stop"]
    #[inline(always)]
    pub fn odstop(&self) -> ODSTOP_R {
        ODSTOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Push-pull baud rate"]
    #[inline(always)]
    pub fn ppbaud(&self) -> PPBAUD_R {
        PPBAUD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Push-Pull low"]
    #[inline(always)]
    pub fn pplow(&self) -> PPLOW_R {
        PPLOW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Open drain baud rate"]
    #[inline(always)]
    pub fn odbaud(&self) -> ODBAUD_R {
        ODBAUD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Open drain high push-pull"]
    #[inline(always)]
    pub fn odhpp(&self) -> ODHPP_R {
        ODHPP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Skew"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - I2C baud rate"]
    #[inline(always)]
    pub fn i2cbaud(&self) -> I2CBAUD_R {
        I2CBAUD_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master enable"]
    #[inline(always)]
    #[must_use]
    pub fn mstena(&mut self) -> MSTENA_W<0> {
        MSTENA_W::new(self)
    }
    #[doc = "Bit 3 - Disable Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn disto(&mut self) -> DISTO_W<3> {
        DISTO_W::new(self)
    }
    #[doc = "Bits 4:5 - High-Keeper"]
    #[inline(always)]
    #[must_use]
    pub fn hkeep(&mut self) -> HKEEP_W<4> {
        HKEEP_W::new(self)
    }
    #[doc = "Bit 6 - Open drain stop"]
    #[inline(always)]
    #[must_use]
    pub fn odstop(&mut self) -> ODSTOP_W<6> {
        ODSTOP_W::new(self)
    }
    #[doc = "Bits 8:11 - Push-pull baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn ppbaud(&mut self) -> PPBAUD_W<8> {
        PPBAUD_W::new(self)
    }
    #[doc = "Bits 12:15 - Push-Pull low"]
    #[inline(always)]
    #[must_use]
    pub fn pplow(&mut self) -> PPLOW_W<12> {
        PPLOW_W::new(self)
    }
    #[doc = "Bits 16:23 - Open drain baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn odbaud(&mut self) -> ODBAUD_W<16> {
        ODBAUD_W::new(self)
    }
    #[doc = "Bit 24 - Open drain high push-pull"]
    #[inline(always)]
    #[must_use]
    pub fn odhpp(&mut self) -> ODHPP_W<24> {
        ODHPP_W::new(self)
    }
    #[doc = "Bits 25:27 - Skew"]
    #[inline(always)]
    #[must_use]
    pub fn skew(&mut self) -> SKEW_W<25> {
        SKEW_W::new(self)
    }
    #[doc = "Bits 28:31 - I2C baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn i2cbaud(&mut self) -> I2CBAUD_W<28> {
        I2CBAUD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mconfig](index.html) module"]
pub struct MCONFIG_SPEC;
impl crate::RegisterSpec for MCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mconfig::R](R) reader structure"]
impl crate::Readable for MCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mconfig::W](W) writer structure"]
impl crate::Writable for MCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCONFIG to value 0"]
impl crate::Resettable for MCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
