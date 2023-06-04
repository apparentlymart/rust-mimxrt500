#[doc = "Register `GammaData0` reader"]
pub struct R(crate::R<GAMMA_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAMMA_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAMMA_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAMMA_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GammaData0` writer"]
pub struct W(crate::W<GAMMA_DATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAMMA_DATA0_SPEC>;
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
impl From<crate::W<GAMMA_DATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAMMA_DATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLUE` reader - Blue translation value."]
pub type BLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLUE` writer - Blue translation value."]
pub type BLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAMMA_DATA0_SPEC, u8, u8, 8, O>;
#[doc = "Field `GREEN` reader - Green translation value."]
pub type GREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GREEN` writer - Green translation value."]
pub type GREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAMMA_DATA0_SPEC, u8, u8, 8, O>;
#[doc = "Field `RED` reader - Red translation value."]
pub type RED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RED` writer - Red translation value."]
pub type RED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAMMA_DATA0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Blue translation value."]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green translation value."]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Red translation value."]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blue translation value."]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    #[doc = "Bits 8:15 - Green translation value."]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<8> {
        GREEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Red translation value."]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<16> {
        RED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Translation Values for the Gamma Table\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gamma_data0](index.html) module"]
pub struct GAMMA_DATA0_SPEC;
impl crate::RegisterSpec for GAMMA_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gamma_data0::R](R) reader structure"]
impl crate::Readable for GAMMA_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gamma_data0::W](W) writer structure"]
impl crate::Writable for GAMMA_DATA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GammaData0 to value 0"]
impl crate::Resettable for GAMMA_DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
