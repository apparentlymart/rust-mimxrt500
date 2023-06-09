#[doc = "Register `ISEL` reader"]
pub struct R(crate::R<ISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISEL` writer"]
pub struct W(crate::W<ISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISEL_SPEC>;
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
impl From<crate::W<ISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMODE` reader - Interrupt mode"]
pub type PMODE_R = crate::FieldReader<u8, PMODE_A>;
#[doc = "Interrupt mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMODE_A {
    #[doc = "0: Edge-sensitive"]
    ISEL_0 = 0,
    #[doc = "1: Level-sensitive"]
    ISEL_1 = 1,
}
impl From<PMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as _
    }
}
impl PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMODE_A> {
        match self.bits {
            0 => Some(PMODE_A::ISEL_0),
            1 => Some(PMODE_A::ISEL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ISEL_0`"]
    #[inline(always)]
    pub fn is_isel_0(&self) -> bool {
        *self == PMODE_A::ISEL_0
    }
    #[doc = "Checks if the value of the field is `ISEL_1`"]
    #[inline(always)]
    pub fn is_isel_1(&self) -> bool {
        *self == PMODE_A::ISEL_1
    }
}
#[doc = "Field `PMODE` writer - Interrupt mode"]
pub type PMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISEL_SPEC, u8, PMODE_A, 8, O>;
impl<'a, const O: u8> PMODE_W<'a, O> {
    #[doc = "Edge-sensitive"]
    #[inline(always)]
    pub fn isel_0(self) -> &'a mut W {
        self.variant(PMODE_A::ISEL_0)
    }
    #[doc = "Level-sensitive"]
    #[inline(always)]
    pub fn isel_1(self) -> &'a mut W {
        self.variant(PMODE_A::ISEL_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt mode"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<0> {
        PMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isel](index.html) module"]
pub struct ISEL_SPEC;
impl crate::RegisterSpec for ISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isel::R](R) reader structure"]
impl crate::Readable for ISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isel::W](W) writer structure"]
impl crate::Writable for ISEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISEL to value 0"]
impl crate::Resettable for ISEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
