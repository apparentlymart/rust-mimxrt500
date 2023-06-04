#[doc = "Register `PanelConfig0` reader"]
pub struct R(crate::R<PANEL_CONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PANEL_CONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PANEL_CONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PANEL_CONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PanelConfig0` writer"]
pub struct W(crate::W<PANEL_CONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PANEL_CONFIG0_SPEC>;
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
impl From<crate::W<PANEL_CONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PANEL_CONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DE` reader - Data Enable"]
pub type DE_R = crate::BitReader<DE_A>;
#[doc = "Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DE_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<DE_A> for bool {
    #[inline(always)]
    fn from(variant: DE_A) -> Self {
        variant as u8 != 0
    }
}
impl DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DE_A {
        match self.bits {
            false => DE_A::DISABLE,
            true => DE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DE_A::ENABLE
    }
}
#[doc = "Field `DE` writer - Data Enable"]
pub type DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PANEL_CONFIG0_SPEC, DE_A, O>;
impl<'a, const O: u8> DE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DE_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DE_A::ENABLE)
    }
}
#[doc = "Field `DE_POLARITY` reader - Data Enable Polarity"]
pub type DE_POLARITY_R = crate::BitReader<DE_POLARITY_A>;
#[doc = "Data Enable Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DE_POLARITY_A {
    #[doc = "0: Positive"]
    POSITIVE = 0,
    #[doc = "1: Negative"]
    NEGATIVE = 1,
}
impl From<DE_POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: DE_POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl DE_POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DE_POLARITY_A {
        match self.bits {
            false => DE_POLARITY_A::POSITIVE,
            true => DE_POLARITY_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DE_POLARITY_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DE_POLARITY_A::NEGATIVE
    }
}
#[doc = "Field `DE_POLARITY` writer - Data Enable Polarity"]
pub type DE_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PANEL_CONFIG0_SPEC, DE_POLARITY_A, O>;
impl<'a, const O: u8> DE_POLARITY_W<'a, O> {
    #[doc = "Positive"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(DE_POLARITY_A::POSITIVE)
    }
    #[doc = "Negative"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(DE_POLARITY_A::NEGATIVE)
    }
}
#[doc = "Field `DATA_POLARITY` reader - Data Polarity"]
pub type DATA_POLARITY_R = crate::BitReader<DATA_POLARITY_A>;
#[doc = "Data Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_POLARITY_A {
    #[doc = "0: Positive"]
    POSITIVE = 0,
    #[doc = "1: Negative"]
    NEGATIVE = 1,
}
impl From<DATA_POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA_POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_POLARITY_A {
        match self.bits {
            false => DATA_POLARITY_A::POSITIVE,
            true => DATA_POLARITY_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DATA_POLARITY_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DATA_POLARITY_A::NEGATIVE
    }
}
#[doc = "Field `DATA_POLARITY` writer - Data Polarity"]
pub type DATA_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PANEL_CONFIG0_SPEC, DATA_POLARITY_A, O>;
impl<'a, const O: u8> DATA_POLARITY_W<'a, O> {
    #[doc = "Positive"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(DATA_POLARITY_A::POSITIVE)
    }
    #[doc = "Negative"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(DATA_POLARITY_A::NEGATIVE)
    }
}
#[doc = "Field `CLOCK` reader - Clock Enable/Disable"]
pub type CLOCK_R = crate::BitReader<CLOCK_A>;
#[doc = "Clock Enable/Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLOCK_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<CLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl CLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_A {
        match self.bits {
            false => CLOCK_A::DISABLE,
            true => CLOCK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLOCK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLOCK_A::ENABLE
    }
}
#[doc = "Field `CLOCK` writer - Clock Enable/Disable"]
pub type CLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PANEL_CONFIG0_SPEC, CLOCK_A, O>;
impl<'a, const O: u8> CLOCK_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLOCK_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLOCK_A::ENABLE)
    }
}
#[doc = "Field `CLOCK_POLARITY` reader - Clock Polarity"]
pub type CLOCK_POLARITY_R = crate::BitReader<CLOCK_POLARITY_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLOCK_POLARITY_A {
    #[doc = "0: Positive"]
    ENABLE = 0,
    #[doc = "1: Negative"]
    DISABLE = 1,
}
impl From<CLOCK_POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl CLOCK_POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_POLARITY_A {
        match self.bits {
            false => CLOCK_POLARITY_A::ENABLE,
            true => CLOCK_POLARITY_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLOCK_POLARITY_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLOCK_POLARITY_A::DISABLE
    }
}
#[doc = "Field `CLOCK_POLARITY` writer - Clock Polarity"]
pub type CLOCK_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PANEL_CONFIG0_SPEC, CLOCK_POLARITY_A, O>;
impl<'a, const O: u8> CLOCK_POLARITY_W<'a, O> {
    #[doc = "Positive"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLOCK_POLARITY_A::ENABLE)
    }
    #[doc = "Negative"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLOCK_POLARITY_A::DISABLE)
    }
}
#[doc = "Field `SEQUENCING` reader - Enable software or hardware panel sequencing."]
pub type SEQUENCING_R = crate::BitReader<SEQUENCING_A>;
#[doc = "Enable software or hardware panel sequencing.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQUENCING_A {
    #[doc = "0: Hardware"]
    HARDWARE = 0,
    #[doc = "1: Software"]
    SOFTWARE = 1,
}
impl From<SEQUENCING_A> for bool {
    #[inline(always)]
    fn from(variant: SEQUENCING_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQUENCING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQUENCING_A {
        match self.bits {
            false => SEQUENCING_A::HARDWARE,
            true => SEQUENCING_A::SOFTWARE,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == SEQUENCING_A::HARDWARE
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == SEQUENCING_A::SOFTWARE
    }
}
#[doc = "Field `SEQUENCING` writer - Enable software or hardware panel sequencing."]
pub type SEQUENCING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PANEL_CONFIG0_SPEC, SEQUENCING_A, O>;
impl<'a, const O: u8> SEQUENCING_W<'a, O> {
    #[doc = "Hardware"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(SEQUENCING_A::HARDWARE)
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(SEQUENCING_A::SOFTWARE)
    }
}
impl R {
    #[doc = "Bit 0 - Data Enable"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Enable Polarity"]
    #[inline(always)]
    pub fn de_polarity(&self) -> DE_POLARITY_R {
        DE_POLARITY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Polarity"]
    #[inline(always)]
    pub fn data_polarity(&self) -> DATA_POLARITY_R {
        DATA_POLARITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock Enable/Disable"]
    #[inline(always)]
    pub fn clock(&self) -> CLOCK_R {
        CLOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Polarity"]
    #[inline(always)]
    pub fn clock_polarity(&self) -> CLOCK_POLARITY_R {
        CLOCK_POLARITY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable software or hardware panel sequencing."]
    #[inline(always)]
    pub fn sequencing(&self) -> SEQUENCING_R {
        SEQUENCING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DE_W<0> {
        DE_W::new(self)
    }
    #[doc = "Bit 1 - Data Enable Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn de_polarity(&mut self) -> DE_POLARITY_W<1> {
        DE_POLARITY_W::new(self)
    }
    #[doc = "Bit 5 - Data Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn data_polarity(&mut self) -> DATA_POLARITY_W<5> {
        DATA_POLARITY_W::new(self)
    }
    #[doc = "Bit 8 - Clock Enable/Disable"]
    #[inline(always)]
    #[must_use]
    pub fn clock(&mut self) -> CLOCK_W<8> {
        CLOCK_W::new(self)
    }
    #[doc = "Bit 9 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clock_polarity(&mut self) -> CLOCK_POLARITY_W<9> {
        CLOCK_POLARITY_W::new(self)
    }
    #[doc = "Bit 31 - Enable software or hardware panel sequencing."]
    #[inline(always)]
    #[must_use]
    pub fn sequencing(&mut self) -> SEQUENCING_W<31> {
        SEQUENCING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Panel Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [panel_config0](index.html) module"]
pub struct PANEL_CONFIG0_SPEC;
impl crate::RegisterSpec for PANEL_CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [panel_config0::R](R) reader structure"]
impl crate::Readable for PANEL_CONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [panel_config0::W](W) writer structure"]
impl crate::Writable for PANEL_CONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PanelConfig0 to value 0x8000_0000"]
impl crate::Resettable for PANEL_CONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
