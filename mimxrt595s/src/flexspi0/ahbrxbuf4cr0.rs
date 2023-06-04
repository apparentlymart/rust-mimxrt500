#[doc = "Register `AHBRXBUF4CR0` reader"]
pub struct R(crate::R<AHBRXBUF4CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRXBUF4CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRXBUF4CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRXBUF4CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBRXBUF4CR0` writer"]
pub struct W(crate::W<AHBRXBUF4CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRXBUF4CR0_SPEC>;
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
impl From<crate::W<AHBRXBUF4CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRXBUF4CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFSZ` reader - AHB RX Buffer Size in 64 bits."]
pub type BUFSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUFSZ` writer - AHB RX Buffer Size in 64 bits."]
pub type BUFSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBRXBUF4CR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `MSTRID` reader - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
pub type MSTRID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSTRID` writer - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
pub type MSTRID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBRXBUF4CR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRIORITY` reader - This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
pub type PRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY` writer - This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
pub type PRIORITY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBRXBUF4CR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `PREFETCHEN` reader - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
pub type PREFETCHEN_R = crate::BitReader<PREFETCHEN_A>;
#[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREFETCHEN_A {
    #[doc = "0: No prefetch"]
    VALUE0 = 0,
    #[doc = "1: Prefetch enable"]
    VALUE1 = 1,
}
impl From<PREFETCHEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREFETCHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PREFETCHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFETCHEN_A {
        match self.bits {
            false => PREFETCHEN_A::VALUE0,
            true => PREFETCHEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == PREFETCHEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PREFETCHEN_A::VALUE1
    }
}
#[doc = "Field `PREFETCHEN` writer - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
pub type PREFETCHEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBRXBUF4CR0_SPEC, PREFETCHEN_A, O>;
impl<'a, const O: u8> PREFETCHEN_W<'a, O> {
    #[doc = "No prefetch"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(PREFETCHEN_A::VALUE0)
    }
    #[doc = "Prefetch enable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PREFETCHEN_A::VALUE1)
    }
}
impl R {
    #[doc = "Bits 0:7 - AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn bufsz(&self) -> BUFSZ_R {
        BUFSZ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn mstrid(&self) -> MSTRID_R {
        MSTRID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn prefetchen(&self) -> PREFETCHEN_R {
        PREFETCHEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    #[must_use]
    pub fn bufsz(&mut self) -> BUFSZ_W<0> {
        BUFSZ_W::new(self)
    }
    #[doc = "Bits 16:19 - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    #[must_use]
    pub fn mstrid(&mut self) -> MSTRID_W<16> {
        MSTRID_W::new(self)
    }
    #[doc = "Bits 24:26 - This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<24> {
        PRIORITY_W::new(self)
    }
    #[doc = "Bit 31 - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    #[must_use]
    pub fn prefetchen(&mut self) -> PREFETCHEN_W<31> {
        PREFETCHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB RX Buffer 4 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrxbuf4cr0](index.html) module"]
pub struct AHBRXBUF4CR0_SPEC;
impl crate::RegisterSpec for AHBRXBUF4CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbrxbuf4cr0::R](R) reader structure"]
impl crate::Readable for AHBRXBUF4CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbrxbuf4cr0::W](W) writer structure"]
impl crate::Writable for AHBRXBUF4CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRXBUF4CR0 to value 0x8004_0010"]
impl crate::Resettable for AHBRXBUF4CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8004_0010;
}
