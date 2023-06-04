#[doc = "Register `SDYNADDR` reader"]
pub struct R(crate::R<SDYNADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDYNADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDYNADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDYNADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDYNADDR` writer"]
pub struct W(crate::W<SDYNADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDYNADDR_SPEC>;
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
impl From<crate::W<SDYNADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDYNADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAVALID` reader - DAVALID"]
pub type DAVALID_R = crate::BitReader<DAVALID_A>;
#[doc = "DAVALID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAVALID_A {
    #[doc = "0: DANOTASSIGNED"]
    DANOTASSIGNED = 0,
    #[doc = "1: DAASSIGNED"]
    DAASSIGNED = 1,
}
impl From<DAVALID_A> for bool {
    #[inline(always)]
    fn from(variant: DAVALID_A) -> Self {
        variant as u8 != 0
    }
}
impl DAVALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAVALID_A {
        match self.bits {
            false => DAVALID_A::DANOTASSIGNED,
            true => DAVALID_A::DAASSIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `DANOTASSIGNED`"]
    #[inline(always)]
    pub fn is_danotassigned(&self) -> bool {
        *self == DAVALID_A::DANOTASSIGNED
    }
    #[doc = "Checks if the value of the field is `DAASSIGNED`"]
    #[inline(always)]
    pub fn is_daassigned(&self) -> bool {
        *self == DAVALID_A::DAASSIGNED
    }
}
#[doc = "Field `DAVALID` writer - DAVALID"]
pub type DAVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDYNADDR_SPEC, DAVALID_A, O>;
impl<'a, const O: u8> DAVALID_W<'a, O> {
    #[doc = "DANOTASSIGNED"]
    #[inline(always)]
    pub fn danotassigned(self) -> &'a mut W {
        self.variant(DAVALID_A::DANOTASSIGNED)
    }
    #[doc = "DAASSIGNED"]
    #[inline(always)]
    pub fn daassigned(self) -> &'a mut W {
        self.variant(DAVALID_A::DAASSIGNED)
    }
}
#[doc = "Field `DADDR` reader - Dynamic address"]
pub type DADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DADDR` writer - Dynamic address"]
pub type DADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDYNADDR_SPEC, u8, u8, 7, O>;
#[doc = "Field `MAPIDX` writer - Mapped Dynamic Address"]
pub type MAPIDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDYNADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAPSA` writer - Map a Static Address"]
pub type MAPSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDYNADDR_SPEC, bool, O>;
#[doc = "Field `KEY` reader - Key"]
pub type KEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `KEY` writer - Key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDYNADDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - DAVALID"]
    #[inline(always)]
    pub fn davalid(&self) -> DAVALID_R {
        DAVALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Dynamic address"]
    #[inline(always)]
    pub fn daddr(&self) -> DADDR_R {
        DADDR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 16:31 - Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - DAVALID"]
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
    #[doc = "Bits 8:11 - Mapped Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn mapidx(&mut self) -> MAPIDX_W<8> {
        MAPIDX_W::new(self)
    }
    #[doc = "Bit 12 - Map a Static Address"]
    #[inline(always)]
    #[must_use]
    pub fn mapsa(&mut self) -> MAPSA_W<12> {
        MAPSA_W::new(self)
    }
    #[doc = "Bits 16:31 - Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<16> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Dynamic Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdynaddr](index.html) module"]
pub struct SDYNADDR_SPEC;
impl crate::RegisterSpec for SDYNADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdynaddr::R](R) reader structure"]
impl crate::Readable for SDYNADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdynaddr::W](W) writer structure"]
impl crate::Writable for SDYNADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDYNADDR to value 0"]
impl crate::Resettable for SDYNADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
