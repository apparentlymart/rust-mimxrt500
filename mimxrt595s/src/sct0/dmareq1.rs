#[doc = "Register `DMAREQ1` reader"]
pub struct R(crate::R<DMAREQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAREQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAREQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAREQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAREQ1` writer"]
pub struct W(crate::W<DMAREQ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAREQ1_SPEC>;
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
impl From<crate::W<DMAREQ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAREQ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_0` reader - DMA Request Event n"]
pub type DEV_0_R = crate::BitReader<bool>;
#[doc = "Field `DEV_0` writer - DMA Request Event n"]
pub type DEV_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_1` reader - DMA Request Event n"]
pub type DEV_1_R = crate::BitReader<bool>;
#[doc = "Field `DEV_1` writer - DMA Request Event n"]
pub type DEV_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_2` reader - DMA Request Event n"]
pub type DEV_2_R = crate::BitReader<bool>;
#[doc = "Field `DEV_2` writer - DMA Request Event n"]
pub type DEV_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_3` reader - DMA Request Event n"]
pub type DEV_3_R = crate::BitReader<bool>;
#[doc = "Field `DEV_3` writer - DMA Request Event n"]
pub type DEV_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_4` reader - DMA Request Event n"]
pub type DEV_4_R = crate::BitReader<bool>;
#[doc = "Field `DEV_4` writer - DMA Request Event n"]
pub type DEV_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_5` reader - DMA Request Event n"]
pub type DEV_5_R = crate::BitReader<bool>;
#[doc = "Field `DEV_5` writer - DMA Request Event n"]
pub type DEV_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_6` reader - DMA Request Event n"]
pub type DEV_6_R = crate::BitReader<bool>;
#[doc = "Field `DEV_6` writer - DMA Request Event n"]
pub type DEV_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_7` reader - DMA Request Event n"]
pub type DEV_7_R = crate::BitReader<bool>;
#[doc = "Field `DEV_7` writer - DMA Request Event n"]
pub type DEV_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_8` reader - DMA Request Event n"]
pub type DEV_8_R = crate::BitReader<bool>;
#[doc = "Field `DEV_8` writer - DMA Request Event n"]
pub type DEV_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_9` reader - DMA Request Event n"]
pub type DEV_9_R = crate::BitReader<bool>;
#[doc = "Field `DEV_9` writer - DMA Request Event n"]
pub type DEV_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_10` reader - DMA Request Event n"]
pub type DEV_10_R = crate::BitReader<bool>;
#[doc = "Field `DEV_10` writer - DMA Request Event n"]
pub type DEV_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_11` reader - DMA Request Event n"]
pub type DEV_11_R = crate::BitReader<bool>;
#[doc = "Field `DEV_11` writer - DMA Request Event n"]
pub type DEV_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_12` reader - DMA Request Event n"]
pub type DEV_12_R = crate::BitReader<bool>;
#[doc = "Field `DEV_12` writer - DMA Request Event n"]
pub type DEV_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_13` reader - DMA Request Event n"]
pub type DEV_13_R = crate::BitReader<bool>;
#[doc = "Field `DEV_13` writer - DMA Request Event n"]
pub type DEV_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_14` reader - DMA Request Event n"]
pub type DEV_14_R = crate::BitReader<bool>;
#[doc = "Field `DEV_14` writer - DMA Request Event n"]
pub type DEV_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DEV_15` reader - DMA Request Event n"]
pub type DEV_15_R = crate::BitReader<bool>;
#[doc = "Field `DEV_15` writer - DMA Request Event n"]
pub type DEV_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DRL1` reader - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
pub type DRL1_R = crate::BitReader<bool>;
#[doc = "Field `DRL1` writer - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
pub type DRL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DRQ1` reader - DMA Request 1 State"]
pub type DRQ1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_0(&self) -> DEV_0_R {
        DEV_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_1(&self) -> DEV_1_R {
        DEV_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_2(&self) -> DEV_2_R {
        DEV_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_3(&self) -> DEV_3_R {
        DEV_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_4(&self) -> DEV_4_R {
        DEV_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_5(&self) -> DEV_5_R {
        DEV_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_6(&self) -> DEV_6_R {
        DEV_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_7(&self) -> DEV_7_R {
        DEV_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_8(&self) -> DEV_8_R {
        DEV_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_9(&self) -> DEV_9_R {
        DEV_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_10(&self) -> DEV_10_R {
        DEV_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_11(&self) -> DEV_11_R {
        DEV_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_12(&self) -> DEV_12_R {
        DEV_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_13(&self) -> DEV_13_R {
        DEV_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_14(&self) -> DEV_14_R {
        DEV_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA Request Event n"]
    #[inline(always)]
    pub fn dev_15(&self) -> DEV_15_R {
        DEV_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    pub fn drl1(&self) -> DRL1_R {
        DRL1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Request 1 State"]
    #[inline(always)]
    pub fn drq1(&self) -> DRQ1_R {
        DRQ1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_0(&mut self) -> DEV_0_W<0> {
        DEV_0_W::new(self)
    }
    #[doc = "Bit 1 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_1(&mut self) -> DEV_1_W<1> {
        DEV_1_W::new(self)
    }
    #[doc = "Bit 2 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_2(&mut self) -> DEV_2_W<2> {
        DEV_2_W::new(self)
    }
    #[doc = "Bit 3 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_3(&mut self) -> DEV_3_W<3> {
        DEV_3_W::new(self)
    }
    #[doc = "Bit 4 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_4(&mut self) -> DEV_4_W<4> {
        DEV_4_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_5(&mut self) -> DEV_5_W<5> {
        DEV_5_W::new(self)
    }
    #[doc = "Bit 6 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_6(&mut self) -> DEV_6_W<6> {
        DEV_6_W::new(self)
    }
    #[doc = "Bit 7 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_7(&mut self) -> DEV_7_W<7> {
        DEV_7_W::new(self)
    }
    #[doc = "Bit 8 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_8(&mut self) -> DEV_8_W<8> {
        DEV_8_W::new(self)
    }
    #[doc = "Bit 9 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_9(&mut self) -> DEV_9_W<9> {
        DEV_9_W::new(self)
    }
    #[doc = "Bit 10 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_10(&mut self) -> DEV_10_W<10> {
        DEV_10_W::new(self)
    }
    #[doc = "Bit 11 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_11(&mut self) -> DEV_11_W<11> {
        DEV_11_W::new(self)
    }
    #[doc = "Bit 12 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_12(&mut self) -> DEV_12_W<12> {
        DEV_12_W::new(self)
    }
    #[doc = "Bit 13 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_13(&mut self) -> DEV_13_W<13> {
        DEV_13_W::new(self)
    }
    #[doc = "Bit 14 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_14(&mut self) -> DEV_14_W<14> {
        DEV_14_W::new(self)
    }
    #[doc = "Bit 15 - DMA Request Event n"]
    #[inline(always)]
    #[must_use]
    pub fn dev_15(&mut self) -> DEV_15_W<15> {
        DEV_15_W::new(self)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    #[must_use]
    pub fn drl1(&mut self) -> DRL1_W<30> {
        DRL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Request 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmareq1](index.html) module"]
pub struct DMAREQ1_SPEC;
impl crate::RegisterSpec for DMAREQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmareq1::R](R) reader structure"]
impl crate::Readable for DMAREQ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmareq1::W](W) writer structure"]
impl crate::Writable for DMAREQ1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAREQ1 to value 0"]
impl crate::Resettable for DMAREQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
