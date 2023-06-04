#[doc = "Register `AHBMATRIXPRIOR` reader"]
pub struct R(crate::R<AHBMATRIXPRIOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBMATRIXPRIOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBMATRIXPRIOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBMATRIXPRIOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBMATRIXPRIOR` writer"]
pub struct W(crate::W<AHBMATRIXPRIOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBMATRIXPRIOR_SPEC>;
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
impl From<crate::W<AHBMATRIXPRIOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBMATRIXPRIOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M0` reader - Master 0 Priority"]
pub type M0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M0` writer - Master 0 Priority"]
pub type M0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M1` reader - Master 1 Priority"]
pub type M1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M1` writer - Master 1 Priority"]
pub type M1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M2` reader - Master 2 Priority"]
pub type M2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M2` writer - Master 2 Priority"]
pub type M2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M3` reader - Master 3 Priority"]
pub type M3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M3` writer - Master 3 Priority"]
pub type M3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M4` reader - Master 4 Priority"]
pub type M4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M4` writer - Master 4 Priority"]
pub type M4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M5` reader - Master 5 Priority"]
pub type M5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M5` writer - Master 5 Priority"]
pub type M5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M6` reader - Master 6 Priority"]
pub type M6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M6` writer - Master 6 Priority"]
pub type M6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M7` reader - Master 7 Priority"]
pub type M7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M7` writer - Master 7 Priority"]
pub type M7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M8` reader - Master 8 Priority"]
pub type M8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M8` writer - Master 8 Priority"]
pub type M8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M9` reader - Master 9 Priority"]
pub type M9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M9` writer - Master 9 Priority"]
pub type M9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M10` reader - Master 10 Priority"]
pub type M10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M10` writer - Master 10 Priority"]
pub type M10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `M11` reader - Master 10 Priority"]
pub type M11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M11` writer - Master 10 Priority"]
pub type M11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBMATRIXPRIOR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2(&self) -> M2_R {
        M2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3(&self) -> M3_R {
        M3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4(&self) -> M4_R {
        M4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Master 5 Priority"]
    #[inline(always)]
    pub fn m5(&self) -> M5_R {
        M5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Master 6 Priority"]
    #[inline(always)]
    pub fn m6(&self) -> M6_R {
        M6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Master 7 Priority"]
    #[inline(always)]
    pub fn m7(&self) -> M7_R {
        M7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8(&self) -> M8_R {
        M8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9(&self) -> M9_R {
        M9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10(&self) -> M10_R {
        M10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Master 10 Priority"]
    #[inline(always)]
    pub fn m11(&self) -> M11_R {
        M11_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<0> {
        M0_W::new(self)
    }
    #[doc = "Bits 2:3 - Master 1 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<2> {
        M1_W::new(self)
    }
    #[doc = "Bits 4:5 - Master 2 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m2(&mut self) -> M2_W<4> {
        M2_W::new(self)
    }
    #[doc = "Bits 6:7 - Master 3 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m3(&mut self) -> M3_W<6> {
        M3_W::new(self)
    }
    #[doc = "Bits 8:9 - Master 4 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m4(&mut self) -> M4_W<8> {
        M4_W::new(self)
    }
    #[doc = "Bits 10:11 - Master 5 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m5(&mut self) -> M5_W<10> {
        M5_W::new(self)
    }
    #[doc = "Bits 12:13 - Master 6 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m6(&mut self) -> M6_W<12> {
        M6_W::new(self)
    }
    #[doc = "Bits 14:15 - Master 7 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m7(&mut self) -> M7_W<14> {
        M7_W::new(self)
    }
    #[doc = "Bits 16:17 - Master 8 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m8(&mut self) -> M8_W<16> {
        M8_W::new(self)
    }
    #[doc = "Bits 18:19 - Master 9 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m9(&mut self) -> M9_W<18> {
        M9_W::new(self)
    }
    #[doc = "Bits 20:21 - Master 10 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m10(&mut self) -> M10_W<20> {
        M10_W::new(self)
    }
    #[doc = "Bits 22:23 - Master 10 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m11(&mut self) -> M11_W<22> {
        M11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB MAX Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbmatrixprior](index.html) module"]
pub struct AHBMATRIXPRIOR_SPEC;
impl crate::RegisterSpec for AHBMATRIXPRIOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbmatrixprior::R](R) reader structure"]
impl crate::Readable for AHBMATRIXPRIOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbmatrixprior::W](W) writer structure"]
impl crate::Writable for AHBMATRIXPRIOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBMATRIXPRIOR to value 0"]
impl crate::Resettable for AHBMATRIXPRIOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
