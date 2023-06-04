#[doc = "Register `EVENTEN` reader"]
pub struct R(crate::R<EVENTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTEN` writer"]
pub struct W(crate::W<EVENTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTEN_SPEC>;
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
impl From<crate::W<EVENTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENT_OFLOW` reader - Event trigger on floating point overflow"]
pub type EVENT_OFLOW_R = crate::BitReader<EVENT_OFLOW_A>;
#[doc = "Event trigger on floating point overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_OFLOW_A {
    #[doc = "0: Disable event trigger"]
    DISABLE = 0,
    #[doc = "1: Enable event trigger"]
    ENABLE = 1,
}
impl From<EVENT_OFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: EVENT_OFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENT_OFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENT_OFLOW_A {
        match self.bits {
            false => EVENT_OFLOW_A::DISABLE,
            true => EVENT_OFLOW_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EVENT_OFLOW_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EVENT_OFLOW_A::ENABLE
    }
}
#[doc = "Field `EVENT_OFLOW` writer - Event trigger on floating point overflow"]
pub type EVENT_OFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTEN_SPEC, EVENT_OFLOW_A, O>;
impl<'a, const O: u8> EVENT_OFLOW_W<'a, O> {
    #[doc = "Disable event trigger"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EVENT_OFLOW_A::DISABLE)
    }
    #[doc = "Enable event trigger"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EVENT_OFLOW_A::ENABLE)
    }
}
#[doc = "Field `EVENT_NAN` reader - Event trigger on floating point NaN"]
pub type EVENT_NAN_R = crate::BitReader<EVENT_NAN_A>;
#[doc = "Event trigger on floating point NaN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_NAN_A {
    #[doc = "0: Disable event trigger"]
    DISABLE = 0,
    #[doc = "1: Enable event trigger"]
    ENABLE = 1,
}
impl From<EVENT_NAN_A> for bool {
    #[inline(always)]
    fn from(variant: EVENT_NAN_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENT_NAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENT_NAN_A {
        match self.bits {
            false => EVENT_NAN_A::DISABLE,
            true => EVENT_NAN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EVENT_NAN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EVENT_NAN_A::ENABLE
    }
}
#[doc = "Field `EVENT_NAN` writer - Event trigger on floating point NaN"]
pub type EVENT_NAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTEN_SPEC, EVENT_NAN_A, O>;
impl<'a, const O: u8> EVENT_NAN_W<'a, O> {
    #[doc = "Disable event trigger"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EVENT_NAN_A::DISABLE)
    }
    #[doc = "Enable event trigger"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EVENT_NAN_A::ENABLE)
    }
}
#[doc = "Field `EVENT_FIXED` reader - Event trigger on fixed-point overflow"]
pub type EVENT_FIXED_R = crate::BitReader<EVENT_FIXED_A>;
#[doc = "Event trigger on fixed-point overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_FIXED_A {
    #[doc = "0: Disable event trigger"]
    DISABLE = 0,
    #[doc = "1: Enable event trigger"]
    ENABLE = 1,
}
impl From<EVENT_FIXED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENT_FIXED_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENT_FIXED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENT_FIXED_A {
        match self.bits {
            false => EVENT_FIXED_A::DISABLE,
            true => EVENT_FIXED_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EVENT_FIXED_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EVENT_FIXED_A::ENABLE
    }
}
#[doc = "Field `EVENT_FIXED` writer - Event trigger on fixed-point overflow"]
pub type EVENT_FIXED_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTEN_SPEC, EVENT_FIXED_A, O>;
impl<'a, const O: u8> EVENT_FIXED_W<'a, O> {
    #[doc = "Disable event trigger"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EVENT_FIXED_A::DISABLE)
    }
    #[doc = "Enable event trigger"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EVENT_FIXED_A::ENABLE)
    }
}
#[doc = "Field `EVENT_UFLOW` reader - Event trigger on subnormal truncation"]
pub type EVENT_UFLOW_R = crate::BitReader<EVENT_UFLOW_A>;
#[doc = "Event trigger on subnormal truncation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_UFLOW_A {
    #[doc = "0: Disable event trigger"]
    DISABLE = 0,
    #[doc = "1: Enable event trigger"]
    ENABLE = 1,
}
impl From<EVENT_UFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: EVENT_UFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENT_UFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENT_UFLOW_A {
        match self.bits {
            false => EVENT_UFLOW_A::DISABLE,
            true => EVENT_UFLOW_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EVENT_UFLOW_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EVENT_UFLOW_A::ENABLE
    }
}
#[doc = "Field `EVENT_UFLOW` writer - Event trigger on subnormal truncation"]
pub type EVENT_UFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTEN_SPEC, EVENT_UFLOW_A, O>;
impl<'a, const O: u8> EVENT_UFLOW_W<'a, O> {
    #[doc = "Disable event trigger"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EVENT_UFLOW_A::DISABLE)
    }
    #[doc = "Enable event trigger"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EVENT_UFLOW_A::ENABLE)
    }
}
#[doc = "Field `EVENT_BERR` reader - Event trigger on AHBM bus error"]
pub type EVENT_BERR_R = crate::BitReader<EVENT_BERR_A>;
#[doc = "Event trigger on AHBM bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_BERR_A {
    #[doc = "0: Disable event trigger"]
    DISABLE = 0,
    #[doc = "1: Enable event trigger"]
    ENABLE = 1,
}
impl From<EVENT_BERR_A> for bool {
    #[inline(always)]
    fn from(variant: EVENT_BERR_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENT_BERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENT_BERR_A {
        match self.bits {
            false => EVENT_BERR_A::DISABLE,
            true => EVENT_BERR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EVENT_BERR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EVENT_BERR_A::ENABLE
    }
}
#[doc = "Field `EVENT_BERR` writer - Event trigger on AHBM bus error"]
pub type EVENT_BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTEN_SPEC, EVENT_BERR_A, O>;
impl<'a, const O: u8> EVENT_BERR_W<'a, O> {
    #[doc = "Disable event trigger"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EVENT_BERR_A::DISABLE)
    }
    #[doc = "Enable event trigger"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EVENT_BERR_A::ENABLE)
    }
}
#[doc = "Field `EVENT_COMP` reader - Event trigger on instruction completion"]
pub type EVENT_COMP_R = crate::BitReader<EVENT_COMP_A>;
#[doc = "Event trigger on instruction completion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_COMP_A {
    #[doc = "0: Disable event trigger"]
    DISABLE = 0,
    #[doc = "1: Enable event trigger"]
    ENABLE = 1,
}
impl From<EVENT_COMP_A> for bool {
    #[inline(always)]
    fn from(variant: EVENT_COMP_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENT_COMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENT_COMP_A {
        match self.bits {
            false => EVENT_COMP_A::DISABLE,
            true => EVENT_COMP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EVENT_COMP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EVENT_COMP_A::ENABLE
    }
}
#[doc = "Field `EVENT_COMP` writer - Event trigger on instruction completion"]
pub type EVENT_COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTEN_SPEC, EVENT_COMP_A, O>;
impl<'a, const O: u8> EVENT_COMP_W<'a, O> {
    #[doc = "Disable event trigger"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EVENT_COMP_A::DISABLE)
    }
    #[doc = "Enable event trigger"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EVENT_COMP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Event trigger on floating point overflow"]
    #[inline(always)]
    pub fn event_oflow(&self) -> EVENT_OFLOW_R {
        EVENT_OFLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event trigger on floating point NaN"]
    #[inline(always)]
    pub fn event_nan(&self) -> EVENT_NAN_R {
        EVENT_NAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event trigger on fixed-point overflow"]
    #[inline(always)]
    pub fn event_fixed(&self) -> EVENT_FIXED_R {
        EVENT_FIXED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event trigger on subnormal truncation"]
    #[inline(always)]
    pub fn event_uflow(&self) -> EVENT_UFLOW_R {
        EVENT_UFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event trigger on AHBM bus error"]
    #[inline(always)]
    pub fn event_berr(&self) -> EVENT_BERR_R {
        EVENT_BERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Event trigger on instruction completion"]
    #[inline(always)]
    pub fn event_comp(&self) -> EVENT_COMP_R {
        EVENT_COMP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event trigger on floating point overflow"]
    #[inline(always)]
    #[must_use]
    pub fn event_oflow(&mut self) -> EVENT_OFLOW_W<0> {
        EVENT_OFLOW_W::new(self)
    }
    #[doc = "Bit 1 - Event trigger on floating point NaN"]
    #[inline(always)]
    #[must_use]
    pub fn event_nan(&mut self) -> EVENT_NAN_W<1> {
        EVENT_NAN_W::new(self)
    }
    #[doc = "Bit 2 - Event trigger on fixed-point overflow"]
    #[inline(always)]
    #[must_use]
    pub fn event_fixed(&mut self) -> EVENT_FIXED_W<2> {
        EVENT_FIXED_W::new(self)
    }
    #[doc = "Bit 3 - Event trigger on subnormal truncation"]
    #[inline(always)]
    #[must_use]
    pub fn event_uflow(&mut self) -> EVENT_UFLOW_W<3> {
        EVENT_UFLOW_W::new(self)
    }
    #[doc = "Bit 4 - Event trigger on AHBM bus error"]
    #[inline(always)]
    #[must_use]
    pub fn event_berr(&mut self) -> EVENT_BERR_W<4> {
        EVENT_BERR_W::new(self)
    }
    #[doc = "Bit 7 - Event trigger on instruction completion"]
    #[inline(always)]
    #[must_use]
    pub fn event_comp(&mut self) -> EVENT_COMP_W<7> {
        EVENT_COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eventen](index.html) module"]
pub struct EVENTEN_SPEC;
impl crate::RegisterSpec for EVENTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eventen::R](R) reader structure"]
impl crate::Readable for EVENTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eventen::W](W) writer structure"]
impl crate::Writable for EVENTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTEN to value 0"]
impl crate::Resettable for EVENTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
