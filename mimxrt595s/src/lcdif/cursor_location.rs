#[doc = "Register `CursorLocation` reader"]
pub struct R(crate::R<CURSOR_LOCATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURSOR_LOCATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURSOR_LOCATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURSOR_LOCATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CursorLocation` writer"]
pub struct W(crate::W<CURSOR_LOCATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CURSOR_LOCATION_SPEC>;
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
impl From<crate::W<CURSOR_LOCATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CURSOR_LOCATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X` reader - X location of cursor's hotspot."]
pub type X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `X` writer - X location of cursor's hotspot."]
pub type X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CURSOR_LOCATION_SPEC, u16, u16, 13, O>;
#[doc = "Field `Y` reader - Y location of cursor's hotspot."]
pub type Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y` writer - Y location of cursor's hotspot."]
pub type Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CURSOR_LOCATION_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:12 - X location of cursor's hotspot."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:27 - Y location of cursor's hotspot."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - X location of cursor's hotspot."]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<0> {
        X_W::new(self)
    }
    #[doc = "Bits 16:27 - Y location of cursor's hotspot."]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> Y_W<16> {
        Y_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Location of the cursor on the owning display\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cursor_location](index.html) module"]
pub struct CURSOR_LOCATION_SPEC;
impl crate::RegisterSpec for CURSOR_LOCATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cursor_location::R](R) reader structure"]
impl crate::Readable for CURSOR_LOCATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cursor_location::W](W) writer structure"]
impl crate::Writable for CURSOR_LOCATION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CursorLocation to value 0"]
impl crate::Resettable for CURSOR_LOCATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
