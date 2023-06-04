#[doc = "Register `INTREN` reader"]
pub struct R(crate::R<INTREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTREN` writer"]
pub struct W(crate::W<INTREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTREN_SPEC>;
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
impl From<crate::W<INTREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_OFLOW` reader - Interrupt floating point overflow"]
pub type INTR_OFLOW_R = crate::BitReader<INTR_OFLOW_A>;
#[doc = "Interrupt floating point overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTR_OFLOW_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<INTR_OFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: INTR_OFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl INTR_OFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTR_OFLOW_A {
        match self.bits {
            false => INTR_OFLOW_A::DISABLE,
            true => INTR_OFLOW_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTR_OFLOW_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTR_OFLOW_A::ENABLE
    }
}
#[doc = "Field `INTR_OFLOW` writer - Interrupt floating point overflow"]
pub type INTR_OFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, INTR_OFLOW_A, O>;
impl<'a, const O: u8> INTR_OFLOW_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTR_OFLOW_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTR_OFLOW_A::ENABLE)
    }
}
#[doc = "Field `INTR_NAN` reader - Interrupt floating point NaN"]
pub type INTR_NAN_R = crate::BitReader<INTR_NAN_A>;
#[doc = "Interrupt floating point NaN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTR_NAN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<INTR_NAN_A> for bool {
    #[inline(always)]
    fn from(variant: INTR_NAN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTR_NAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTR_NAN_A {
        match self.bits {
            false => INTR_NAN_A::DISABLE,
            true => INTR_NAN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTR_NAN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTR_NAN_A::ENABLE
    }
}
#[doc = "Field `INTR_NAN` writer - Interrupt floating point NaN"]
pub type INTR_NAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, INTR_NAN_A, O>;
impl<'a, const O: u8> INTR_NAN_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTR_NAN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTR_NAN_A::ENABLE)
    }
}
#[doc = "Field `INTR_FIXED` reader - Interrupt on fixed-point overflow"]
pub type INTR_FIXED_R = crate::BitReader<INTR_FIXED_A>;
#[doc = "Interrupt on fixed-point overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTR_FIXED_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<INTR_FIXED_A> for bool {
    #[inline(always)]
    fn from(variant: INTR_FIXED_A) -> Self {
        variant as u8 != 0
    }
}
impl INTR_FIXED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTR_FIXED_A {
        match self.bits {
            false => INTR_FIXED_A::DISABLE,
            true => INTR_FIXED_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTR_FIXED_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTR_FIXED_A::ENABLE
    }
}
#[doc = "Field `INTR_FIXED` writer - Interrupt on fixed-point overflow"]
pub type INTR_FIXED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, INTR_FIXED_A, O>;
impl<'a, const O: u8> INTR_FIXED_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTR_FIXED_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTR_FIXED_A::ENABLE)
    }
}
#[doc = "Field `INTR_UFLOW` reader - Interrupt on subnormal truncation"]
pub type INTR_UFLOW_R = crate::BitReader<INTR_UFLOW_A>;
#[doc = "Interrupt on subnormal truncation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTR_UFLOW_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<INTR_UFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: INTR_UFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl INTR_UFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTR_UFLOW_A {
        match self.bits {
            false => INTR_UFLOW_A::DISABLE,
            true => INTR_UFLOW_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTR_UFLOW_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTR_UFLOW_A::ENABLE
    }
}
#[doc = "Field `INTR_UFLOW` writer - Interrupt on subnormal truncation"]
pub type INTR_UFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, INTR_UFLOW_A, O>;
impl<'a, const O: u8> INTR_UFLOW_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTR_UFLOW_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTR_UFLOW_A::ENABLE)
    }
}
#[doc = "Field `INTR_BERR` reader - Interrupt on AHBM bus error"]
pub type INTR_BERR_R = crate::BitReader<INTR_BERR_A>;
#[doc = "Interrupt on AHBM bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTR_BERR_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<INTR_BERR_A> for bool {
    #[inline(always)]
    fn from(variant: INTR_BERR_A) -> Self {
        variant as u8 != 0
    }
}
impl INTR_BERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTR_BERR_A {
        match self.bits {
            false => INTR_BERR_A::DISABLE,
            true => INTR_BERR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTR_BERR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTR_BERR_A::ENABLE
    }
}
#[doc = "Field `INTR_BERR` writer - Interrupt on AHBM bus error"]
pub type INTR_BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, INTR_BERR_A, O>;
impl<'a, const O: u8> INTR_BERR_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTR_BERR_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTR_BERR_A::ENABLE)
    }
}
#[doc = "Field `INTR_COMP` reader - Interrupt on instruction completion"]
pub type INTR_COMP_R = crate::BitReader<INTR_COMP_A>;
#[doc = "Interrupt on instruction completion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTR_COMP_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<INTR_COMP_A> for bool {
    #[inline(always)]
    fn from(variant: INTR_COMP_A) -> Self {
        variant as u8 != 0
    }
}
impl INTR_COMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTR_COMP_A {
        match self.bits {
            false => INTR_COMP_A::DISABLE,
            true => INTR_COMP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTR_COMP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTR_COMP_A::ENABLE
    }
}
#[doc = "Field `INTR_COMP` writer - Interrupt on instruction completion"]
pub type INTR_COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, INTR_COMP_A, O>;
impl<'a, const O: u8> INTR_COMP_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTR_COMP_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTR_COMP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt floating point overflow"]
    #[inline(always)]
    pub fn intr_oflow(&self) -> INTR_OFLOW_R {
        INTR_OFLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt floating point NaN"]
    #[inline(always)]
    pub fn intr_nan(&self) -> INTR_NAN_R {
        INTR_NAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on fixed-point overflow"]
    #[inline(always)]
    pub fn intr_fixed(&self) -> INTR_FIXED_R {
        INTR_FIXED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on subnormal truncation"]
    #[inline(always)]
    pub fn intr_uflow(&self) -> INTR_UFLOW_R {
        INTR_UFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt on AHBM bus error"]
    #[inline(always)]
    pub fn intr_berr(&self) -> INTR_BERR_R {
        INTR_BERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt on instruction completion"]
    #[inline(always)]
    pub fn intr_comp(&self) -> INTR_COMP_R {
        INTR_COMP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt floating point overflow"]
    #[inline(always)]
    #[must_use]
    pub fn intr_oflow(&mut self) -> INTR_OFLOW_W<0> {
        INTR_OFLOW_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt floating point NaN"]
    #[inline(always)]
    #[must_use]
    pub fn intr_nan(&mut self) -> INTR_NAN_W<1> {
        INTR_NAN_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt on fixed-point overflow"]
    #[inline(always)]
    #[must_use]
    pub fn intr_fixed(&mut self) -> INTR_FIXED_W<2> {
        INTR_FIXED_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt on subnormal truncation"]
    #[inline(always)]
    #[must_use]
    pub fn intr_uflow(&mut self) -> INTR_UFLOW_W<3> {
        INTR_UFLOW_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt on AHBM bus error"]
    #[inline(always)]
    #[must_use]
    pub fn intr_berr(&mut self) -> INTR_BERR_W<4> {
        INTR_BERR_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt on instruction completion"]
    #[inline(always)]
    #[must_use]
    pub fn intr_comp(&mut self) -> INTR_COMP_W<7> {
        INTR_COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intren](index.html) module"]
pub struct INTREN_SPEC;
impl crate::RegisterSpec for INTREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intren::R](R) reader structure"]
impl crate::Readable for INTREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intren::W](W) writer structure"]
impl crate::Writable for INTREN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTREN to value 0"]
impl crate::Resettable for INTREN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
