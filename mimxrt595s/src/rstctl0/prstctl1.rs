#[doc = "Register `PRSTCTL1` reader"]
pub struct R(crate::R<PRSTCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSTCTL1` writer"]
pub struct W(crate::W<PRSTCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL1_SPEC>;
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
impl From<crate::W<PRSTCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO0` reader - SDIO0 reset control"]
pub type SDIO0_R = crate::BitReader<SDIO0_A>;
#[doc = "SDIO0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO0_A {
    #[doc = "0: Clear Reset"]
    SDIO0_CLR = 0,
    #[doc = "1: Set Reset"]
    SDIO0_SET = 1,
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
            false => SDIO0_A::SDIO0_CLR,
            true => SDIO0_A::SDIO0_SET,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO0_CLR`"]
    #[inline(always)]
    pub fn is_sdio0_clr(&self) -> bool {
        *self == SDIO0_A::SDIO0_CLR
    }
    #[doc = "Checks if the value of the field is `SDIO0_SET`"]
    #[inline(always)]
    pub fn is_sdio0_set(&self) -> bool {
        *self == SDIO0_A::SDIO0_SET
    }
}
#[doc = "Field `SDIO0` writer - SDIO0 reset control"]
pub type SDIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, SDIO0_A, O>;
impl<'a, const O: u8> SDIO0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn sdio0_clr(self) -> &'a mut W {
        self.variant(SDIO0_A::SDIO0_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn sdio0_set(self) -> &'a mut W {
        self.variant(SDIO0_A::SDIO0_SET)
    }
}
#[doc = "Field `SDIO1` reader - SDIO1 reset control"]
pub type SDIO1_R = crate::BitReader<SDIO1_A>;
#[doc = "SDIO1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO1_A {
    #[doc = "0: Clear Reset"]
    SDIO1_CLR = 0,
    #[doc = "1: Set Reset"]
    SDIO1_SET = 1,
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
            false => SDIO1_A::SDIO1_CLR,
            true => SDIO1_A::SDIO1_SET,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO1_CLR`"]
    #[inline(always)]
    pub fn is_sdio1_clr(&self) -> bool {
        *self == SDIO1_A::SDIO1_CLR
    }
    #[doc = "Checks if the value of the field is `SDIO1_SET`"]
    #[inline(always)]
    pub fn is_sdio1_set(&self) -> bool {
        *self == SDIO1_A::SDIO1_SET
    }
}
#[doc = "Field `SDIO1` writer - SDIO1 reset control"]
pub type SDIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, SDIO1_A, O>;
impl<'a, const O: u8> SDIO1_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn sdio1_clr(self) -> &'a mut W {
        self.variant(SDIO1_A::SDIO1_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn sdio1_set(self) -> &'a mut W {
        self.variant(SDIO1_A::SDIO1_SET)
    }
}
#[doc = "Field `ACMP0` reader - Analog comparator reset control"]
pub type ACMP0_R = crate::BitReader<ACMP0_A>;
#[doc = "Analog comparator reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP0_A {
    #[doc = "0: Clear Reset"]
    ACMP0_CLR = 0,
    #[doc = "1: Set Reset"]
    ACMP0_SET = 1,
}
impl From<ACMP0_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP0_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP0_A {
        match self.bits {
            false => ACMP0_A::ACMP0_CLR,
            true => ACMP0_A::ACMP0_SET,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP0_CLR`"]
    #[inline(always)]
    pub fn is_acmp0_clr(&self) -> bool {
        *self == ACMP0_A::ACMP0_CLR
    }
    #[doc = "Checks if the value of the field is `ACMP0_SET`"]
    #[inline(always)]
    pub fn is_acmp0_set(&self) -> bool {
        *self == ACMP0_A::ACMP0_SET
    }
}
#[doc = "Field `ACMP0` writer - Analog comparator reset control"]
pub type ACMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, ACMP0_A, O>;
impl<'a, const O: u8> ACMP0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn acmp0_clr(self) -> &'a mut W {
        self.variant(ACMP0_A::ACMP0_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn acmp0_set(self) -> &'a mut W {
        self.variant(ACMP0_A::ACMP0_SET)
    }
}
#[doc = "Field `ADC0` reader - Analog-to-Digital converter reset control"]
pub type ADC0_R = crate::BitReader<ADC0_A>;
#[doc = "Analog-to-Digital converter reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_A {
    #[doc = "0: Clear Reset"]
    ADC0_CLR = 0,
    #[doc = "1: Set Reset"]
    ADC0_SET = 1,
}
impl From<ADC0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_A {
        match self.bits {
            false => ADC0_A::ADC0_CLR,
            true => ADC0_A::ADC0_SET,
        }
    }
    #[doc = "Checks if the value of the field is `ADC0_CLR`"]
    #[inline(always)]
    pub fn is_adc0_clr(&self) -> bool {
        *self == ADC0_A::ADC0_CLR
    }
    #[doc = "Checks if the value of the field is `ADC0_SET`"]
    #[inline(always)]
    pub fn is_adc0_set(&self) -> bool {
        *self == ADC0_A::ADC0_SET
    }
}
#[doc = "Field `ADC0` writer - Analog-to-Digital converter reset control"]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, ADC0_A, O>;
impl<'a, const O: u8> ADC0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn adc0_clr(self) -> &'a mut W {
        self.variant(ADC0_A::ADC0_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn adc0_set(self) -> &'a mut W {
        self.variant(ADC0_A::ADC0_SET)
    }
}
#[doc = "Field `SHSGPIO0` reader - Secure GPIO 0 reset control"]
pub type SHSGPIO0_R = crate::BitReader<SHSGPIO0_A>;
#[doc = "Secure GPIO 0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHSGPIO0_A {
    #[doc = "0: Clear Reset"]
    SHSGPIO0_CLR = 0,
    #[doc = "1: Set Reset"]
    SHSGPIO0_SET = 1,
}
impl From<SHSGPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: SHSGPIO0_A) -> Self {
        variant as u8 != 0
    }
}
impl SHSGPIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHSGPIO0_A {
        match self.bits {
            false => SHSGPIO0_A::SHSGPIO0_CLR,
            true => SHSGPIO0_A::SHSGPIO0_SET,
        }
    }
    #[doc = "Checks if the value of the field is `SHSGPIO0_CLR`"]
    #[inline(always)]
    pub fn is_shsgpio0_clr(&self) -> bool {
        *self == SHSGPIO0_A::SHSGPIO0_CLR
    }
    #[doc = "Checks if the value of the field is `SHSGPIO0_SET`"]
    #[inline(always)]
    pub fn is_shsgpio0_set(&self) -> bool {
        *self == SHSGPIO0_A::SHSGPIO0_SET
    }
}
#[doc = "Field `SHSGPIO0` writer - Secure GPIO 0 reset control"]
pub type SHSGPIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, SHSGPIO0_A, O>;
impl<'a, const O: u8> SHSGPIO0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn shsgpio0_clr(self) -> &'a mut W {
        self.variant(SHSGPIO0_A::SHSGPIO0_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn shsgpio0_set(self) -> &'a mut W {
        self.variant(SHSGPIO0_A::SHSGPIO0_SET)
    }
}
impl R {
    #[doc = "Bit 2 - SDIO0 reset control"]
    #[inline(always)]
    pub fn sdio0(&self) -> SDIO0_R {
        SDIO0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDIO1 reset control"]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog comparator reset control"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog-to-Digital converter reset control"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Secure GPIO 0 reset control"]
    #[inline(always)]
    pub fn shsgpio0(&self) -> SHSGPIO0_R {
        SHSGPIO0_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SDIO0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0(&mut self) -> SDIO0_W<2> {
        SDIO0_W::new(self)
    }
    #[doc = "Bit 3 - SDIO1 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1(&mut self) -> SDIO1_W<3> {
        SDIO1_W::new(self)
    }
    #[doc = "Bit 15 - Analog comparator reset control"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> ACMP0_W<15> {
        ACMP0_W::new(self)
    }
    #[doc = "Bit 16 - Analog-to-Digital converter reset control"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<16> {
        ADC0_W::new(self)
    }
    #[doc = "Bit 24 - Secure GPIO 0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn shsgpio0(&mut self) -> SHSGPIO0_W<24> {
        SHSGPIO0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl1](index.html) module"]
pub struct PRSTCTL1_SPEC;
impl crate::RegisterSpec for PRSTCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstctl1::R](R) reader structure"]
impl crate::Readable for PRSTCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prstctl1::W](W) writer structure"]
impl crate::Writable for PRSTCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL1 to value 0x0101_800c"]
impl crate::Resettable for PRSTCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_800c;
}
