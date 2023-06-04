#[doc = "Register `MERRWARN` reader"]
pub struct R(crate::R<MERRWARN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MERRWARN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MERRWARN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MERRWARN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MERRWARN` writer"]
pub struct W(crate::W<MERRWARN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MERRWARN_SPEC>;
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
impl From<crate::W<MERRWARN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MERRWARN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NACK` reader - Not acknowledge (NACK) error"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `NACK` writer - Not acknowledge (NACK) error"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
#[doc = "Field `WRABT` reader - WRABT (Write abort) error"]
pub type WRABT_R = crate::BitReader<bool>;
#[doc = "Field `WRABT` writer - WRABT (Write abort) error"]
pub type WRABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
#[doc = "Field `TERM` reader - Terminate error"]
pub type TERM_R = crate::BitReader<bool>;
#[doc = "Field `TERM` writer - Terminate error"]
pub type TERM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
#[doc = "Field `HPAR` reader - High data rate parity"]
pub type HPAR_R = crate::BitReader<bool>;
#[doc = "Field `HPAR` writer - High data rate parity"]
pub type HPAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
#[doc = "Field `HCRC` reader - High data rate CRC error"]
pub type HCRC_R = crate::BitReader<bool>;
#[doc = "Field `HCRC` writer - High data rate CRC error"]
pub type HCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
#[doc = "Field `OREAD` reader - Over-read error"]
pub type OREAD_R = crate::BitReader<bool>;
#[doc = "Field `OREAD` writer - Over-read error"]
pub type OREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
#[doc = "Field `OWRITE` reader - Over-write error"]
pub type OWRITE_R = crate::BitReader<bool>;
#[doc = "Field `OWRITE` writer - Over-write error"]
pub type OWRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
#[doc = "Field `MSGERR` reader - Message error"]
pub type MSGERR_R = crate::BitReader<bool>;
#[doc = "Field `MSGERR` writer - Message error"]
pub type MSGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
#[doc = "Field `INVREQ` reader - Invalid request error"]
pub type INVREQ_R = crate::BitReader<bool>;
#[doc = "Field `INVREQ` writer - Invalid request error"]
pub type INVREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - TIMEOUT error"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - TIMEOUT error"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MERRWARN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Not acknowledge (NACK) error"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WRABT (Write abort) error"]
    #[inline(always)]
    pub fn wrabt(&self) -> WRABT_R {
        WRABT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Terminate error"]
    #[inline(always)]
    pub fn term(&self) -> TERM_R {
        TERM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - High data rate parity"]
    #[inline(always)]
    pub fn hpar(&self) -> HPAR_R {
        HPAR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - High data rate CRC error"]
    #[inline(always)]
    pub fn hcrc(&self) -> HCRC_R {
        HCRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Over-read error"]
    #[inline(always)]
    pub fn oread(&self) -> OREAD_R {
        OREAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Over-write error"]
    #[inline(always)]
    pub fn owrite(&self) -> OWRITE_R {
        OWRITE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Message error"]
    #[inline(always)]
    pub fn msgerr(&self) -> MSGERR_R {
        MSGERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Invalid request error"]
    #[inline(always)]
    pub fn invreq(&self) -> INVREQ_R {
        INVREQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIMEOUT error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Not acknowledge (NACK) error"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<2> {
        NACK_W::new(self)
    }
    #[doc = "Bit 3 - WRABT (Write abort) error"]
    #[inline(always)]
    #[must_use]
    pub fn wrabt(&mut self) -> WRABT_W<3> {
        WRABT_W::new(self)
    }
    #[doc = "Bit 4 - Terminate error"]
    #[inline(always)]
    #[must_use]
    pub fn term(&mut self) -> TERM_W<4> {
        TERM_W::new(self)
    }
    #[doc = "Bit 9 - High data rate parity"]
    #[inline(always)]
    #[must_use]
    pub fn hpar(&mut self) -> HPAR_W<9> {
        HPAR_W::new(self)
    }
    #[doc = "Bit 10 - High data rate CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn hcrc(&mut self) -> HCRC_W<10> {
        HCRC_W::new(self)
    }
    #[doc = "Bit 16 - Over-read error"]
    #[inline(always)]
    #[must_use]
    pub fn oread(&mut self) -> OREAD_W<16> {
        OREAD_W::new(self)
    }
    #[doc = "Bit 17 - Over-write error"]
    #[inline(always)]
    #[must_use]
    pub fn owrite(&mut self) -> OWRITE_W<17> {
        OWRITE_W::new(self)
    }
    #[doc = "Bit 18 - Message error"]
    #[inline(always)]
    #[must_use]
    pub fn msgerr(&mut self) -> MSGERR_W<18> {
        MSGERR_W::new(self)
    }
    #[doc = "Bit 19 - Invalid request error"]
    #[inline(always)]
    #[must_use]
    pub fn invreq(&mut self) -> INVREQ_W<19> {
        INVREQ_W::new(self)
    }
    #[doc = "Bit 20 - TIMEOUT error"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<20> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Errors and Warnings Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [merrwarn](index.html) module"]
pub struct MERRWARN_SPEC;
impl crate::RegisterSpec for MERRWARN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [merrwarn::R](R) reader structure"]
impl crate::Readable for MERRWARN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [merrwarn::W](W) writer structure"]
impl crate::Writable for MERRWARN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MERRWARN to value 0"]
impl crate::Resettable for MERRWARN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
