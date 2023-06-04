#[doc = "Register `MDATACTRL` reader"]
pub struct R(crate::R<MDATACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDATACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDATACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDATACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDATACTRL` writer"]
pub struct W(crate::W<MDATACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDATACTRL_SPEC>;
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
impl From<crate::W<MDATACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDATACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLUSHTB` writer - Flush to-bus buffer/FIFO"]
pub type FLUSHTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDATACTRL_SPEC, bool, O>;
#[doc = "Field `FLUSHFB` writer - Flush from-bus buffer/FIFO"]
pub type FLUSHFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDATACTRL_SPEC, bool, O>;
#[doc = "Field `UNLOCK` writer - Unlock"]
pub type UNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDATACTRL_SPEC, bool, O>;
#[doc = "Field `TXTRIG` reader - TX trigger level"]
pub type TXTRIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXTRIG` writer - TX trigger level"]
pub type TXTRIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDATACTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXTRIG` reader - RX trigger level"]
pub type RXTRIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXTRIG` writer - RX trigger level"]
pub type RXTRIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDATACTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXCOUNT` reader - TX byte count"]
pub type TXCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXCOUNT` reader - RX byte count"]
pub type RXCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFULL` reader - TX is full"]
pub type TXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXEMPTY` reader - RX is empty"]
pub type RXEMPTY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 4:5 - TX trigger level"]
    #[inline(always)]
    pub fn txtrig(&self) -> TXTRIG_R {
        TXTRIG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RX trigger level"]
    #[inline(always)]
    pub fn rxtrig(&self) -> RXTRIG_R {
        RXTRIG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:20 - TX byte count"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - RX byte count"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - TX is full"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RX is empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush to-bus buffer/FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn flushtb(&mut self) -> FLUSHTB_W<0> {
        FLUSHTB_W::new(self)
    }
    #[doc = "Bit 1 - Flush from-bus buffer/FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn flushfb(&mut self) -> FLUSHFB_W<1> {
        FLUSHFB_W::new(self)
    }
    #[doc = "Bit 2 - Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<2> {
        UNLOCK_W::new(self)
    }
    #[doc = "Bits 4:5 - TX trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn txtrig(&mut self) -> TXTRIG_W<4> {
        TXTRIG_W::new(self)
    }
    #[doc = "Bits 6:7 - RX trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn rxtrig(&mut self) -> RXTRIG_W<6> {
        RXTRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Data Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdatactrl](index.html) module"]
pub struct MDATACTRL_SPEC;
impl crate::RegisterSpec for MDATACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdatactrl::R](R) reader structure"]
impl crate::Readable for MDATACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdatactrl::W](W) writer structure"]
impl crate::Writable for MDATACTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDATACTRL to value 0x8000_0030"]
impl crate::Resettable for MDATACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0030;
}
