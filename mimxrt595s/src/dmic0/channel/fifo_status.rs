#[doc = "Register `FIFO_STATUS` reader"]
pub struct R(crate::R<FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_STATUS` writer"]
pub struct W(crate::W<FIFO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_STATUS_SPEC>;
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
impl From<crate::W<FIFO_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - Status of Interrupt (write 1 to clear)"]
pub type INT_R = crate::BitReader<bool>;
#[doc = "Field `INT` writer - Status of Interrupt (write 1 to clear)"]
pub type INT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FIFO_STATUS_SPEC, bool, O>;
#[doc = "Field `OVERRUN` reader - Overrun Detected (write 1 to clear)"]
pub type OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` writer - Overrun Detected (write 1 to clear)"]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FIFO_STATUS_SPEC, bool, O>;
#[doc = "Field `UNDERRUN` reader - Underrun Detected (write 1 to clear)"]
pub type UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN` writer - Underrun Detected (write 1 to clear)"]
pub type UNDERRUN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FIFO_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Status of Interrupt (write 1 to clear)"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status of Interrupt (write 1 to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<0> {
        INT_W::new(self)
    }
    #[doc = "Bit 1 - Overrun Detected (write 1 to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<1> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 2 - Underrun Detected (write 1 to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn underrun(&mut self) -> UNDERRUN_W<2> {
        UNDERRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_status](index.html) module"]
pub struct FIFO_STATUS_SPEC;
impl crate::RegisterSpec for FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_status::R](R) reader structure"]
impl crate::Readable for FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_status::W](W) writer structure"]
impl crate::Writable for FIFO_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x07;
}
#[doc = "`reset()` method sets FIFO_STATUS to value 0"]
impl crate::Resettable for FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
