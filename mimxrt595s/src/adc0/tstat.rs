#[doc = "Register `TSTAT` reader"]
pub struct R(crate::R<TSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSTAT` writer"]
pub struct W(crate::W<TSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSTAT_SPEC>;
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
impl From<crate::W<TSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXC_NUM` reader - Trigger Exception Number"]
pub type TEXC_NUM_R = crate::FieldReader<u16, TEXC_NUM_A>;
#[doc = "Trigger Exception Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TEXC_NUM_A {
    #[doc = "0: No triggers have been interrupted by a high-priority exception."]
    NO_EXCEPTIONS = 0,
    #[doc = "1: Trigger 0 has been interrupted by a high-priority exception."]
    BIT0_MEANS_TRIGGER_0_INTERRUPTED = 1,
    #[doc = "2: Trigger 1 has been interrupted by a high-priority exception."]
    BIT1_MEANS_TRIGGER_1_INTERRUPTED = 2,
    #[doc = "3: Associated trigger sequence has interrupted by a high-priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_3 = 3,
    #[doc = "4: Associated trigger sequence has interrupted by a high-priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_4 = 4,
    #[doc = "5: Associated trigger sequence has interrupted by a high-priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_5 = 5,
    #[doc = "6: Associated trigger sequence has interrupted by a high-priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_6 = 6,
    #[doc = "7: Associated trigger sequence has interrupted by a high-priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_7 = 7,
    #[doc = "8: Associated trigger sequence has interrupted by a high-priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_8 = 8,
    #[doc = "9: Associated trigger sequence has interrupted by a high-priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_9 = 9,
    #[doc = "65535: Every trigger sequence has been interrupted by a high-priority exception."]
    ALL_BITS_SET_INDICATE_ALL_TRIGGERS_INTERRUPTED = 65535,
}
impl From<TEXC_NUM_A> for u16 {
    #[inline(always)]
    fn from(variant: TEXC_NUM_A) -> Self {
        variant as _
    }
}
impl TEXC_NUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEXC_NUM_A> {
        match self.bits {
            0 => Some(TEXC_NUM_A::NO_EXCEPTIONS),
            1 => Some(TEXC_NUM_A::BIT0_MEANS_TRIGGER_0_INTERRUPTED),
            2 => Some(TEXC_NUM_A::BIT1_MEANS_TRIGGER_1_INTERRUPTED),
            3 => Some(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_3),
            4 => Some(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_4),
            5 => Some(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_5),
            6 => Some(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_6),
            7 => Some(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_7),
            8 => Some(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_8),
            9 => Some(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_9),
            65535 => Some(TEXC_NUM_A::ALL_BITS_SET_INDICATE_ALL_TRIGGERS_INTERRUPTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EXCEPTIONS`"]
    #[inline(always)]
    pub fn is_no_exceptions(&self) -> bool {
        *self == TEXC_NUM_A::NO_EXCEPTIONS
    }
    #[doc = "Checks if the value of the field is `BIT0_MEANS_TRIGGER_0_INTERRUPTED`"]
    #[inline(always)]
    pub fn is_bit0_means_trigger_0_interrupted(&self) -> bool {
        *self == TEXC_NUM_A::BIT0_MEANS_TRIGGER_0_INTERRUPTED
    }
    #[doc = "Checks if the value of the field is `BIT1_MEANS_TRIGGER_1_INTERRUPTED`"]
    #[inline(always)]
    pub fn is_bit1_means_trigger_1_interrupted(&self) -> bool {
        *self == TEXC_NUM_A::BIT1_MEANS_TRIGGER_1_INTERRUPTED
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_3`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_interrupted_3(&self) -> bool {
        *self == TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_3
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_4`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_interrupted_4(&self) -> bool {
        *self == TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_4
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_5`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_interrupted_5(&self) -> bool {
        *self == TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_5
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_6`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_interrupted_6(&self) -> bool {
        *self == TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_6
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_7`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_interrupted_7(&self) -> bool {
        *self == TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_7
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_8`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_interrupted_8(&self) -> bool {
        *self == TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_8
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_9`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_interrupted_9(&self) -> bool {
        *self == TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_9
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_SET_INDICATE_ALL_TRIGGERS_INTERRUPTED`"]
    #[inline(always)]
    pub fn is_all_bits_set_indicate_all_triggers_interrupted(&self) -> bool {
        *self == TEXC_NUM_A::ALL_BITS_SET_INDICATE_ALL_TRIGGERS_INTERRUPTED
    }
}
#[doc = "Field `TEXC_NUM` writer - Trigger Exception Number"]
pub type TEXC_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSTAT_SPEC, u16, TEXC_NUM_A, 16, O>;
impl<'a, const O: u8> TEXC_NUM_W<'a, O> {
    #[doc = "No triggers have been interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn no_exceptions(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::NO_EXCEPTIONS)
    }
    #[doc = "Trigger 0 has been interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn bit0_means_trigger_0_interrupted(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::BIT0_MEANS_TRIGGER_0_INTERRUPTED)
    }
    #[doc = "Trigger 1 has been interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn bit1_means_trigger_1_interrupted(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::BIT1_MEANS_TRIGGER_1_INTERRUPTED)
    }
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_interrupted_3(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_3)
    }
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_interrupted_4(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_4)
    }
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_interrupted_5(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_5)
    }
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_interrupted_6(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_6)
    }
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_interrupted_7(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_7)
    }
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_interrupted_8(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_8)
    }
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_interrupted_9(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_9)
    }
    #[doc = "Every trigger sequence has been interrupted by a high-priority exception."]
    #[inline(always)]
    pub fn all_bits_set_indicate_all_triggers_interrupted(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::ALL_BITS_SET_INDICATE_ALL_TRIGGERS_INTERRUPTED)
    }
}
#[doc = "Field `TCOMP_FLAG` reader - Trigger Completion Flag"]
pub type TCOMP_FLAG_R = crate::FieldReader<u16, TCOMP_FLAG_A>;
#[doc = "Trigger Completion Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TCOMP_FLAG_A {
    #[doc = "0: No triggers have been completed. Trigger completion interrupts are disabled."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 0 has been completed and trigger 0 has enabled completion interrupts."]
    BIT0_MEANS_TRIGGER_0_COMPLETED = 1,
    #[doc = "2: Trigger 1 has been completed and trigger 1 has enabled completion interrupts."]
    BIT1_MEANS_TRIGGER_1_COMPLETED = 2,
    #[doc = "3: Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_3 = 3,
    #[doc = "4: Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_4 = 4,
    #[doc = "5: Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_5 = 5,
    #[doc = "6: Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_6 = 6,
    #[doc = "7: Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_7 = 7,
    #[doc = "8: Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_8 = 8,
    #[doc = "9: Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_9 = 9,
    #[doc = "65535: Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    ALL_BITS_SET_INDICATE_ALL_TRIGGERS_COMPLETED = 65535,
}
impl From<TCOMP_FLAG_A> for u16 {
    #[inline(always)]
    fn from(variant: TCOMP_FLAG_A) -> Self {
        variant as _
    }
}
impl TCOMP_FLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCOMP_FLAG_A> {
        match self.bits {
            0 => Some(TCOMP_FLAG_A::NO_TRIGGER),
            1 => Some(TCOMP_FLAG_A::BIT0_MEANS_TRIGGER_0_COMPLETED),
            2 => Some(TCOMP_FLAG_A::BIT1_MEANS_TRIGGER_1_COMPLETED),
            3 => Some(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_3),
            4 => Some(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_4),
            5 => Some(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_5),
            6 => Some(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_6),
            7 => Some(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_7),
            8 => Some(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_8),
            9 => Some(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_9),
            65535 => Some(TCOMP_FLAG_A::ALL_BITS_SET_INDICATE_ALL_TRIGGERS_COMPLETED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TCOMP_FLAG_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `BIT0_MEANS_TRIGGER_0_COMPLETED`"]
    #[inline(always)]
    pub fn is_bit0_means_trigger_0_completed(&self) -> bool {
        *self == TCOMP_FLAG_A::BIT0_MEANS_TRIGGER_0_COMPLETED
    }
    #[doc = "Checks if the value of the field is `BIT1_MEANS_TRIGGER_1_COMPLETED`"]
    #[inline(always)]
    pub fn is_bit1_means_trigger_1_completed(&self) -> bool {
        *self == TCOMP_FLAG_A::BIT1_MEANS_TRIGGER_1_COMPLETED
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_COMPLETED_3`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_completed_3(&self) -> bool {
        *self == TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_3
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_COMPLETED_4`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_completed_4(&self) -> bool {
        *self == TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_4
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_COMPLETED_5`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_completed_5(&self) -> bool {
        *self == TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_5
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_COMPLETED_6`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_completed_6(&self) -> bool {
        *self == TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_6
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_COMPLETED_7`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_completed_7(&self) -> bool {
        *self == TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_7
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_COMPLETED_8`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_completed_8(&self) -> bool {
        *self == TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_8
    }
    #[doc = "Checks if the value of the field is `SET_BITS_INDICATE_TRIGGER_X_COMPLETED_9`"]
    #[inline(always)]
    pub fn is_set_bits_indicate_trigger_x_completed_9(&self) -> bool {
        *self == TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_9
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_SET_INDICATE_ALL_TRIGGERS_COMPLETED`"]
    #[inline(always)]
    pub fn is_all_bits_set_indicate_all_triggers_completed(&self) -> bool {
        *self == TCOMP_FLAG_A::ALL_BITS_SET_INDICATE_ALL_TRIGGERS_COMPLETED
    }
}
#[doc = "Field `TCOMP_FLAG` writer - Trigger Completion Flag"]
pub type TCOMP_FLAG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSTAT_SPEC, u16, TCOMP_FLAG_A, 16, O>;
impl<'a, const O: u8> TCOMP_FLAG_W<'a, O> {
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::NO_TRIGGER)
    }
    #[doc = "Trigger 0 has been completed and trigger 0 has enabled completion interrupts."]
    #[inline(always)]
    pub fn bit0_means_trigger_0_completed(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::BIT0_MEANS_TRIGGER_0_COMPLETED)
    }
    #[doc = "Trigger 1 has been completed and trigger 1 has enabled completion interrupts."]
    #[inline(always)]
    pub fn bit1_means_trigger_1_completed(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::BIT1_MEANS_TRIGGER_1_COMPLETED)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_completed_3(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_3)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_completed_4(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_4)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_completed_5(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_5)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_completed_6(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_6)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_completed_7(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_7)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_completed_8(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_8)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn set_bits_indicate_trigger_x_completed_9(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::SET_BITS_INDICATE_TRIGGER_X_COMPLETED_9)
    }
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    #[inline(always)]
    pub fn all_bits_set_indicate_all_triggers_completed(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::ALL_BITS_SET_INDICATE_ALL_TRIGGERS_COMPLETED)
    }
}
impl R {
    #[doc = "Bits 0:15 - Trigger Exception Number"]
    #[inline(always)]
    pub fn texc_num(&self) -> TEXC_NUM_R {
        TEXC_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Trigger Completion Flag"]
    #[inline(always)]
    pub fn tcomp_flag(&self) -> TCOMP_FLAG_R {
        TCOMP_FLAG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Trigger Exception Number"]
    #[inline(always)]
    #[must_use]
    pub fn texc_num(&mut self) -> TEXC_NUM_W<0> {
        TEXC_NUM_W::new(self)
    }
    #[doc = "Bits 16:31 - Trigger Completion Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp_flag(&mut self) -> TCOMP_FLAG_W<16> {
        TCOMP_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstat](index.html) module"]
pub struct TSTAT_SPEC;
impl crate::RegisterSpec for TSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tstat::R](R) reader structure"]
impl crate::Readable for TSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tstat::W](W) writer structure"]
impl crate::Writable for TSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets TSTAT to value 0"]
impl crate::Resettable for TSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
