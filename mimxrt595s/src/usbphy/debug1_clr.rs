#[doc = "Register `DEBUG1_CLR` reader"]
pub struct R(crate::R<DEBUG1_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG1_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG1_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG1_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG1_CLR` writer"]
pub struct W(crate::W<DEBUG1_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG1_CLR_SPEC>;
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
impl From<crate::W<DEBUG1_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG1_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENTAILADJVD` reader - Enable delay increment"]
pub type ENTAILADJVD_R = crate::FieldReader<u8, ENTAILADJVD_A>;
#[doc = "Enable delay increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENTAILADJVD_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding DEBUG1 bit"]
    ENABLE = 1,
}
impl From<ENTAILADJVD_A> for u8 {
    #[inline(always)]
    fn from(variant: ENTAILADJVD_A) -> Self {
        variant as _
    }
}
impl ENTAILADJVD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENTAILADJVD_A> {
        match self.bits {
            0 => Some(ENTAILADJVD_A::DISABLE),
            1 => Some(ENTAILADJVD_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENTAILADJVD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENTAILADJVD_A::ENABLE
    }
}
#[doc = "Field `ENTAILADJVD` writer - Enable delay increment"]
pub type ENTAILADJVD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG1_CLR_SPEC, u8, ENTAILADJVD_A, 2, O>;
impl<'a, const O: u8> ENTAILADJVD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::DISABLE)
    }
    #[doc = "Clears the corresponding DEBUG1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::ENABLE)
    }
}
#[doc = "Field `USB2_REFBIAS_VBGADJ` reader - Bandgap adjustment"]
pub type USB2_REFBIAS_VBGADJ_R = crate::FieldReader<u8, USB2_REFBIAS_VBGADJ_A>;
#[doc = "Bandgap adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB2_REFBIAS_VBGADJ_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding DEBUG1 bit"]
    ENABLE = 1,
}
impl From<USB2_REFBIAS_VBGADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: USB2_REFBIAS_VBGADJ_A) -> Self {
        variant as _
    }
}
impl USB2_REFBIAS_VBGADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USB2_REFBIAS_VBGADJ_A> {
        match self.bits {
            0 => Some(USB2_REFBIAS_VBGADJ_A::DISABLE),
            1 => Some(USB2_REFBIAS_VBGADJ_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB2_REFBIAS_VBGADJ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB2_REFBIAS_VBGADJ_A::ENABLE
    }
}
#[doc = "Field `USB2_REFBIAS_VBGADJ` writer - Bandgap adjustment"]
pub type USB2_REFBIAS_VBGADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG1_CLR_SPEC, u8, USB2_REFBIAS_VBGADJ_A, 3, O>;
impl<'a, const O: u8> USB2_REFBIAS_VBGADJ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB2_REFBIAS_VBGADJ_A::DISABLE)
    }
    #[doc = "Clears the corresponding DEBUG1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB2_REFBIAS_VBGADJ_A::ENABLE)
    }
}
#[doc = "Field `USB2_REFBIAS_TST` reader - Bias current control"]
pub type USB2_REFBIAS_TST_R = crate::FieldReader<u8, USB2_REFBIAS_TST_A>;
#[doc = "Bias current control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB2_REFBIAS_TST_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding DEBUG1 bit"]
    ENABLE = 1,
}
impl From<USB2_REFBIAS_TST_A> for u8 {
    #[inline(always)]
    fn from(variant: USB2_REFBIAS_TST_A) -> Self {
        variant as _
    }
}
impl USB2_REFBIAS_TST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USB2_REFBIAS_TST_A> {
        match self.bits {
            0 => Some(USB2_REFBIAS_TST_A::DISABLE),
            1 => Some(USB2_REFBIAS_TST_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB2_REFBIAS_TST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB2_REFBIAS_TST_A::ENABLE
    }
}
#[doc = "Field `USB2_REFBIAS_TST` writer - Bias current control"]
pub type USB2_REFBIAS_TST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG1_CLR_SPEC, u8, USB2_REFBIAS_TST_A, 2, O>;
impl<'a, const O: u8> USB2_REFBIAS_TST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB2_REFBIAS_TST_A::DISABLE)
    }
    #[doc = "Clears the corresponding DEBUG1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB2_REFBIAS_TST_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 13:14 - Enable delay increment"]
    #[inline(always)]
    pub fn entailadjvd(&self) -> ENTAILADJVD_R {
        ENTAILADJVD_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Bandgap adjustment"]
    #[inline(always)]
    pub fn usb2_refbias_vbgadj(&self) -> USB2_REFBIAS_VBGADJ_R {
        USB2_REFBIAS_VBGADJ_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Bias current control"]
    #[inline(always)]
    pub fn usb2_refbias_tst(&self) -> USB2_REFBIAS_TST_R {
        USB2_REFBIAS_TST_R::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 13:14 - Enable delay increment"]
    #[inline(always)]
    #[must_use]
    pub fn entailadjvd(&mut self) -> ENTAILADJVD_W<13> {
        ENTAILADJVD_W::new(self)
    }
    #[doc = "Bits 18:20 - Bandgap adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn usb2_refbias_vbgadj(&mut self) -> USB2_REFBIAS_VBGADJ_W<18> {
        USB2_REFBIAS_VBGADJ_W::new(self)
    }
    #[doc = "Bits 21:22 - Bias current control"]
    #[inline(always)]
    #[must_use]
    pub fn usb2_refbias_tst(&mut self) -> USB2_REFBIAS_TST_W<21> {
        USB2_REFBIAS_TST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UTMI Debug 1 Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug1_clr](index.html) module"]
pub struct DEBUG1_CLR_SPEC;
impl crate::RegisterSpec for DEBUG1_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug1_clr::R](R) reader structure"]
impl crate::Readable for DEBUG1_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug1_clr::W](W) writer structure"]
impl crate::Writable for DEBUG1_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG1_CLR to value 0"]
impl crate::Resettable for DEBUG1_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
