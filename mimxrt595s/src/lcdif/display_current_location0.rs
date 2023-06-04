#[doc = "Register `DisplayCurrentLocation0` reader"]
pub struct R(crate::R<DISPLAY_CURRENT_LOCATION0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISPLAY_CURRENT_LOCATION0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISPLAY_CURRENT_LOCATION0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISPLAY_CURRENT_LOCATION0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `X` reader - Current X location."]
pub type X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y` reader - Current Y location."]
pub type Y_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current X location."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current Y location."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Current x,y Location of Display Controller\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [display_current_location0](index.html) module"]
pub struct DISPLAY_CURRENT_LOCATION0_SPEC;
impl crate::RegisterSpec for DISPLAY_CURRENT_LOCATION0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [display_current_location0::R](R) reader structure"]
impl crate::Readable for DISPLAY_CURRENT_LOCATION0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DisplayCurrentLocation0 to value 0"]
impl crate::Resettable for DISPLAY_CURRENT_LOCATION0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
