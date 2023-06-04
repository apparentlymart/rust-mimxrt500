#[doc = "Register `SCT0_IN_SEL[%s]` reader"]
pub struct R(crate::R<SCT0_IN_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCT0_IN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCT0_IN_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCT0_IN_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCT0_IN_SEL[%s]` writer"]
pub struct W(crate::W<SCT0_IN_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCT0_IN_SEL_SPEC>;
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
impl From<crate::W<SCT0_IN_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCT0_IN_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCT_IN_SEL` reader - SCT0 Input Selection."]
pub type SCT_IN_SEL_R = crate::FieldReader<u8, SCT_IN_SEL_A>;
#[doc = "SCT0 Input Selection.\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCT_IN_SEL_A {
    #[doc = "0: SCT0_PIN_INP0"]
    SCT_IN_SEL_0 = 0,
    #[doc = "1: SCT0_PIN_INP1"]
    SCT_IN_SEL_1 = 1,
    #[doc = "2: SCT0_PIN_INP2"]
    SCT_IN_SEL_2 = 2,
    #[doc = "3: SCT0_PIN_INP3"]
    SCT_IN_SEL_3 = 3,
    #[doc = "4: SCT0_PIN_INP4"]
    SCT_IN_SEL_4 = 4,
    #[doc = "5: SCT0_PIN_INP5"]
    SCT_IN_SEL_5 = 5,
    #[doc = "6: SCT0_PIN_INP6"]
    SCT_IN_SEL_6 = 6,
    #[doc = "7: SCT0_PIN_INP7"]
    SCT_IN_SEL_7 = 7,
    #[doc = "8: CT32BIT0_MAT0"]
    SCT_IN_SEL_8 = 8,
    #[doc = "9: CT32BIT1_MAT0"]
    SCT_IN_SEL_9 = 9,
    #[doc = "10: CT32BIT2_MAT0"]
    SCT_IN_SEL_10 = 10,
    #[doc = "11: CT32BIT3_MAT0"]
    SCT_IN_SEL_11 = 11,
    #[doc = "12: CT32BIT4_MAT0"]
    SCT_IN_SEL_12 = 12,
    #[doc = "13: ADCIRQ"]
    SCT_IN_SEL_13 = 13,
    #[doc = "14: GPIOINT_BMATCH"]
    SCT_IN_SEL_14 = 14,
    #[doc = "15: USB0_FRAME_TOGGLE"]
    SCT_IN_SEL_15 = 15,
    #[doc = "16: CMP0_OUT"]
    SCT_IN_SEL_16 = 16,
    #[doc = "17: SHARED I2S0_SCLK"]
    SCT_IN_SEL_17 = 17,
    #[doc = "18: SHARED I2S1_SCLK"]
    SCT_IN_SEL_18 = 18,
    #[doc = "19: SHARED I2S0_WS"]
    SCT_IN_SEL_19 = 19,
    #[doc = "20: SHARED I2S1_WS"]
    SCT_IN_SEL_20 = 20,
    #[doc = "21: MCLK"]
    SCT_IN_SEL_21 = 21,
    #[doc = "22: ARM_TXEV"]
    SCT_IN_SEL_22 = 22,
    #[doc = "23: DEBUG_HALTED"]
    SCT_IN_SEL_23 = 23,
}
impl From<SCT_IN_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCT_IN_SEL_A) -> Self {
        variant as _
    }
}
impl SCT_IN_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCT_IN_SEL_A> {
        match self.bits {
            0 => Some(SCT_IN_SEL_A::SCT_IN_SEL_0),
            1 => Some(SCT_IN_SEL_A::SCT_IN_SEL_1),
            2 => Some(SCT_IN_SEL_A::SCT_IN_SEL_2),
            3 => Some(SCT_IN_SEL_A::SCT_IN_SEL_3),
            4 => Some(SCT_IN_SEL_A::SCT_IN_SEL_4),
            5 => Some(SCT_IN_SEL_A::SCT_IN_SEL_5),
            6 => Some(SCT_IN_SEL_A::SCT_IN_SEL_6),
            7 => Some(SCT_IN_SEL_A::SCT_IN_SEL_7),
            8 => Some(SCT_IN_SEL_A::SCT_IN_SEL_8),
            9 => Some(SCT_IN_SEL_A::SCT_IN_SEL_9),
            10 => Some(SCT_IN_SEL_A::SCT_IN_SEL_10),
            11 => Some(SCT_IN_SEL_A::SCT_IN_SEL_11),
            12 => Some(SCT_IN_SEL_A::SCT_IN_SEL_12),
            13 => Some(SCT_IN_SEL_A::SCT_IN_SEL_13),
            14 => Some(SCT_IN_SEL_A::SCT_IN_SEL_14),
            15 => Some(SCT_IN_SEL_A::SCT_IN_SEL_15),
            16 => Some(SCT_IN_SEL_A::SCT_IN_SEL_16),
            17 => Some(SCT_IN_SEL_A::SCT_IN_SEL_17),
            18 => Some(SCT_IN_SEL_A::SCT_IN_SEL_18),
            19 => Some(SCT_IN_SEL_A::SCT_IN_SEL_19),
            20 => Some(SCT_IN_SEL_A::SCT_IN_SEL_20),
            21 => Some(SCT_IN_SEL_A::SCT_IN_SEL_21),
            22 => Some(SCT_IN_SEL_A::SCT_IN_SEL_22),
            23 => Some(SCT_IN_SEL_A::SCT_IN_SEL_23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_0`"]
    #[inline(always)]
    pub fn is_sct_in_sel_0(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_0
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_1`"]
    #[inline(always)]
    pub fn is_sct_in_sel_1(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_1
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_2`"]
    #[inline(always)]
    pub fn is_sct_in_sel_2(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_2
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_3`"]
    #[inline(always)]
    pub fn is_sct_in_sel_3(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_3
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_4`"]
    #[inline(always)]
    pub fn is_sct_in_sel_4(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_4
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_5`"]
    #[inline(always)]
    pub fn is_sct_in_sel_5(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_5
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_6`"]
    #[inline(always)]
    pub fn is_sct_in_sel_6(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_6
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_7`"]
    #[inline(always)]
    pub fn is_sct_in_sel_7(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_7
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_8`"]
    #[inline(always)]
    pub fn is_sct_in_sel_8(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_8
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_9`"]
    #[inline(always)]
    pub fn is_sct_in_sel_9(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_9
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_10`"]
    #[inline(always)]
    pub fn is_sct_in_sel_10(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_10
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_11`"]
    #[inline(always)]
    pub fn is_sct_in_sel_11(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_11
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_12`"]
    #[inline(always)]
    pub fn is_sct_in_sel_12(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_12
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_13`"]
    #[inline(always)]
    pub fn is_sct_in_sel_13(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_13
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_14`"]
    #[inline(always)]
    pub fn is_sct_in_sel_14(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_14
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_15`"]
    #[inline(always)]
    pub fn is_sct_in_sel_15(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_15
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_16`"]
    #[inline(always)]
    pub fn is_sct_in_sel_16(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_16
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_17`"]
    #[inline(always)]
    pub fn is_sct_in_sel_17(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_17
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_18`"]
    #[inline(always)]
    pub fn is_sct_in_sel_18(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_18
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_19`"]
    #[inline(always)]
    pub fn is_sct_in_sel_19(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_19
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_20`"]
    #[inline(always)]
    pub fn is_sct_in_sel_20(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_20
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_21`"]
    #[inline(always)]
    pub fn is_sct_in_sel_21(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_21
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_22`"]
    #[inline(always)]
    pub fn is_sct_in_sel_22(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_22
    }
    #[doc = "Checks if the value of the field is `SCT_IN_SEL_23`"]
    #[inline(always)]
    pub fn is_sct_in_sel_23(&self) -> bool {
        *self == SCT_IN_SEL_A::SCT_IN_SEL_23
    }
}
#[doc = "Field `SCT_IN_SEL` writer - SCT0 Input Selection."]
pub type SCT_IN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCT0_IN_SEL_SPEC, u8, SCT_IN_SEL_A, 5, O>;
impl<'a, const O: u8> SCT_IN_SEL_W<'a, O> {
    #[doc = "SCT0_PIN_INP0"]
    #[inline(always)]
    pub fn sct_in_sel_0(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_0)
    }
    #[doc = "SCT0_PIN_INP1"]
    #[inline(always)]
    pub fn sct_in_sel_1(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_1)
    }
    #[doc = "SCT0_PIN_INP2"]
    #[inline(always)]
    pub fn sct_in_sel_2(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_2)
    }
    #[doc = "SCT0_PIN_INP3"]
    #[inline(always)]
    pub fn sct_in_sel_3(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_3)
    }
    #[doc = "SCT0_PIN_INP4"]
    #[inline(always)]
    pub fn sct_in_sel_4(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_4)
    }
    #[doc = "SCT0_PIN_INP5"]
    #[inline(always)]
    pub fn sct_in_sel_5(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_5)
    }
    #[doc = "SCT0_PIN_INP6"]
    #[inline(always)]
    pub fn sct_in_sel_6(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_6)
    }
    #[doc = "SCT0_PIN_INP7"]
    #[inline(always)]
    pub fn sct_in_sel_7(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_7)
    }
    #[doc = "CT32BIT0_MAT0"]
    #[inline(always)]
    pub fn sct_in_sel_8(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_8)
    }
    #[doc = "CT32BIT1_MAT0"]
    #[inline(always)]
    pub fn sct_in_sel_9(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_9)
    }
    #[doc = "CT32BIT2_MAT0"]
    #[inline(always)]
    pub fn sct_in_sel_10(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_10)
    }
    #[doc = "CT32BIT3_MAT0"]
    #[inline(always)]
    pub fn sct_in_sel_11(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_11)
    }
    #[doc = "CT32BIT4_MAT0"]
    #[inline(always)]
    pub fn sct_in_sel_12(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_12)
    }
    #[doc = "ADCIRQ"]
    #[inline(always)]
    pub fn sct_in_sel_13(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_13)
    }
    #[doc = "GPIOINT_BMATCH"]
    #[inline(always)]
    pub fn sct_in_sel_14(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_14)
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn sct_in_sel_15(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_15)
    }
    #[doc = "CMP0_OUT"]
    #[inline(always)]
    pub fn sct_in_sel_16(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_16)
    }
    #[doc = "SHARED I2S0_SCLK"]
    #[inline(always)]
    pub fn sct_in_sel_17(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_17)
    }
    #[doc = "SHARED I2S1_SCLK"]
    #[inline(always)]
    pub fn sct_in_sel_18(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_18)
    }
    #[doc = "SHARED I2S0_WS"]
    #[inline(always)]
    pub fn sct_in_sel_19(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_19)
    }
    #[doc = "SHARED I2S1_WS"]
    #[inline(always)]
    pub fn sct_in_sel_20(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_20)
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn sct_in_sel_21(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_21)
    }
    #[doc = "ARM_TXEV"]
    #[inline(always)]
    pub fn sct_in_sel_22(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_22)
    }
    #[doc = "DEBUG_HALTED"]
    #[inline(always)]
    pub fn sct_in_sel_23(self) -> &'a mut W {
        self.variant(SCT_IN_SEL_A::SCT_IN_SEL_23)
    }
}
impl R {
    #[doc = "Bits 0:4 - SCT0 Input Selection."]
    #[inline(always)]
    pub fn sct_in_sel(&self) -> SCT_IN_SEL_R {
        SCT_IN_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SCT0 Input Selection."]
    #[inline(always)]
    #[must_use]
    pub fn sct_in_sel(&mut self) -> SCT_IN_SEL_W<0> {
        SCT_IN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT Peripheral Input multiplexer index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sct0_in_sel](index.html) module"]
pub struct SCT0_IN_SEL_SPEC;
impl crate::RegisterSpec for SCT0_IN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sct0_in_sel::R](R) reader structure"]
impl crate::Readable for SCT0_IN_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sct0_in_sel::W](W) writer structure"]
impl crate::Writable for SCT0_IN_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCT0_IN_SEL[%s]
to value 0x1f"]
impl crate::Resettable for SCT0_IN_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
