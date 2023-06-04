#[doc = "Register `CHANEN` reader"]
pub struct R(crate::R<CHANEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANEN` writer"]
pub struct W(crate::W<CHANEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANEN_SPEC>;
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
impl From<crate::W<CHANEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_CH0` reader - Enable Channel n"]
pub type EN_CH0_R = crate::BitReader<EN_CH0_A>;
#[doc = "Enable Channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CH0_A {
    #[doc = "0: PDM channel n is disabled."]
    DISABLED = 0,
    #[doc = "1: PDM channel n is enabled."]
    ENABLED = 1,
}
impl From<EN_CH0_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CH0_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CH0_A {
        match self.bits {
            false => EN_CH0_A::DISABLED,
            true => EN_CH0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_CH0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_CH0_A::ENABLED
    }
}
#[doc = "Field `EN_CH0` writer - Enable Channel n"]
pub type EN_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, EN_CH0_A, O>;
impl<'a, const O: u8> EN_CH0_W<'a, O> {
    #[doc = "PDM channel n is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_CH0_A::DISABLED)
    }
    #[doc = "PDM channel n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_CH0_A::ENABLED)
    }
}
#[doc = "Field `EN_CH1` reader - Enable Channel n"]
pub type EN_CH1_R = crate::BitReader<EN_CH1_A>;
#[doc = "Enable Channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CH1_A {
    #[doc = "0: PDM channel n is disabled."]
    DISABLED = 0,
    #[doc = "1: PDM channel n is enabled."]
    ENABLED = 1,
}
impl From<EN_CH1_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CH1_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CH1_A {
        match self.bits {
            false => EN_CH1_A::DISABLED,
            true => EN_CH1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_CH1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_CH1_A::ENABLED
    }
}
#[doc = "Field `EN_CH1` writer - Enable Channel n"]
pub type EN_CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, EN_CH1_A, O>;
impl<'a, const O: u8> EN_CH1_W<'a, O> {
    #[doc = "PDM channel n is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_CH1_A::DISABLED)
    }
    #[doc = "PDM channel n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_CH1_A::ENABLED)
    }
}
#[doc = "Field `EN_CH2` reader - Enable Channel n"]
pub type EN_CH2_R = crate::BitReader<EN_CH2_A>;
#[doc = "Enable Channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CH2_A {
    #[doc = "0: PDM channel n is disabled."]
    DISABLED = 0,
    #[doc = "1: PDM channel n is enabled."]
    ENABLED = 1,
}
impl From<EN_CH2_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CH2_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CH2_A {
        match self.bits {
            false => EN_CH2_A::DISABLED,
            true => EN_CH2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_CH2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_CH2_A::ENABLED
    }
}
#[doc = "Field `EN_CH2` writer - Enable Channel n"]
pub type EN_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, EN_CH2_A, O>;
impl<'a, const O: u8> EN_CH2_W<'a, O> {
    #[doc = "PDM channel n is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_CH2_A::DISABLED)
    }
    #[doc = "PDM channel n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_CH2_A::ENABLED)
    }
}
#[doc = "Field `EN_CH3` reader - Enable Channel n"]
pub type EN_CH3_R = crate::BitReader<EN_CH3_A>;
#[doc = "Enable Channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CH3_A {
    #[doc = "0: PDM channel n is disabled."]
    DISABLED = 0,
    #[doc = "1: PDM channel n is enabled."]
    ENABLED = 1,
}
impl From<EN_CH3_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CH3_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CH3_A {
        match self.bits {
            false => EN_CH3_A::DISABLED,
            true => EN_CH3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_CH3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_CH3_A::ENABLED
    }
}
#[doc = "Field `EN_CH3` writer - Enable Channel n"]
pub type EN_CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, EN_CH3_A, O>;
impl<'a, const O: u8> EN_CH3_W<'a, O> {
    #[doc = "PDM channel n is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_CH3_A::DISABLED)
    }
    #[doc = "PDM channel n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_CH3_A::ENABLED)
    }
}
#[doc = "Field `EN_CH4` reader - Enable Channel n"]
pub type EN_CH4_R = crate::BitReader<EN_CH4_A>;
#[doc = "Enable Channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CH4_A {
    #[doc = "0: PDM channel n is disabled."]
    DISABLED = 0,
    #[doc = "1: PDM channel n is enabled."]
    ENABLED = 1,
}
impl From<EN_CH4_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CH4_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CH4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CH4_A {
        match self.bits {
            false => EN_CH4_A::DISABLED,
            true => EN_CH4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_CH4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_CH4_A::ENABLED
    }
}
#[doc = "Field `EN_CH4` writer - Enable Channel n"]
pub type EN_CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, EN_CH4_A, O>;
impl<'a, const O: u8> EN_CH4_W<'a, O> {
    #[doc = "PDM channel n is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_CH4_A::DISABLED)
    }
    #[doc = "PDM channel n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_CH4_A::ENABLED)
    }
}
#[doc = "Field `EN_CH5` reader - Enable Channel n"]
pub type EN_CH5_R = crate::BitReader<EN_CH5_A>;
#[doc = "Enable Channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CH5_A {
    #[doc = "0: PDM channel n is disabled."]
    DISABLED = 0,
    #[doc = "1: PDM channel n is enabled."]
    ENABLED = 1,
}
impl From<EN_CH5_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CH5_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CH5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CH5_A {
        match self.bits {
            false => EN_CH5_A::DISABLED,
            true => EN_CH5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_CH5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_CH5_A::ENABLED
    }
}
#[doc = "Field `EN_CH5` writer - Enable Channel n"]
pub type EN_CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, EN_CH5_A, O>;
impl<'a, const O: u8> EN_CH5_W<'a, O> {
    #[doc = "PDM channel n is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_CH5_A::DISABLED)
    }
    #[doc = "PDM channel n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_CH5_A::ENABLED)
    }
}
#[doc = "Field `EN_CH6` reader - Enable Channel n"]
pub type EN_CH6_R = crate::BitReader<EN_CH6_A>;
#[doc = "Enable Channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CH6_A {
    #[doc = "0: PDM channel n is disabled."]
    DISABLED = 0,
    #[doc = "1: PDM channel n is enabled."]
    ENABLED = 1,
}
impl From<EN_CH6_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CH6_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CH6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CH6_A {
        match self.bits {
            false => EN_CH6_A::DISABLED,
            true => EN_CH6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_CH6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_CH6_A::ENABLED
    }
}
#[doc = "Field `EN_CH6` writer - Enable Channel n"]
pub type EN_CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, EN_CH6_A, O>;
impl<'a, const O: u8> EN_CH6_W<'a, O> {
    #[doc = "PDM channel n is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_CH6_A::DISABLED)
    }
    #[doc = "PDM channel n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_CH6_A::ENABLED)
    }
}
#[doc = "Field `EN_CH7` reader - Enable Channel n"]
pub type EN_CH7_R = crate::BitReader<EN_CH7_A>;
#[doc = "Enable Channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CH7_A {
    #[doc = "0: PDM channel n is disabled."]
    DISABLED = 0,
    #[doc = "1: PDM channel n is enabled."]
    ENABLED = 1,
}
impl From<EN_CH7_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CH7_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CH7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CH7_A {
        match self.bits {
            false => EN_CH7_A::DISABLED,
            true => EN_CH7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_CH7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_CH7_A::ENABLED
    }
}
#[doc = "Field `EN_CH7` writer - Enable Channel n"]
pub type EN_CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, EN_CH7_A, O>;
impl<'a, const O: u8> EN_CH7_W<'a, O> {
    #[doc = "PDM channel n is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_CH7_A::DISABLED)
    }
    #[doc = "PDM channel n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_CH7_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Channel n"]
    #[inline(always)]
    pub fn en_ch0(&self) -> EN_CH0_R {
        EN_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Channel n"]
    #[inline(always)]
    pub fn en_ch1(&self) -> EN_CH1_R {
        EN_CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Channel n"]
    #[inline(always)]
    pub fn en_ch2(&self) -> EN_CH2_R {
        EN_CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Channel n"]
    #[inline(always)]
    pub fn en_ch3(&self) -> EN_CH3_R {
        EN_CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Channel n"]
    #[inline(always)]
    pub fn en_ch4(&self) -> EN_CH4_R {
        EN_CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Channel n"]
    #[inline(always)]
    pub fn en_ch5(&self) -> EN_CH5_R {
        EN_CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Channel n"]
    #[inline(always)]
    pub fn en_ch6(&self) -> EN_CH6_R {
        EN_CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Channel n"]
    #[inline(always)]
    pub fn en_ch7(&self) -> EN_CH7_R {
        EN_CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Channel n"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch0(&mut self) -> EN_CH0_W<0> {
        EN_CH0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Channel n"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch1(&mut self) -> EN_CH1_W<1> {
        EN_CH1_W::new(self)
    }
    #[doc = "Bit 2 - Enable Channel n"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch2(&mut self) -> EN_CH2_W<2> {
        EN_CH2_W::new(self)
    }
    #[doc = "Bit 3 - Enable Channel n"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch3(&mut self) -> EN_CH3_W<3> {
        EN_CH3_W::new(self)
    }
    #[doc = "Bit 4 - Enable Channel n"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch4(&mut self) -> EN_CH4_W<4> {
        EN_CH4_W::new(self)
    }
    #[doc = "Bit 5 - Enable Channel n"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch5(&mut self) -> EN_CH5_W<5> {
        EN_CH5_W::new(self)
    }
    #[doc = "Bit 6 - Enable Channel n"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch6(&mut self) -> EN_CH6_W<6> {
        EN_CH6_W::new(self)
    }
    #[doc = "Bit 7 - Enable Channel n"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch7(&mut self) -> EN_CH7_W<7> {
        EN_CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chanen](index.html) module"]
pub struct CHANEN_SPEC;
impl crate::RegisterSpec for CHANEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chanen::R](R) reader structure"]
impl crate::Readable for CHANEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chanen::W](W) writer structure"]
impl crate::Writable for CHANEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANEN to value 0"]
impl crate::Resettable for CHANEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
