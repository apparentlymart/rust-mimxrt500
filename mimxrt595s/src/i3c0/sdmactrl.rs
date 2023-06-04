#[doc = "Register `SDMACTRL` reader"]
pub struct R(crate::R<SDMACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMACTRL` writer"]
pub struct W(crate::W<SDMACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMACTRL_SPEC>;
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
impl From<crate::W<SDMACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAFB` reader - DMA Read (From-bus) trigger"]
pub type DMAFB_R = crate::FieldReader<u8, DMAFB_A>;
#[doc = "DMA Read (From-bus) trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAFB_A {
    #[doc = "0: DMA not used"]
    NOT_USED = 0,
    #[doc = "1: DMA is enabled for 1 frame"]
    ENABLE_ONE_FRAME = 1,
    #[doc = "2: DMA enable"]
    ENABLE = 2,
}
impl From<DMAFB_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAFB_A) -> Self {
        variant as _
    }
}
impl DMAFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAFB_A> {
        match self.bits {
            0 => Some(DMAFB_A::NOT_USED),
            1 => Some(DMAFB_A::ENABLE_ONE_FRAME),
            2 => Some(DMAFB_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == DMAFB_A::NOT_USED
    }
    #[doc = "Checks if the value of the field is `ENABLE_ONE_FRAME`"]
    #[inline(always)]
    pub fn is_enable_one_frame(&self) -> bool {
        *self == DMAFB_A::ENABLE_ONE_FRAME
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAFB_A::ENABLE
    }
}
#[doc = "Field `DMAFB` writer - DMA Read (From-bus) trigger"]
pub type DMAFB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDMACTRL_SPEC, u8, DMAFB_A, 2, O>;
impl<'a, const O: u8> DMAFB_W<'a, O> {
    #[doc = "DMA not used"]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut W {
        self.variant(DMAFB_A::NOT_USED)
    }
    #[doc = "DMA is enabled for 1 frame"]
    #[inline(always)]
    pub fn enable_one_frame(self) -> &'a mut W {
        self.variant(DMAFB_A::ENABLE_ONE_FRAME)
    }
    #[doc = "DMA enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAFB_A::ENABLE)
    }
}
#[doc = "Field `DMATB` reader - DMA Write (To-bus) trigger"]
pub type DMATB_R = crate::FieldReader<u8, DMATB_A>;
#[doc = "DMA Write (To-bus) trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMATB_A {
    #[doc = "0: NOT_USED"]
    NOT_USED = 0,
    #[doc = "1: ENABLE_ONE_FRAME"]
    ENABLE_ONE_FRAME = 1,
    #[doc = "2: ENABLE"]
    ENABLE = 2,
}
impl From<DMATB_A> for u8 {
    #[inline(always)]
    fn from(variant: DMATB_A) -> Self {
        variant as _
    }
}
impl DMATB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMATB_A> {
        match self.bits {
            0 => Some(DMATB_A::NOT_USED),
            1 => Some(DMATB_A::ENABLE_ONE_FRAME),
            2 => Some(DMATB_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == DMATB_A::NOT_USED
    }
    #[doc = "Checks if the value of the field is `ENABLE_ONE_FRAME`"]
    #[inline(always)]
    pub fn is_enable_one_frame(&self) -> bool {
        *self == DMATB_A::ENABLE_ONE_FRAME
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMATB_A::ENABLE
    }
}
#[doc = "Field `DMATB` writer - DMA Write (To-bus) trigger"]
pub type DMATB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDMACTRL_SPEC, u8, DMATB_A, 2, O>;
impl<'a, const O: u8> DMATB_W<'a, O> {
    #[doc = "NOT_USED"]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut W {
        self.variant(DMATB_A::NOT_USED)
    }
    #[doc = "ENABLE_ONE_FRAME"]
    #[inline(always)]
    pub fn enable_one_frame(self) -> &'a mut W {
        self.variant(DMATB_A::ENABLE_ONE_FRAME)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMATB_A::ENABLE)
    }
}
#[doc = "Field `DMAWIDTH` reader - Width of DMA operations"]
pub type DMAWIDTH_R = crate::FieldReader<u8, DMAWIDTH_A>;
#[doc = "Width of DMA operations\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAWIDTH_A {
    #[doc = "0: BYTE"]
    BYTE = 0,
    #[doc = "1: BYTE_AGAIN"]
    BYTE_AGAIN = 1,
    #[doc = "2: HALF_WORD: Half word (16 bits). This will make sure that 2 bytes are free/available in the FIFO."]
    HALF_WORD = 2,
}
impl From<DMAWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAWIDTH_A) -> Self {
        variant as _
    }
}
impl DMAWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAWIDTH_A> {
        match self.bits {
            0 => Some(DMAWIDTH_A::BYTE),
            1 => Some(DMAWIDTH_A::BYTE_AGAIN),
            2 => Some(DMAWIDTH_A::HALF_WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DMAWIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `BYTE_AGAIN`"]
    #[inline(always)]
    pub fn is_byte_again(&self) -> bool {
        *self == DMAWIDTH_A::BYTE_AGAIN
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DMAWIDTH_A::HALF_WORD
    }
}
#[doc = "Field `DMAWIDTH` writer - Width of DMA operations"]
pub type DMAWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMACTRL_SPEC, u8, DMAWIDTH_A, 2, O>;
impl<'a, const O: u8> DMAWIDTH_W<'a, O> {
    #[doc = "BYTE"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DMAWIDTH_A::BYTE)
    }
    #[doc = "BYTE_AGAIN"]
    #[inline(always)]
    pub fn byte_again(self) -> &'a mut W {
        self.variant(DMAWIDTH_A::BYTE_AGAIN)
    }
    #[doc = "HALF_WORD: Half word (16 bits). This will make sure that 2 bytes are free/available in the FIFO."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(DMAWIDTH_A::HALF_WORD)
    }
}
impl R {
    #[doc = "Bits 0:1 - DMA Read (From-bus) trigger"]
    #[inline(always)]
    pub fn dmafb(&self) -> DMAFB_R {
        DMAFB_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DMA Write (To-bus) trigger"]
    #[inline(always)]
    pub fn dmatb(&self) -> DMATB_R {
        DMATB_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Width of DMA operations"]
    #[inline(always)]
    pub fn dmawidth(&self) -> DMAWIDTH_R {
        DMAWIDTH_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMA Read (From-bus) trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dmafb(&mut self) -> DMAFB_W<0> {
        DMAFB_W::new(self)
    }
    #[doc = "Bits 2:3 - DMA Write (To-bus) trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dmatb(&mut self) -> DMATB_W<2> {
        DMATB_W::new(self)
    }
    #[doc = "Bits 4:5 - Width of DMA operations"]
    #[inline(always)]
    #[must_use]
    pub fn dmawidth(&mut self) -> DMAWIDTH_W<4> {
        DMAWIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmactrl](index.html) module"]
pub struct SDMACTRL_SPEC;
impl crate::RegisterSpec for SDMACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmactrl::R](R) reader structure"]
impl crate::Readable for SDMACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmactrl::W](W) writer structure"]
impl crate::Writable for SDMACTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMACTRL to value 0x10"]
impl crate::Resettable for SDMACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
