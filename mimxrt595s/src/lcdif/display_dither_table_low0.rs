#[doc = "Register `DisplayDitherTableLow0` reader"]
pub struct R(crate::R<DISPLAY_DITHER_TABLE_LOW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISPLAY_DITHER_TABLE_LOW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISPLAY_DITHER_TABLE_LOW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISPLAY_DITHER_TABLE_LOW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DisplayDitherTableLow0` writer"]
pub struct W(crate::W<DISPLAY_DITHER_TABLE_LOW0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISPLAY_DITHER_TABLE_LOW0_SPEC>;
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
impl From<crate::W<DISPLAY_DITHER_TABLE_LOW0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISPLAY_DITHER_TABLE_LOW0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y0_X0` reader - Dither threshold value for x,y=0,0."]
pub type Y0_X0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y0_X0` writer - Dither threshold value for x,y=0,0."]
pub type Y0_X0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_LOW0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y0_X1` reader - Dither threshold value for x,y=1,0."]
pub type Y0_X1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y0_X1` writer - Dither threshold value for x,y=1,0."]
pub type Y0_X1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_LOW0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y0_X2` reader - Dither threshold value for x,y=2,0."]
pub type Y0_X2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y0_X2` writer - Dither threshold value for x,y=2,0."]
pub type Y0_X2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_LOW0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y0_X3` reader - Dither threshold value for x,y=3,0."]
pub type Y0_X3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y0_X3` writer - Dither threshold value for x,y=3,0."]
pub type Y0_X3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_LOW0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y1_X0` reader - Dither threshold value for x,y=0,1."]
pub type Y1_X0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y1_X0` writer - Dither threshold value for x,y=0,1."]
pub type Y1_X0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_LOW0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y1_X1` reader - Dither threshold value for x,y=1,1."]
pub type Y1_X1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y1_X1` writer - Dither threshold value for x,y=1,1."]
pub type Y1_X1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_LOW0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y1_X2` reader - Dither threshold value for x,y=2,1."]
pub type Y1_X2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y1_X2` writer - Dither threshold value for x,y=2,1."]
pub type Y1_X2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_LOW0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y1_X3` reader - Dither threshold value for x,y=3,1."]
pub type Y1_X3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y1_X3` writer - Dither threshold value for x,y=3,1."]
pub type Y1_X3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_LOW0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Dither threshold value for x,y=0,0."]
    #[inline(always)]
    pub fn y0_x0(&self) -> Y0_X0_R {
        Y0_X0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Dither threshold value for x,y=1,0."]
    #[inline(always)]
    pub fn y0_x1(&self) -> Y0_X1_R {
        Y0_X1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Dither threshold value for x,y=2,0."]
    #[inline(always)]
    pub fn y0_x2(&self) -> Y0_X2_R {
        Y0_X2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Dither threshold value for x,y=3,0."]
    #[inline(always)]
    pub fn y0_x3(&self) -> Y0_X3_R {
        Y0_X3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Dither threshold value for x,y=0,1."]
    #[inline(always)]
    pub fn y1_x0(&self) -> Y1_X0_R {
        Y1_X0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Dither threshold value for x,y=1,1."]
    #[inline(always)]
    pub fn y1_x1(&self) -> Y1_X1_R {
        Y1_X1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Dither threshold value for x,y=2,1."]
    #[inline(always)]
    pub fn y1_x2(&self) -> Y1_X2_R {
        Y1_X2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Dither threshold value for x,y=3,1."]
    #[inline(always)]
    pub fn y1_x3(&self) -> Y1_X3_R {
        Y1_X3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dither threshold value for x,y=0,0."]
    #[inline(always)]
    #[must_use]
    pub fn y0_x0(&mut self) -> Y0_X0_W<0> {
        Y0_X0_W::new(self)
    }
    #[doc = "Bits 4:7 - Dither threshold value for x,y=1,0."]
    #[inline(always)]
    #[must_use]
    pub fn y0_x1(&mut self) -> Y0_X1_W<4> {
        Y0_X1_W::new(self)
    }
    #[doc = "Bits 8:11 - Dither threshold value for x,y=2,0."]
    #[inline(always)]
    #[must_use]
    pub fn y0_x2(&mut self) -> Y0_X2_W<8> {
        Y0_X2_W::new(self)
    }
    #[doc = "Bits 12:15 - Dither threshold value for x,y=3,0."]
    #[inline(always)]
    #[must_use]
    pub fn y0_x3(&mut self) -> Y0_X3_W<12> {
        Y0_X3_W::new(self)
    }
    #[doc = "Bits 16:19 - Dither threshold value for x,y=0,1."]
    #[inline(always)]
    #[must_use]
    pub fn y1_x0(&mut self) -> Y1_X0_W<16> {
        Y1_X0_W::new(self)
    }
    #[doc = "Bits 20:23 - Dither threshold value for x,y=1,1."]
    #[inline(always)]
    #[must_use]
    pub fn y1_x1(&mut self) -> Y1_X1_W<20> {
        Y1_X1_W::new(self)
    }
    #[doc = "Bits 24:27 - Dither threshold value for x,y=2,1."]
    #[inline(always)]
    #[must_use]
    pub fn y1_x2(&mut self) -> Y1_X2_W<24> {
        Y1_X2_W::new(self)
    }
    #[doc = "Bits 28:31 - Dither threshold value for x,y=3,1."]
    #[inline(always)]
    #[must_use]
    pub fn y1_x3(&mut self) -> Y1_X3_W<28> {
        Y1_X3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dither Table Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [display_dither_table_low0](index.html) module"]
pub struct DISPLAY_DITHER_TABLE_LOW0_SPEC;
impl crate::RegisterSpec for DISPLAY_DITHER_TABLE_LOW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [display_dither_table_low0::R](R) reader structure"]
impl crate::Readable for DISPLAY_DITHER_TABLE_LOW0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [display_dither_table_low0::W](W) writer structure"]
impl crate::Writable for DISPLAY_DITHER_TABLE_LOW0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DisplayDitherTableLow0 to value 0"]
impl crate::Resettable for DISPLAY_DITHER_TABLE_LOW0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
