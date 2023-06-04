#[doc = "Register `SERRWARN` reader"]
pub struct R(crate::R<SERRWARN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERRWARN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERRWARN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERRWARN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SERRWARN` writer"]
pub struct W(crate::W<SERRWARN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SERRWARN_SPEC>;
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
impl From<crate::W<SERRWARN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SERRWARN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ORUN` reader - Overrun error"]
pub type ORUN_R = crate::BitReader<bool>;
#[doc = "Field `ORUN` writer - Overrun error"]
pub type ORUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `URUN` reader - Underrun error"]
pub type URUN_R = crate::BitReader<bool>;
#[doc = "Field `URUN` writer - Underrun error"]
pub type URUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `URUNNACK` reader - Underrun and Not Acknowledged (NACKed) error"]
pub type URUNNACK_R = crate::BitReader<bool>;
#[doc = "Field `URUNNACK` writer - Underrun and Not Acknowledged (NACKed) error"]
pub type URUNNACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `TERM` reader - Terminated error"]
pub type TERM_R = crate::BitReader<bool>;
#[doc = "Field `TERM` writer - Terminated error"]
pub type TERM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `INVSTART` reader - Invalid start error"]
pub type INVSTART_R = crate::BitReader<bool>;
#[doc = "Field `INVSTART` writer - Invalid start error"]
pub type INVSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `SPAR` reader - SDR parity error"]
pub type SPAR_R = crate::BitReader<bool>;
#[doc = "Field `SPAR` writer - SDR parity error"]
pub type SPAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `HPAR` reader - HDR parity error"]
pub type HPAR_R = crate::BitReader<bool>;
#[doc = "Field `HPAR` writer - HDR parity error"]
pub type HPAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `HCRC` reader - HDR-DDR CRC error"]
pub type HCRC_R = crate::BitReader<bool>;
#[doc = "Field `HCRC` writer - HDR-DDR CRC error"]
pub type HCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `S0S1` reader - S0 or S1 error"]
pub type S0S1_R = crate::BitReader<bool>;
#[doc = "Field `S0S1` writer - S0 or S1 error"]
pub type S0S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `OREAD` reader - Over-read error"]
pub type OREAD_R = crate::BitReader<bool>;
#[doc = "Field `OREAD` writer - Over-read error"]
pub type OREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
#[doc = "Field `OWRITE` reader - Over-write error"]
pub type OWRITE_R = crate::BitReader<bool>;
#[doc = "Field `OWRITE` writer - Over-write error"]
pub type OWRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERRWARN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn orun(&self) -> ORUN_R {
        ORUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underrun error"]
    #[inline(always)]
    pub fn urun(&self) -> URUN_R {
        URUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underrun and Not Acknowledged (NACKed) error"]
    #[inline(always)]
    pub fn urunnack(&self) -> URUNNACK_R {
        URUNNACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Terminated error"]
    #[inline(always)]
    pub fn term(&self) -> TERM_R {
        TERM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Invalid start error"]
    #[inline(always)]
    pub fn invstart(&self) -> INVSTART_R {
        INVSTART_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SDR parity error"]
    #[inline(always)]
    pub fn spar(&self) -> SPAR_R {
        SPAR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HDR parity error"]
    #[inline(always)]
    pub fn hpar(&self) -> HPAR_R {
        HPAR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HDR-DDR CRC error"]
    #[inline(always)]
    pub fn hcrc(&self) -> HCRC_R {
        HCRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - S0 or S1 error"]
    #[inline(always)]
    pub fn s0s1(&self) -> S0S1_R {
        S0S1_R::new(((self.bits >> 11) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    #[must_use]
    pub fn orun(&mut self) -> ORUN_W<0> {
        ORUN_W::new(self)
    }
    #[doc = "Bit 1 - Underrun error"]
    #[inline(always)]
    #[must_use]
    pub fn urun(&mut self) -> URUN_W<1> {
        URUN_W::new(self)
    }
    #[doc = "Bit 2 - Underrun and Not Acknowledged (NACKed) error"]
    #[inline(always)]
    #[must_use]
    pub fn urunnack(&mut self) -> URUNNACK_W<2> {
        URUNNACK_W::new(self)
    }
    #[doc = "Bit 3 - Terminated error"]
    #[inline(always)]
    #[must_use]
    pub fn term(&mut self) -> TERM_W<3> {
        TERM_W::new(self)
    }
    #[doc = "Bit 4 - Invalid start error"]
    #[inline(always)]
    #[must_use]
    pub fn invstart(&mut self) -> INVSTART_W<4> {
        INVSTART_W::new(self)
    }
    #[doc = "Bit 8 - SDR parity error"]
    #[inline(always)]
    #[must_use]
    pub fn spar(&mut self) -> SPAR_W<8> {
        SPAR_W::new(self)
    }
    #[doc = "Bit 9 - HDR parity error"]
    #[inline(always)]
    #[must_use]
    pub fn hpar(&mut self) -> HPAR_W<9> {
        HPAR_W::new(self)
    }
    #[doc = "Bit 10 - HDR-DDR CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn hcrc(&mut self) -> HCRC_W<10> {
        HCRC_W::new(self)
    }
    #[doc = "Bit 11 - S0 or S1 error"]
    #[inline(always)]
    #[must_use]
    pub fn s0s1(&mut self) -> S0S1_W<11> {
        S0S1_W::new(self)
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Errors and Warnings Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serrwarn](index.html) module"]
pub struct SERRWARN_SPEC;
impl crate::RegisterSpec for SERRWARN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [serrwarn::R](R) reader structure"]
impl crate::Readable for SERRWARN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [serrwarn::W](W) writer structure"]
impl crate::Writable for SERRWARN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SERRWARN to value 0"]
impl crate::Resettable for SERRWARN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
