#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECODE_OPCODE` reader - Decode opcode"]
pub type DECODE_OPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECODE_OPCODE` writer - Decode opcode"]
pub type DECODE_OPCODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DECODE_MACHINE` reader - Decode machine"]
pub type DECODE_MACHINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECODE_MACHINE` writer - Decode machine"]
pub type DECODE_MACHINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `INST_BUSY` reader - Instruction busy"]
pub type INST_BUSY_R = crate::BitReader<INST_BUSY_A>;
#[doc = "Instruction busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INST_BUSY_A {
    #[doc = "0: Not busy"]
    NOT_BUSY = 0,
    #[doc = "1: busy"]
    BUSY = 1,
}
impl From<INST_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: INST_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl INST_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INST_BUSY_A {
        match self.bits {
            false => INST_BUSY_A::NOT_BUSY,
            true => INST_BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == INST_BUSY_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == INST_BUSY_A::BUSY
    }
}
impl R {
    #[doc = "Bits 0:3 - Decode opcode"]
    #[inline(always)]
    pub fn decode_opcode(&self) -> DECODE_OPCODE_R {
        DECODE_OPCODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Decode machine"]
    #[inline(always)]
    pub fn decode_machine(&self) -> DECODE_MACHINE_R {
        DECODE_MACHINE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Instruction busy"]
    #[inline(always)]
    pub fn inst_busy(&self) -> INST_BUSY_R {
        INST_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Decode opcode"]
    #[inline(always)]
    #[must_use]
    pub fn decode_opcode(&mut self) -> DECODE_OPCODE_W<0> {
        DECODE_OPCODE_W::new(self)
    }
    #[doc = "Bits 4:7 - Decode machine"]
    #[inline(always)]
    #[must_use]
    pub fn decode_machine(&mut self) -> DECODE_MACHINE_W<4> {
        DECODE_MACHINE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
