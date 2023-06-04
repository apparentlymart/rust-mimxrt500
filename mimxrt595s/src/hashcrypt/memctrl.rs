#[doc = "Register `MEMCTRL` reader"]
pub struct R(crate::R<MEMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMCTRL` writer"]
pub struct W(crate::W<MEMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMCTRL_SPEC>;
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
impl From<crate::W<MEMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER` reader - Master"]
pub type MASTER_R = crate::BitReader<MASTER_A>;
#[doc = "Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_A {
    #[doc = "0: Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    NOT_USED = 0,
    #[doc = "1: Mastering is enabled and DMA and INDATA should not be used."]
    ENABLED = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::NOT_USED,
            true => MASTER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == MASTER_A::NOT_USED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MASTER_A::ENABLED
    }
}
#[doc = "Field `MASTER` writer - Master"]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMCTRL_SPEC, MASTER_A, O>;
impl<'a, const O: u8> MASTER_W<'a, O> {
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut W {
        self.variant(MASTER_A::NOT_USED)
    }
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MASTER_A::ENABLED)
    }
}
#[doc = "Field `COUNT` reader - Count"]
pub type COUNT_R = crate::FieldReader<u16, COUNT_A>;
#[doc = "Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum COUNT_A {
    #[doc = "0: Done. Nothing to process"]
    COUNT_0 = 0,
    #[doc = "1: One 512-bit block to hash"]
    COUNT_1 = 1,
    #[doc = "2: Two 512-bit block to hash"]
    COUNT_2 = 2,
    #[doc = "3: Three 512-bit block to hash"]
    COUNT_3 = 3,
}
impl From<COUNT_A> for u16 {
    #[inline(always)]
    fn from(variant: COUNT_A) -> Self {
        variant as _
    }
}
impl COUNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COUNT_A> {
        match self.bits {
            0 => Some(COUNT_A::COUNT_0),
            1 => Some(COUNT_A::COUNT_1),
            2 => Some(COUNT_A::COUNT_2),
            3 => Some(COUNT_A::COUNT_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COUNT_0`"]
    #[inline(always)]
    pub fn is_count_0(&self) -> bool {
        *self == COUNT_A::COUNT_0
    }
    #[doc = "Checks if the value of the field is `COUNT_1`"]
    #[inline(always)]
    pub fn is_count_1(&self) -> bool {
        *self == COUNT_A::COUNT_1
    }
    #[doc = "Checks if the value of the field is `COUNT_2`"]
    #[inline(always)]
    pub fn is_count_2(&self) -> bool {
        *self == COUNT_A::COUNT_2
    }
    #[doc = "Checks if the value of the field is `COUNT_3`"]
    #[inline(always)]
    pub fn is_count_3(&self) -> bool {
        *self == COUNT_A::COUNT_3
    }
}
#[doc = "Field `COUNT` writer - Count"]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEMCTRL_SPEC, u16, COUNT_A, 11, O>;
impl<'a, const O: u8> COUNT_W<'a, O> {
    #[doc = "Done. Nothing to process"]
    #[inline(always)]
    pub fn count_0(self) -> &'a mut W {
        self.variant(COUNT_A::COUNT_0)
    }
    #[doc = "One 512-bit block to hash"]
    #[inline(always)]
    pub fn count_1(self) -> &'a mut W {
        self.variant(COUNT_A::COUNT_1)
    }
    #[doc = "Two 512-bit block to hash"]
    #[inline(always)]
    pub fn count_2(self) -> &'a mut W {
        self.variant(COUNT_A::COUNT_2)
    }
    #[doc = "Three 512-bit block to hash"]
    #[inline(always)]
    pub fn count_3(self) -> &'a mut W {
        self.variant(COUNT_A::COUNT_3)
    }
}
impl R {
    #[doc = "Bit 0 - Master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:26 - Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Master"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<0> {
        MASTER_W::new(self)
    }
    #[doc = "Bits 16:26 - Count"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<16> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memctrl](index.html) module"]
pub struct MEMCTRL_SPEC;
impl crate::RegisterSpec for MEMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memctrl::R](R) reader structure"]
impl crate::Readable for MEMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memctrl::W](W) writer structure"]
impl crate::Writable for MEMCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEMCTRL to value 0"]
impl crate::Resettable for MEMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
