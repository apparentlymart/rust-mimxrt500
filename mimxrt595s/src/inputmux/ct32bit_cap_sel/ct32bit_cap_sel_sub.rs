#[doc = "Register `CT32BIT_CAP_SEL_SUB[%s]` reader"]
pub struct R(crate::R<CT32BIT_CAP_SEL_SUB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CT32BIT_CAP_SEL_SUB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CT32BIT_CAP_SEL_SUB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CT32BIT_CAP_SEL_SUB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CT32BIT_CAP_SEL_SUB[%s]` writer"]
pub struct W(crate::W<CT32BIT_CAP_SEL_SUB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CT32BIT_CAP_SEL_SUB_SPEC>;
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
impl From<crate::W<CT32BIT_CAP_SEL_SUB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CT32BIT_CAP_SEL_SUB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPn_SEL` reader - Counter Timer n, Capture Input m"]
pub type CAPN_SEL_R = crate::FieldReader<u8, CAPN_SEL_A>;
#[doc = "Counter Timer n, Capture Input m\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPN_SEL_A {
    #[doc = "0: CT_INP0 (function must be selected in IOPCTL)"]
    CT_INP0 = 0,
    #[doc = "1: CT_INP1 (function must be selected in IOPCTL)"]
    CT_INP1 = 1,
    #[doc = "2: CT_INP2 (function must be selected in IOPCTL)"]
    CT_INP2 = 2,
    #[doc = "3: CT_INP3 (function must be selected in IOPCTL)"]
    CT_INP3 = 3,
    #[doc = "4: CT_INP4 (function must be selected in IOPCTL)"]
    CT_INP4 = 4,
    #[doc = "5: CT_INP5 (function must be selected in IOPCTL)"]
    CT_INP5 = 5,
    #[doc = "6: CT_INP6 (function must be selected in IOPCTL)"]
    CT_INP6 = 6,
    #[doc = "7: CT_INP7 (function must be selected in IOPCTL)"]
    CT_INP7 = 7,
    #[doc = "8: CT_INP8 (function must be selected in IOPCTL)"]
    CT_INP8 = 8,
    #[doc = "9: CT_INP9 (function must be selected in IOPCTL)"]
    CT_INP9 = 9,
    #[doc = "10: CT_INP10 (function must be selected in IOPCTL)"]
    CT_INP10 = 10,
    #[doc = "11: CT_INP11 (function must be selected in IOPCTL)"]
    CT_INP11 = 11,
    #[doc = "12: CT_INP12 (function must be selected in IOPCTL)"]
    CT_INP12 = 12,
    #[doc = "13: CT_INP13 (function must be selected in IOPCTL)"]
    CT_INP13 = 13,
    #[doc = "14: CT_INP14 (function must be selected in IOPCTL)"]
    CT_INP14 = 14,
    #[doc = "15: CT_INP15 (function must be selected in IOPCTL)"]
    CT_INP15 = 15,
    #[doc = "16: SHARED I2S0_WS"]
    CT_INP16 = 16,
    #[doc = "17: SHARED I2S1_WS"]
    CT_INP17 = 17,
    #[doc = "18: USB1_FRAME_TOGGLE (see USB Controller Chapter)"]
    CT_INP18 = 18,
}
impl From<CAPN_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPN_SEL_A) -> Self {
        variant as _
    }
}
impl CAPN_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPN_SEL_A> {
        match self.bits {
            0 => Some(CAPN_SEL_A::CT_INP0),
            1 => Some(CAPN_SEL_A::CT_INP1),
            2 => Some(CAPN_SEL_A::CT_INP2),
            3 => Some(CAPN_SEL_A::CT_INP3),
            4 => Some(CAPN_SEL_A::CT_INP4),
            5 => Some(CAPN_SEL_A::CT_INP5),
            6 => Some(CAPN_SEL_A::CT_INP6),
            7 => Some(CAPN_SEL_A::CT_INP7),
            8 => Some(CAPN_SEL_A::CT_INP8),
            9 => Some(CAPN_SEL_A::CT_INP9),
            10 => Some(CAPN_SEL_A::CT_INP10),
            11 => Some(CAPN_SEL_A::CT_INP11),
            12 => Some(CAPN_SEL_A::CT_INP12),
            13 => Some(CAPN_SEL_A::CT_INP13),
            14 => Some(CAPN_SEL_A::CT_INP14),
            15 => Some(CAPN_SEL_A::CT_INP15),
            16 => Some(CAPN_SEL_A::CT_INP16),
            17 => Some(CAPN_SEL_A::CT_INP17),
            18 => Some(CAPN_SEL_A::CT_INP18),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CT_INP0`"]
    #[inline(always)]
    pub fn is_ct_inp0(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP0
    }
    #[doc = "Checks if the value of the field is `CT_INP1`"]
    #[inline(always)]
    pub fn is_ct_inp1(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP1
    }
    #[doc = "Checks if the value of the field is `CT_INP2`"]
    #[inline(always)]
    pub fn is_ct_inp2(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP2
    }
    #[doc = "Checks if the value of the field is `CT_INP3`"]
    #[inline(always)]
    pub fn is_ct_inp3(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP3
    }
    #[doc = "Checks if the value of the field is `CT_INP4`"]
    #[inline(always)]
    pub fn is_ct_inp4(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP4
    }
    #[doc = "Checks if the value of the field is `CT_INP5`"]
    #[inline(always)]
    pub fn is_ct_inp5(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP5
    }
    #[doc = "Checks if the value of the field is `CT_INP6`"]
    #[inline(always)]
    pub fn is_ct_inp6(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP6
    }
    #[doc = "Checks if the value of the field is `CT_INP7`"]
    #[inline(always)]
    pub fn is_ct_inp7(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP7
    }
    #[doc = "Checks if the value of the field is `CT_INP8`"]
    #[inline(always)]
    pub fn is_ct_inp8(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP8
    }
    #[doc = "Checks if the value of the field is `CT_INP9`"]
    #[inline(always)]
    pub fn is_ct_inp9(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP9
    }
    #[doc = "Checks if the value of the field is `CT_INP10`"]
    #[inline(always)]
    pub fn is_ct_inp10(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP10
    }
    #[doc = "Checks if the value of the field is `CT_INP11`"]
    #[inline(always)]
    pub fn is_ct_inp11(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP11
    }
    #[doc = "Checks if the value of the field is `CT_INP12`"]
    #[inline(always)]
    pub fn is_ct_inp12(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP12
    }
    #[doc = "Checks if the value of the field is `CT_INP13`"]
    #[inline(always)]
    pub fn is_ct_inp13(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP13
    }
    #[doc = "Checks if the value of the field is `CT_INP14`"]
    #[inline(always)]
    pub fn is_ct_inp14(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP14
    }
    #[doc = "Checks if the value of the field is `CT_INP15`"]
    #[inline(always)]
    pub fn is_ct_inp15(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP15
    }
    #[doc = "Checks if the value of the field is `CT_INP16`"]
    #[inline(always)]
    pub fn is_ct_inp16(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP16
    }
    #[doc = "Checks if the value of the field is `CT_INP17`"]
    #[inline(always)]
    pub fn is_ct_inp17(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP17
    }
    #[doc = "Checks if the value of the field is `CT_INP18`"]
    #[inline(always)]
    pub fn is_ct_inp18(&self) -> bool {
        *self == CAPN_SEL_A::CT_INP18
    }
}
#[doc = "Field `CAPn_SEL` writer - Counter Timer n, Capture Input m"]
pub type CAPN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CT32BIT_CAP_SEL_SUB_SPEC, u8, CAPN_SEL_A, 5, O>;
impl<'a, const O: u8> CAPN_SEL_W<'a, O> {
    #[doc = "CT_INP0 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp0(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP0)
    }
    #[doc = "CT_INP1 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp1(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP1)
    }
    #[doc = "CT_INP2 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp2(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP2)
    }
    #[doc = "CT_INP3 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp3(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP3)
    }
    #[doc = "CT_INP4 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp4(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP4)
    }
    #[doc = "CT_INP5 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp5(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP5)
    }
    #[doc = "CT_INP6 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp6(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP6)
    }
    #[doc = "CT_INP7 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp7(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP7)
    }
    #[doc = "CT_INP8 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp8(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP8)
    }
    #[doc = "CT_INP9 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp9(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP9)
    }
    #[doc = "CT_INP10 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp10(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP10)
    }
    #[doc = "CT_INP11 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp11(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP11)
    }
    #[doc = "CT_INP12 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp12(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP12)
    }
    #[doc = "CT_INP13 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp13(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP13)
    }
    #[doc = "CT_INP14 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp14(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP14)
    }
    #[doc = "CT_INP15 (function must be selected in IOPCTL)"]
    #[inline(always)]
    pub fn ct_inp15(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP15)
    }
    #[doc = "SHARED I2S0_WS"]
    #[inline(always)]
    pub fn ct_inp16(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP16)
    }
    #[doc = "SHARED I2S1_WS"]
    #[inline(always)]
    pub fn ct_inp17(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP17)
    }
    #[doc = "USB1_FRAME_TOGGLE (see USB Controller Chapter)"]
    #[inline(always)]
    pub fn ct_inp18(self) -> &'a mut W {
        self.variant(CAPN_SEL_A::CT_INP18)
    }
}
impl R {
    #[doc = "Bits 0:4 - Counter Timer n, Capture Input m"]
    #[inline(always)]
    pub fn capn_sel(&self) -> CAPN_SEL_R {
        CAPN_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Counter Timer n, Capture Input m"]
    #[inline(always)]
    #[must_use]
    pub fn capn_sel(&mut self) -> CAPN_SEL_W<0> {
        CAPN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CT32BIT Timer Capture Multiplexers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ct32bit_cap_sel_sub](index.html) module"]
pub struct CT32BIT_CAP_SEL_SUB_SPEC;
impl crate::RegisterSpec for CT32BIT_CAP_SEL_SUB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ct32bit_cap_sel_sub::R](R) reader structure"]
impl crate::Readable for CT32BIT_CAP_SEL_SUB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ct32bit_cap_sel_sub::W](W) writer structure"]
impl crate::Writable for CT32BIT_CAP_SEL_SUB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CT32BIT_CAP_SEL_SUB[%s]
to value 0x1f"]
impl crate::Resettable for CT32BIT_CAP_SEL_SUB_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
