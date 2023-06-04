#[doc = "Register `OTFADKEY_SRCSEL` reader"]
pub struct R(crate::R<OTFADKEY_SRCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTFADKEY_SRCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTFADKEY_SRCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTFADKEY_SRCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTFADKEY_SRCSEL` writer"]
pub struct W(crate::W<OTFADKEY_SRCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTFADKEY_SRCSEL_SPEC>;
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
impl From<crate::W<OTFADKEY_SRCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTFADKEY_SRCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTFADKEY_SRCSEL` reader - OTFAD Key Source Select"]
pub type OTFADKEY_SRCSEL_R = crate::FieldReader<u8, OTFADKEY_SRCSEL_A>;
#[doc = "OTFAD Key Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OTFADKEY_SRCSEL_A {
    #[doc = "0: PUF"]
    OTFADKEY_SRCSEL_0 = 0,
    #[doc = "1: PUF"]
    OTFADKEY_SRCSEL_1 = 1,
    #[doc = "2: OTP"]
    OTFADKEY_SRCSEL_2 = 2,
    #[doc = "3: PUF"]
    OTFADKEY_SRCSEL_3 = 3,
}
impl From<OTFADKEY_SRCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OTFADKEY_SRCSEL_A) -> Self {
        variant as _
    }
}
impl OTFADKEY_SRCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTFADKEY_SRCSEL_A {
        match self.bits {
            0 => OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_0,
            1 => OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_1,
            2 => OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_2,
            3 => OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OTFADKEY_SRCSEL_0`"]
    #[inline(always)]
    pub fn is_otfadkey_srcsel_0(&self) -> bool {
        *self == OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_0
    }
    #[doc = "Checks if the value of the field is `OTFADKEY_SRCSEL_1`"]
    #[inline(always)]
    pub fn is_otfadkey_srcsel_1(&self) -> bool {
        *self == OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_1
    }
    #[doc = "Checks if the value of the field is `OTFADKEY_SRCSEL_2`"]
    #[inline(always)]
    pub fn is_otfadkey_srcsel_2(&self) -> bool {
        *self == OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_2
    }
    #[doc = "Checks if the value of the field is `OTFADKEY_SRCSEL_3`"]
    #[inline(always)]
    pub fn is_otfadkey_srcsel_3(&self) -> bool {
        *self == OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_3
    }
}
#[doc = "Field `OTFADKEY_SRCSEL` writer - OTFAD Key Source Select"]
pub type OTFADKEY_SRCSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OTFADKEY_SRCSEL_SPEC, u8, OTFADKEY_SRCSEL_A, 2, O>;
impl<'a, const O: u8> OTFADKEY_SRCSEL_W<'a, O> {
    #[doc = "PUF"]
    #[inline(always)]
    pub fn otfadkey_srcsel_0(self) -> &'a mut W {
        self.variant(OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_0)
    }
    #[doc = "PUF"]
    #[inline(always)]
    pub fn otfadkey_srcsel_1(self) -> &'a mut W {
        self.variant(OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_1)
    }
    #[doc = "OTP"]
    #[inline(always)]
    pub fn otfadkey_srcsel_2(self) -> &'a mut W {
        self.variant(OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_2)
    }
    #[doc = "PUF"]
    #[inline(always)]
    pub fn otfadkey_srcsel_3(self) -> &'a mut W {
        self.variant(OTFADKEY_SRCSEL_A::OTFADKEY_SRCSEL_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - OTFAD Key Source Select"]
    #[inline(always)]
    pub fn otfadkey_srcsel(&self) -> OTFADKEY_SRCSEL_R {
        OTFADKEY_SRCSEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OTFAD Key Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn otfadkey_srcsel(&mut self) -> OTFADKEY_SRCSEL_W<0> {
        OTFADKEY_SRCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFAD Key Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otfadkey_srcsel](index.html) module"]
pub struct OTFADKEY_SRCSEL_SPEC;
impl crate::RegisterSpec for OTFADKEY_SRCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otfadkey_srcsel::R](R) reader structure"]
impl crate::Readable for OTFADKEY_SRCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otfadkey_srcsel::W](W) writer structure"]
impl crate::Writable for OTFADKEY_SRCSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTFADKEY_SRCSEL to value 0"]
impl crate::Resettable for OTFADKEY_SRCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
