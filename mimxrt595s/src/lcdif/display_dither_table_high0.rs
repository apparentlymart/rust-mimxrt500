#[doc = "Register `DisplayDitherTableHigh0` reader"]
pub struct R(crate::R<DISPLAY_DITHER_TABLE_HIGH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISPLAY_DITHER_TABLE_HIGH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISPLAY_DITHER_TABLE_HIGH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISPLAY_DITHER_TABLE_HIGH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DisplayDitherTableHigh0` writer"]
pub struct W(crate::W<DISPLAY_DITHER_TABLE_HIGH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISPLAY_DITHER_TABLE_HIGH0_SPEC>;
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
impl From<crate::W<DISPLAY_DITHER_TABLE_HIGH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISPLAY_DITHER_TABLE_HIGH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y2_X0` reader - Dither threshold value for x,y=0,2."]
pub type Y2_X0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y2_X0` writer - Dither threshold value for x,y=0,2."]
pub type Y2_X0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_HIGH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y2_X1` reader - Dither threshold value for x,y=1,2."]
pub type Y2_X1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y2_X1` writer - Dither threshold value for x,y=1,2."]
pub type Y2_X1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_HIGH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y2_X2` reader - Dither threshold value for x,y=2,2."]
pub type Y2_X2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y2_X2` writer - Dither threshold value for x,y=2,2."]
pub type Y2_X2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_HIGH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y2_X3` reader - Dither threshold value for x,y=3,2."]
pub type Y2_X3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y2_X3` writer - Dither threshold value for x,y=3,2."]
pub type Y2_X3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_HIGH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y3_X0` reader - Dither threshold value for x,y=0,3."]
pub type Y3_X0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y3_X0` writer - Dither threshold value for x,y=0,3."]
pub type Y3_X0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_HIGH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y3_X1` reader - Dither threshold value for x,y=1,3."]
pub type Y3_X1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y3_X1` writer - Dither threshold value for x,y=1,3."]
pub type Y3_X1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_HIGH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y3_X2` reader - Dither threshold value for x,y=2,3."]
pub type Y3_X2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y3_X2` writer - Dither threshold value for x,y=2,3."]
pub type Y3_X2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_HIGH0_SPEC, u8, u8, 4, O>;
#[doc = "Field `Y3_X3` reader - Dither threshold value for x,y=3,3."]
pub type Y3_X3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y3_X3` writer - Dither threshold value for x,y=3,3."]
pub type Y3_X3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_TABLE_HIGH0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Dither threshold value for x,y=0,2."]
    #[inline(always)]
    pub fn y2_x0(&self) -> Y2_X0_R {
        Y2_X0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Dither threshold value for x,y=1,2."]
    #[inline(always)]
    pub fn y2_x1(&self) -> Y2_X1_R {
        Y2_X1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Dither threshold value for x,y=2,2."]
    #[inline(always)]
    pub fn y2_x2(&self) -> Y2_X2_R {
        Y2_X2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Dither threshold value for x,y=3,2."]
    #[inline(always)]
    pub fn y2_x3(&self) -> Y2_X3_R {
        Y2_X3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Dither threshold value for x,y=0,3."]
    #[inline(always)]
    pub fn y3_x0(&self) -> Y3_X0_R {
        Y3_X0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Dither threshold value for x,y=1,3."]
    #[inline(always)]
    pub fn y3_x1(&self) -> Y3_X1_R {
        Y3_X1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Dither threshold value for x,y=2,3."]
    #[inline(always)]
    pub fn y3_x2(&self) -> Y3_X2_R {
        Y3_X2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Dither threshold value for x,y=3,3."]
    #[inline(always)]
    pub fn y3_x3(&self) -> Y3_X3_R {
        Y3_X3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dither threshold value for x,y=0,2."]
    #[inline(always)]
    #[must_use]
    pub fn y2_x0(&mut self) -> Y2_X0_W<0> {
        Y2_X0_W::new(self)
    }
    #[doc = "Bits 4:7 - Dither threshold value for x,y=1,2."]
    #[inline(always)]
    #[must_use]
    pub fn y2_x1(&mut self) -> Y2_X1_W<4> {
        Y2_X1_W::new(self)
    }
    #[doc = "Bits 8:11 - Dither threshold value for x,y=2,2."]
    #[inline(always)]
    #[must_use]
    pub fn y2_x2(&mut self) -> Y2_X2_W<8> {
        Y2_X2_W::new(self)
    }
    #[doc = "Bits 12:15 - Dither threshold value for x,y=3,2."]
    #[inline(always)]
    #[must_use]
    pub fn y2_x3(&mut self) -> Y2_X3_W<12> {
        Y2_X3_W::new(self)
    }
    #[doc = "Bits 16:19 - Dither threshold value for x,y=0,3."]
    #[inline(always)]
    #[must_use]
    pub fn y3_x0(&mut self) -> Y3_X0_W<16> {
        Y3_X0_W::new(self)
    }
    #[doc = "Bits 20:23 - Dither threshold value for x,y=1,3."]
    #[inline(always)]
    #[must_use]
    pub fn y3_x1(&mut self) -> Y3_X1_W<20> {
        Y3_X1_W::new(self)
    }
    #[doc = "Bits 24:27 - Dither threshold value for x,y=2,3."]
    #[inline(always)]
    #[must_use]
    pub fn y3_x2(&mut self) -> Y3_X2_W<24> {
        Y3_X2_W::new(self)
    }
    #[doc = "Bits 28:31 - Dither threshold value for x,y=3,3."]
    #[inline(always)]
    #[must_use]
    pub fn y3_x3(&mut self) -> Y3_X3_W<28> {
        Y3_X3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dither Table High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [display_dither_table_high0](index.html) module"]
pub struct DISPLAY_DITHER_TABLE_HIGH0_SPEC;
impl crate::RegisterSpec for DISPLAY_DITHER_TABLE_HIGH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [display_dither_table_high0::R](R) reader structure"]
impl crate::Readable for DISPLAY_DITHER_TABLE_HIGH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [display_dither_table_high0::W](W) writer structure"]
impl crate::Writable for DISPLAY_DITHER_TABLE_HIGH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DisplayDitherTableHigh0 to value 0"]
impl crate::Resettable for DISPLAY_DITHER_TABLE_HIGH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
