#[doc = "Register `DICEHWREGn` reader"]
pub struct R(crate::R<DICEHWREGN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DICEHWREGN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DICEHWREGN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DICEHWREGN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DICEHWREGn` writer"]
pub struct W(crate::W<DICEHWREGN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DICEHWREGN_SPEC>;
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
impl From<crate::W<DICEHWREGN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DICEHWREGN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DICEHWREGN` reader - DICE General Purpose 32-Bit Data Register"]
pub type DICEHWREGN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DICEHWREGN` writer - DICE General Purpose 32-Bit Data Register"]
pub type DICEHWREGN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DICEHWREGN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DICE General Purpose 32-Bit Data Register"]
    #[inline(always)]
    pub fn dicehwregn(&self) -> DICEHWREGN_R {
        DICEHWREGN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DICE General Purpose 32-Bit Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn dicehwregn(&mut self) -> DICEHWREGN_W<0> {
        DICEHWREGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compound Device Identifier (CDI)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dicehwregn](index.html) module"]
pub struct DICEHWREGN_SPEC;
impl crate::RegisterSpec for DICEHWREGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dicehwregn::R](R) reader structure"]
impl crate::Readable for DICEHWREGN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dicehwregn::W](W) writer structure"]
impl crate::Writable for DICEHWREGN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DICEHWREGn to value 0"]
impl crate::Resettable for DICEHWREGN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
