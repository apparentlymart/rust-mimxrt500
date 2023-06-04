#[doc = "Register `SCONFIG` reader"]
pub struct R(crate::R<SCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCONFIG` writer"]
pub struct W(crate::W<SCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCONFIG_SPEC>;
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
impl From<crate::W<SCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLVENA` reader - Slave enable"]
pub type SLVENA_R = crate::BitReader<bool>;
#[doc = "Field `SLVENA` writer - Slave enable"]
pub type SLVENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCONFIG_SPEC, bool, O>;
#[doc = "Field `NACK` reader - Not acknowledge"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `NACK` writer - Not acknowledge"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCONFIG_SPEC, bool, O>;
#[doc = "Field `MATCHSS` reader - Match START or STOP"]
pub type MATCHSS_R = crate::BitReader<bool>;
#[doc = "Field `MATCHSS` writer - Match START or STOP"]
pub type MATCHSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCONFIG_SPEC, bool, O>;
#[doc = "Field `S0IGNORE` reader - S0/S1 errors ignore"]
pub type S0IGNORE_R = crate::BitReader<bool>;
#[doc = "Field `S0IGNORE` writer - S0/S1 errors ignore"]
pub type S0IGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCONFIG_SPEC, bool, O>;
#[doc = "Field `DDROK` reader - Double Data Rate OK"]
pub type DDROK_R = crate::BitReader<bool>;
#[doc = "Field `DDROK` writer - Double Data Rate OK"]
pub type DDROK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCONFIG_SPEC, bool, O>;
#[doc = "Field `IDRAND` reader - ID random"]
pub type IDRAND_R = crate::BitReader<bool>;
#[doc = "Field `IDRAND` writer - ID random"]
pub type IDRAND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCONFIG_SPEC, bool, O>;
#[doc = "Field `OFFLINE` reader - Offline"]
pub type OFFLINE_R = crate::BitReader<bool>;
#[doc = "Field `OFFLINE` writer - Offline"]
pub type OFFLINE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCONFIG_SPEC, bool, O>;
#[doc = "Field `BAMATCH` reader - Bus available match"]
pub type BAMATCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAMATCH` writer - Bus available match"]
pub type BAMATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `SADDR` reader - Static address"]
pub type SADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SADDR` writer - Static address"]
pub type SADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCONFIG_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Slave enable"]
    #[inline(always)]
    pub fn slvena(&self) -> SLVENA_R {
        SLVENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Not acknowledge"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Match START or STOP"]
    #[inline(always)]
    pub fn matchss(&self) -> MATCHSS_R {
        MATCHSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - S0/S1 errors ignore"]
    #[inline(always)]
    pub fn s0ignore(&self) -> S0IGNORE_R {
        S0IGNORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Double Data Rate OK"]
    #[inline(always)]
    pub fn ddrok(&self) -> DDROK_R {
        DDROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - ID random"]
    #[inline(always)]
    pub fn idrand(&self) -> IDRAND_R {
        IDRAND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Offline"]
    #[inline(always)]
    pub fn offline(&self) -> OFFLINE_R {
        OFFLINE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bus available match"]
    #[inline(always)]
    pub fn bamatch(&self) -> BAMATCH_R {
        BAMATCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 25:31 - Static address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Slave enable"]
    #[inline(always)]
    #[must_use]
    pub fn slvena(&mut self) -> SLVENA_W<0> {
        SLVENA_W::new(self)
    }
    #[doc = "Bit 1 - Not acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<1> {
        NACK_W::new(self)
    }
    #[doc = "Bit 2 - Match START or STOP"]
    #[inline(always)]
    #[must_use]
    pub fn matchss(&mut self) -> MATCHSS_W<2> {
        MATCHSS_W::new(self)
    }
    #[doc = "Bit 3 - S0/S1 errors ignore"]
    #[inline(always)]
    #[must_use]
    pub fn s0ignore(&mut self) -> S0IGNORE_W<3> {
        S0IGNORE_W::new(self)
    }
    #[doc = "Bit 4 - Double Data Rate OK"]
    #[inline(always)]
    #[must_use]
    pub fn ddrok(&mut self) -> DDROK_W<4> {
        DDROK_W::new(self)
    }
    #[doc = "Bit 8 - ID random"]
    #[inline(always)]
    #[must_use]
    pub fn idrand(&mut self) -> IDRAND_W<8> {
        IDRAND_W::new(self)
    }
    #[doc = "Bit 9 - Offline"]
    #[inline(always)]
    #[must_use]
    pub fn offline(&mut self) -> OFFLINE_W<9> {
        OFFLINE_W::new(self)
    }
    #[doc = "Bits 16:23 - Bus available match"]
    #[inline(always)]
    #[must_use]
    pub fn bamatch(&mut self) -> BAMATCH_W<16> {
        BAMATCH_W::new(self)
    }
    #[doc = "Bits 25:31 - Static address"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SADDR_W<25> {
        SADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sconfig](index.html) module"]
pub struct SCONFIG_SPEC;
impl crate::RegisterSpec for SCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sconfig::R](R) reader structure"]
impl crate::Readable for SCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sconfig::W](W) writer structure"]
impl crate::Writable for SCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCONFIG to value 0"]
impl crate::Resettable for SCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
