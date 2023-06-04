#[doc = "Register `CSAR` reader"]
pub struct R(crate::R<CSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSAR` writer"]
pub struct W(crate::W<CSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSAR_SPEC>;
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
impl From<crate::W<CSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LGO` reader - Initiate Cache Line Command"]
pub type LGO_R = crate::BitReader<LGO_A>;
#[doc = "Initiate Cache Line Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LGO_A {
    #[doc = "0: Write: no effect; Read: no line command active"]
    NO_EFFECT = 0,
    #[doc = "1: Write: initiate line command; Read: line command active"]
    INIT_CMD = 1,
}
impl From<LGO_A> for bool {
    #[inline(always)]
    fn from(variant: LGO_A) -> Self {
        variant as u8 != 0
    }
}
impl LGO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LGO_A {
        match self.bits {
            false => LGO_A::NO_EFFECT,
            true => LGO_A::INIT_CMD,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LGO_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `INIT_CMD`"]
    #[inline(always)]
    pub fn is_init_cmd(&self) -> bool {
        *self == LGO_A::INIT_CMD
    }
}
#[doc = "Field `LGO` writer - Initiate Cache Line Command"]
pub type LGO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSAR_SPEC, LGO_A, O>;
impl<'a, const O: u8> LGO_W<'a, O> {
    #[doc = "Write: no effect; Read: no line command active"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(LGO_A::NO_EFFECT)
    }
    #[doc = "Write: initiate line command; Read: line command active"]
    #[inline(always)]
    pub fn init_cmd(self) -> &'a mut W {
        self.variant(LGO_A::INIT_CMD)
    }
}
#[doc = "Field `PHYADDR27_1` reader - Physical Address"]
pub type PHYADDR27_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PHYADDR27_1` writer - Physical Address"]
pub type PHYADDR27_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSAR_SPEC, u32, u32, 27, O>;
#[doc = "Field `PHYADDR31_29` reader - Physical Address"]
pub type PHYADDR31_29_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYADDR31_29` writer - Physical Address"]
pub type PHYADDR31_29_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSAR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&self) -> LGO_R {
        LGO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:27 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr27_1(&self) -> PHYADDR27_1_R {
        PHYADDR27_1_R::new((self.bits >> 1) & 0x07ff_ffff)
    }
    #[doc = "Bits 29:31 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr31_29(&self) -> PHYADDR31_29_R {
        PHYADDR31_29_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    #[must_use]
    pub fn lgo(&mut self) -> LGO_W<0> {
        LGO_W::new(self)
    }
    #[doc = "Bits 1:27 - Physical Address"]
    #[inline(always)]
    #[must_use]
    pub fn phyaddr27_1(&mut self) -> PHYADDR27_1_W<1> {
        PHYADDR27_1_W::new(self)
    }
    #[doc = "Bits 29:31 - Physical Address"]
    #[inline(always)]
    #[must_use]
    pub fn phyaddr31_29(&mut self) -> PHYADDR31_29_W<29> {
        PHYADDR31_29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Search Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csar](index.html) module"]
pub struct CSAR_SPEC;
impl crate::RegisterSpec for CSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csar::R](R) reader structure"]
impl crate::Readable for CSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csar::W](W) writer structure"]
impl crate::Writable for CSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSAR to value 0"]
impl crate::Resettable for CSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
