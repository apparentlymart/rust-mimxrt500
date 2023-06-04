#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSA` writer - Slave Select Assert"]
pub type SSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `SSD` writer - Slave Select Deassert"]
pub type SSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `STALLED` reader - Stalled Status Flag"]
pub type STALLED_R = crate::BitReader<bool>;
#[doc = "Field `ENDTRANSFER` reader - End Transfer Control"]
pub type ENDTRANSFER_R = crate::BitReader<bool>;
#[doc = "Field `ENDTRANSFER` writer - End Transfer Control"]
pub type ENDTRANSFER_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `MSTIDLE` reader - Master Idle Status Flag"]
pub type MSTIDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 6 - Stalled Status Flag"]
    #[inline(always)]
    pub fn stalled(&self) -> STALLED_R {
        STALLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End Transfer Control"]
    #[inline(always)]
    pub fn endtransfer(&self) -> ENDTRANSFER_R {
        ENDTRANSFER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Idle Status Flag"]
    #[inline(always)]
    pub fn mstidle(&self) -> MSTIDLE_R {
        MSTIDLE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Slave Select Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ssa(&mut self) -> SSA_W<4> {
        SSA_W::new(self)
    }
    #[doc = "Bit 5 - Slave Select Deassert"]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<5> {
        SSD_W::new(self)
    }
    #[doc = "Bit 7 - End Transfer Control"]
    #[inline(always)]
    #[must_use]
    pub fn endtransfer(&mut self) -> ENDTRANSFER_W<7> {
        ENDTRANSFER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0100"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
