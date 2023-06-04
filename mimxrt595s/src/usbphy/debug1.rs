#[doc = "Register `DEBUG1` reader"]
pub struct R(crate::R<DEBUG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG1` writer"]
pub struct W(crate::W<DEBUG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG1_SPEC>;
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
impl From<crate::W<DEBUG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENTAILADJVD` reader - Enable delay increment"]
pub type ENTAILADJVD_R = crate::FieldReader<u8, ENTAILADJVD_A>;
#[doc = "Enable delay increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENTAILADJVD_A {
    #[doc = "0: Delay is nominal"]
    ENTAILADJVD_0 = 0,
    #[doc = "1: Delay is +20%"]
    ENTAILADJVD_1 = 1,
    #[doc = "2: Delay is -20%"]
    ENTAILADJVD_2 = 2,
    #[doc = "3: Delay is -40%"]
    ENTAILADJVD_3 = 3,
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
    pub fn variant(&self) -> ENTAILADJVD_A {
        match self.bits {
            0 => ENTAILADJVD_A::ENTAILADJVD_0,
            1 => ENTAILADJVD_A::ENTAILADJVD_1,
            2 => ENTAILADJVD_A::ENTAILADJVD_2,
            3 => ENTAILADJVD_A::ENTAILADJVD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENTAILADJVD_0`"]
    #[inline(always)]
    pub fn is_entailadjvd_0(&self) -> bool {
        *self == ENTAILADJVD_A::ENTAILADJVD_0
    }
    #[doc = "Checks if the value of the field is `ENTAILADJVD_1`"]
    #[inline(always)]
    pub fn is_entailadjvd_1(&self) -> bool {
        *self == ENTAILADJVD_A::ENTAILADJVD_1
    }
    #[doc = "Checks if the value of the field is `ENTAILADJVD_2`"]
    #[inline(always)]
    pub fn is_entailadjvd_2(&self) -> bool {
        *self == ENTAILADJVD_A::ENTAILADJVD_2
    }
    #[doc = "Checks if the value of the field is `ENTAILADJVD_3`"]
    #[inline(always)]
    pub fn is_entailadjvd_3(&self) -> bool {
        *self == ENTAILADJVD_A::ENTAILADJVD_3
    }
}
#[doc = "Field `ENTAILADJVD` writer - Enable delay increment"]
pub type ENTAILADJVD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DEBUG1_SPEC, u8, ENTAILADJVD_A, 2, O>;
impl<'a, const O: u8> ENTAILADJVD_W<'a, O> {
    #[doc = "Delay is nominal"]
    #[inline(always)]
    pub fn entailadjvd_0(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::ENTAILADJVD_0)
    }
    #[doc = "Delay is +20%"]
    #[inline(always)]
    pub fn entailadjvd_1(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::ENTAILADJVD_1)
    }
    #[doc = "Delay is -20%"]
    #[inline(always)]
    pub fn entailadjvd_2(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::ENTAILADJVD_2)
    }
    #[doc = "Delay is -40%"]
    #[inline(always)]
    pub fn entailadjvd_3(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::ENTAILADJVD_3)
    }
}
#[doc = "Field `USB2_REFBIAS_VBGADJ` reader - Bandgap adjustment"]
pub type USB2_REFBIAS_VBGADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB2_REFBIAS_VBGADJ` writer - Bandgap adjustment"]
pub type USB2_REFBIAS_VBGADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `USB2_REFBIAS_TST` reader - Bias current control"]
pub type USB2_REFBIAS_TST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB2_REFBIAS_TST` writer - Bias current control"]
pub type USB2_REFBIAS_TST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG1_SPEC, u8, u8, 2, O>;
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
#[doc = "UTMI Debug 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug1](index.html) module"]
pub struct DEBUG1_SPEC;
impl crate::RegisterSpec for DEBUG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug1::R](R) reader structure"]
impl crate::Readable for DEBUG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug1::W](W) writer structure"]
impl crate::Writable for DEBUG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG1 to value 0"]
impl crate::Resettable for DEBUG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
