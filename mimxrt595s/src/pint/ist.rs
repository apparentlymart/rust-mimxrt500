#[doc = "Register `IST` reader"]
pub struct R(crate::R<IST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IST` writer"]
pub struct W(crate::W<IST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IST_SPEC>;
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
impl From<crate::W<IST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSTAT` reader - Pin interrupt status"]
pub type PSTAT_R = crate::FieldReader<u8, PSTAT_A>;
#[doc = "Pin interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSTAT_A {
    #[doc = "0: Read 0- interrupt is not being requested for this pin, Write 0- no operation."]
    PSTAT_0 = 0,
    #[doc = "1: Read 1- interrupt is being requested for this pin, Write 1 (edge-sensitive)- clear rising- and falling-edge detection for this pin, Write 1 (level-sensitive)- switch the active level for this pin (in the IENF register)."]
    PSTAT_1 = 1,
}
impl From<PSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSTAT_A) -> Self {
        variant as _
    }
}
impl PSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSTAT_A> {
        match self.bits {
            0 => Some(PSTAT_A::PSTAT_0),
            1 => Some(PSTAT_A::PSTAT_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PSTAT_0`"]
    #[inline(always)]
    pub fn is_pstat_0(&self) -> bool {
        *self == PSTAT_A::PSTAT_0
    }
    #[doc = "Checks if the value of the field is `PSTAT_1`"]
    #[inline(always)]
    pub fn is_pstat_1(&self) -> bool {
        *self == PSTAT_A::PSTAT_1
    }
}
#[doc = "Field `PSTAT` writer - Pin interrupt status"]
pub type PSTAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IST_SPEC, u8, PSTAT_A, 8, O>;
impl<'a, const O: u8> PSTAT_W<'a, O> {
    #[doc = "Read 0- interrupt is not being requested for this pin, Write 0- no operation."]
    #[inline(always)]
    pub fn pstat_0(self) -> &'a mut W {
        self.variant(PSTAT_A::PSTAT_0)
    }
    #[doc = "Read 1- interrupt is being requested for this pin, Write 1 (edge-sensitive)- clear rising- and falling-edge detection for this pin, Write 1 (level-sensitive)- switch the active level for this pin (in the IENF register)."]
    #[inline(always)]
    pub fn pstat_1(self) -> &'a mut W {
        self.variant(PSTAT_A::PSTAT_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Pin interrupt status"]
    #[inline(always)]
    pub fn pstat(&self) -> PSTAT_R {
        PSTAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn pstat(&mut self) -> PSTAT_W<0> {
        PSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ist](index.html) module"]
pub struct IST_SPEC;
impl crate::RegisterSpec for IST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ist::R](R) reader structure"]
impl crate::Readable for IST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ist::W](W) writer structure"]
impl crate::Writable for IST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IST to value 0"]
impl crate::Resettable for IST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
