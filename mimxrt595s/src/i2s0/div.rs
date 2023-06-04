#[doc = "Register `DIV` reader"]
pub struct R(crate::R<DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV` writer"]
pub struct W(crate::W<DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_SPEC>;
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
impl From<crate::W<DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Divider"]
pub type DIV_R = crate::FieldReader<u16, DIV_A>;
#[doc = "Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DIV_A {
    #[doc = "0: FCLK is used directly."]
    DIV_1 = 0,
    #[doc = "1: FCLK is divided by 2."]
    DIV_2 = 1,
    #[doc = "2: FCLK is divided by 3."]
    DIV_3 = 2,
    #[doc = "4095: FCLK is divided by 4,096."]
    DIV_4096 = 4095,
}
impl From<DIV_A> for u16 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIV_A> {
        match self.bits {
            0 => Some(DIV_A::DIV_1),
            1 => Some(DIV_A::DIV_2),
            2 => Some(DIV_A::DIV_3),
            4095 => Some(DIV_A::DIV_4096),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV_1`"]
    #[inline(always)]
    pub fn is_div_1(&self) -> bool {
        *self == DIV_A::DIV_1
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == DIV_A::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_3`"]
    #[inline(always)]
    pub fn is_div_3(&self) -> bool {
        *self == DIV_A::DIV_3
    }
    #[doc = "Checks if the value of the field is `DIV_4096`"]
    #[inline(always)]
    pub fn is_div_4096(&self) -> bool {
        *self == DIV_A::DIV_4096
    }
}
#[doc = "Field `DIV` writer - Divider"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIV_SPEC, u16, DIV_A, 12, O>;
impl<'a, const O: u8> DIV_W<'a, O> {
    #[doc = "FCLK is used directly."]
    #[inline(always)]
    pub fn div_1(self) -> &'a mut W {
        self.variant(DIV_A::DIV_1)
    }
    #[doc = "FCLK is divided by 2."]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut W {
        self.variant(DIV_A::DIV_2)
    }
    #[doc = "FCLK is divided by 3."]
    #[inline(always)]
    pub fn div_3(self) -> &'a mut W {
        self.variant(DIV_A::DIV_3)
    }
    #[doc = "FCLK is divided by 4,096."]
    #[inline(always)]
    pub fn div_4096(self) -> &'a mut W {
        self.variant(DIV_A::DIV_4096)
    }
}
impl R {
    #[doc = "Bits 0:11 - Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div](index.html) module"]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div::R](R) reader structure"]
impl crate::Readable for DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div::W](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
