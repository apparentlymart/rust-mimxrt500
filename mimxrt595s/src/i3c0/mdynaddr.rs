#[doc = "Register `MDYNADDR` reader"]
pub struct R(crate::R<MDYNADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDYNADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDYNADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDYNADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDYNADDR` writer"]
pub struct W(crate::W<MDYNADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDYNADDR_SPEC>;
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
impl From<crate::W<MDYNADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDYNADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAVALID` reader - Dynamic address valid"]
pub type DAVALID_R = crate::BitReader<bool>;
#[doc = "Field `DAVALID` writer - Dynamic address valid"]
pub type DAVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDYNADDR_SPEC, bool, O>;
#[doc = "Field `DADDR` reader - Dynamic address"]
pub type DADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DADDR` writer - Dynamic address"]
pub type DADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDYNADDR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Dynamic address valid"]
    #[inline(always)]
    pub fn davalid(&self) -> DAVALID_R {
        DAVALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Dynamic address"]
    #[inline(always)]
    pub fn daddr(&self) -> DADDR_R {
        DADDR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dynamic address valid"]
    #[inline(always)]
    #[must_use]
    pub fn davalid(&mut self) -> DAVALID_W<0> {
        DAVALID_W::new(self)
    }
    #[doc = "Bits 1:7 - Dynamic address"]
    #[inline(always)]
    #[must_use]
    pub fn daddr(&mut self) -> DADDR_W<1> {
        DADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Dynamic Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdynaddr](index.html) module"]
pub struct MDYNADDR_SPEC;
impl crate::RegisterSpec for MDYNADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdynaddr::R](R) reader structure"]
impl crate::Readable for MDYNADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdynaddr::W](W) writer structure"]
impl crate::Writable for MDYNADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDYNADDR to value 0"]
impl crate::Resettable for MDYNADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
