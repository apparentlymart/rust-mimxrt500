#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - Interrupt request"]
pub type INTEN_R = crate::BitReader<INTEN_A>;
#[doc = "Interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::DISABLED,
            true => INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN_A::ENABLED
    }
}
#[doc = "Field `INTEN` writer - Interrupt request"]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, INTEN_A, O>;
impl<'a, const O: u8> INTEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN_A::ENABLED)
    }
}
#[doc = "Field `MODE` reader - MRT Operating mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "MRT Operating mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Repeat Interrupt mode"]
    REPEAT_INTERRUPT_MODE = 0,
    #[doc = "1: One-Shot Interrupt mode"]
    ONE_SHOT_INTERRUPT_MODE = 1,
    #[doc = "2: One-Shot Stall mode"]
    ONE_SHOT_STALL_MODE = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::REPEAT_INTERRUPT_MODE),
            1 => Some(MODE_A::ONE_SHOT_INTERRUPT_MODE),
            2 => Some(MODE_A::ONE_SHOT_STALL_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REPEAT_INTERRUPT_MODE`"]
    #[inline(always)]
    pub fn is_repeat_interrupt_mode(&self) -> bool {
        *self == MODE_A::REPEAT_INTERRUPT_MODE
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT_INTERRUPT_MODE`"]
    #[inline(always)]
    pub fn is_one_shot_interrupt_mode(&self) -> bool {
        *self == MODE_A::ONE_SHOT_INTERRUPT_MODE
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT_STALL_MODE`"]
    #[inline(always)]
    pub fn is_one_shot_stall_mode(&self) -> bool {
        *self == MODE_A::ONE_SHOT_STALL_MODE
    }
}
#[doc = "Field `MODE` writer - MRT Operating mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Repeat Interrupt mode"]
    #[inline(always)]
    pub fn repeat_interrupt_mode(self) -> &'a mut W {
        self.variant(MODE_A::REPEAT_INTERRUPT_MODE)
    }
    #[doc = "One-Shot Interrupt mode"]
    #[inline(always)]
    pub fn one_shot_interrupt_mode(self) -> &'a mut W {
        self.variant(MODE_A::ONE_SHOT_INTERRUPT_MODE)
    }
    #[doc = "One-Shot Stall mode"]
    #[inline(always)]
    pub fn one_shot_stall_mode(self) -> &'a mut W {
        self.variant(MODE_A::ONE_SHOT_STALL_MODE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt request"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MRT Operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt request"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<0> {
        INTEN_W::new(self)
    }
    #[doc = "Bits 1:2 - MRT Operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
