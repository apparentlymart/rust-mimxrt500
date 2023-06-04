#[doc = "Register `MIBIRULES` reader"]
pub struct R(crate::R<MIBIRULES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIBIRULES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIBIRULES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIBIRULES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIBIRULES` writer"]
pub struct W(crate::W<MIBIRULES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIBIRULES_SPEC>;
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
impl From<crate::W<MIBIRULES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIBIRULES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR0` reader - ADDR0"]
pub type ADDR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR0` writer - ADDR0"]
pub type ADDR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIBIRULES_SPEC, u8, u8, 6, O>;
#[doc = "Field `ADDR1` reader - ADDR1"]
pub type ADDR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR1` writer - ADDR1"]
pub type ADDR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIBIRULES_SPEC, u8, u8, 6, O>;
#[doc = "Field `ADDR2` reader - ADDR2"]
pub type ADDR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR2` writer - ADDR2"]
pub type ADDR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIBIRULES_SPEC, u8, u8, 6, O>;
#[doc = "Field `ADDR3` reader - ADDR3"]
pub type ADDR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR3` writer - ADDR3"]
pub type ADDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIBIRULES_SPEC, u8, u8, 6, O>;
#[doc = "Field `ADDR4` reader - ADDR4"]
pub type ADDR4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR4` writer - ADDR4"]
pub type ADDR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIBIRULES_SPEC, u8, u8, 6, O>;
#[doc = "Field `MSB0` reader - Set Most Significant address Bit to 0"]
pub type MSB0_R = crate::BitReader<bool>;
#[doc = "Field `MSB0` writer - Set Most Significant address Bit to 0"]
pub type MSB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIBIRULES_SPEC, bool, O>;
#[doc = "Field `NOBYTE` reader - No IBI byte"]
pub type NOBYTE_R = crate::BitReader<bool>;
#[doc = "Field `NOBYTE` writer - No IBI byte"]
pub type NOBYTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIBIRULES_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - ADDR0"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - ADDR1"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - ADDR2"]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - ADDR3"]
    #[inline(always)]
    pub fn addr3(&self) -> ADDR3_R {
        ADDR3_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - ADDR4"]
    #[inline(always)]
    pub fn addr4(&self) -> ADDR4_R {
        ADDR4_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Set Most Significant address Bit to 0"]
    #[inline(always)]
    pub fn msb0(&self) -> MSB0_R {
        MSB0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - No IBI byte"]
    #[inline(always)]
    pub fn nobyte(&self) -> NOBYTE_R {
        NOBYTE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - ADDR0"]
    #[inline(always)]
    #[must_use]
    pub fn addr0(&mut self) -> ADDR0_W<0> {
        ADDR0_W::new(self)
    }
    #[doc = "Bits 6:11 - ADDR1"]
    #[inline(always)]
    #[must_use]
    pub fn addr1(&mut self) -> ADDR1_W<6> {
        ADDR1_W::new(self)
    }
    #[doc = "Bits 12:17 - ADDR2"]
    #[inline(always)]
    #[must_use]
    pub fn addr2(&mut self) -> ADDR2_W<12> {
        ADDR2_W::new(self)
    }
    #[doc = "Bits 18:23 - ADDR3"]
    #[inline(always)]
    #[must_use]
    pub fn addr3(&mut self) -> ADDR3_W<18> {
        ADDR3_W::new(self)
    }
    #[doc = "Bits 24:29 - ADDR4"]
    #[inline(always)]
    #[must_use]
    pub fn addr4(&mut self) -> ADDR4_W<24> {
        ADDR4_W::new(self)
    }
    #[doc = "Bit 30 - Set Most Significant address Bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn msb0(&mut self) -> MSB0_W<30> {
        MSB0_W::new(self)
    }
    #[doc = "Bit 31 - No IBI byte"]
    #[inline(always)]
    #[must_use]
    pub fn nobyte(&mut self) -> NOBYTE_W<31> {
        NOBYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master In-band Interrupt Registry and Rules Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mibirules](index.html) module"]
pub struct MIBIRULES_SPEC;
impl crate::RegisterSpec for MIBIRULES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mibirules::R](R) reader structure"]
impl crate::Readable for MIBIRULES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mibirules::W](W) writer structure"]
impl crate::Writable for MIBIRULES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIBIRULES to value 0"]
impl crate::Resettable for MIBIRULES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
