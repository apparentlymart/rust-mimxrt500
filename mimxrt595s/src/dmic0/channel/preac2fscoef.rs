#[doc = "Register `PREAC2FSCOEF` reader"]
pub struct R(crate::R<PREAC2FSCOEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREAC2FSCOEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREAC2FSCOEF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREAC2FSCOEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREAC2FSCOEF` writer"]
pub struct W(crate::W<PREAC2FSCOEF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREAC2FSCOEF_SPEC>;
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
impl From<crate::W<PREAC2FSCOEF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREAC2FSCOEF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - Compensation value"]
pub type COMP_R = crate::FieldReader<u8, COMP_A>;
#[doc = "Compensation value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP_A {
    #[doc = "0: Compensation = 0. This is the recommended setting."]
    COMP0 = 0,
    #[doc = "1: Compensation = -0.16"]
    COMP1 = 1,
    #[doc = "2: Compensation = -0.15"]
    COMP2 = 2,
    #[doc = "3: Compensation = -0.13"]
    COMP3 = 3,
}
impl From<COMP_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_A) -> Self {
        variant as _
    }
}
impl COMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_A {
        match self.bits {
            0 => COMP_A::COMP0,
            1 => COMP_A::COMP1,
            2 => COMP_A::COMP2,
            3 => COMP_A::COMP3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == COMP_A::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == COMP_A::COMP1
    }
    #[doc = "Checks if the value of the field is `COMP2`"]
    #[inline(always)]
    pub fn is_comp2(&self) -> bool {
        *self == COMP_A::COMP2
    }
    #[doc = "Checks if the value of the field is `COMP3`"]
    #[inline(always)]
    pub fn is_comp3(&self) -> bool {
        *self == COMP_A::COMP3
    }
}
#[doc = "Field `COMP` writer - Compensation value"]
pub type COMP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PREAC2FSCOEF_SPEC, u8, COMP_A, 2, O>;
impl<'a, const O: u8> COMP_W<'a, O> {
    #[doc = "Compensation = 0. This is the recommended setting."]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut W {
        self.variant(COMP_A::COMP0)
    }
    #[doc = "Compensation = -0.16"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(COMP_A::COMP1)
    }
    #[doc = "Compensation = -0.15"]
    #[inline(always)]
    pub fn comp2(self) -> &'a mut W {
        self.variant(COMP_A::COMP2)
    }
    #[doc = "Compensation = -0.13"]
    #[inline(always)]
    pub fn comp3(self) -> &'a mut W {
        self.variant(COMP_A::COMP3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Compensation value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Compensation value"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<0> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compensation Filter for 2 FS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preac2fscoef](index.html) module"]
pub struct PREAC2FSCOEF_SPEC;
impl crate::RegisterSpec for PREAC2FSCOEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [preac2fscoef::R](R) reader structure"]
impl crate::Readable for PREAC2FSCOEF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [preac2fscoef::W](W) writer structure"]
impl crate::Writable for PREAC2FSCOEF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PREAC2FSCOEF to value 0"]
impl crate::Resettable for PREAC2FSCOEF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
