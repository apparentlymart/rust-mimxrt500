#[doc = "Register `TIMER0` reader"]
pub struct R(crate::R<TIMER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0` writer"]
pub struct W(crate::W<TIMER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_SPEC>;
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
impl From<crate::W<TIMER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNITCON` reader - Unit Connection Timer Elapse (in ms)"]
pub type TUNITCON_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSEQ_INIT` reader - Sequence Initiation Time"]
pub type TSEQ_INIT_R = crate::FieldReader<u16, TSEQ_INIT_A>;
#[doc = "Sequence Initiation Time\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TSEQ_INIT_A {
    #[doc = "0: 0ms - 1023ms"]
    MS_0 = 0,
    #[doc = "1: 0ms - 1023ms"]
    MS_1 = 1,
    #[doc = "2: 0ms - 1023ms"]
    MS_2 = 2,
    #[doc = "3: 0ms - 1023ms"]
    MS_3 = 3,
    #[doc = "4: 0ms - 1023ms"]
    MS_4 = 4,
    #[doc = "5: 0ms - 1023ms"]
    MS_5 = 5,
    #[doc = "6: 0ms - 1023ms"]
    MS_6 = 6,
    #[doc = "7: 0ms - 1023ms"]
    MS_7 = 7,
    #[doc = "8: 0ms - 1023ms"]
    MS_8 = 8,
    #[doc = "9: 0ms - 1023ms"]
    MS_9 = 9,
}
impl From<TSEQ_INIT_A> for u16 {
    #[inline(always)]
    fn from(variant: TSEQ_INIT_A) -> Self {
        variant as _
    }
}
impl TSEQ_INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSEQ_INIT_A> {
        match self.bits {
            0 => Some(TSEQ_INIT_A::MS_0),
            1 => Some(TSEQ_INIT_A::MS_1),
            2 => Some(TSEQ_INIT_A::MS_2),
            3 => Some(TSEQ_INIT_A::MS_3),
            4 => Some(TSEQ_INIT_A::MS_4),
            5 => Some(TSEQ_INIT_A::MS_5),
            6 => Some(TSEQ_INIT_A::MS_6),
            7 => Some(TSEQ_INIT_A::MS_7),
            8 => Some(TSEQ_INIT_A::MS_8),
            9 => Some(TSEQ_INIT_A::MS_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MS_0`"]
    #[inline(always)]
    pub fn is_ms_0(&self) -> bool {
        *self == TSEQ_INIT_A::MS_0
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline(always)]
    pub fn is_ms_1(&self) -> bool {
        *self == TSEQ_INIT_A::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline(always)]
    pub fn is_ms_2(&self) -> bool {
        *self == TSEQ_INIT_A::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline(always)]
    pub fn is_ms_3(&self) -> bool {
        *self == TSEQ_INIT_A::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline(always)]
    pub fn is_ms_4(&self) -> bool {
        *self == TSEQ_INIT_A::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline(always)]
    pub fn is_ms_5(&self) -> bool {
        *self == TSEQ_INIT_A::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline(always)]
    pub fn is_ms_6(&self) -> bool {
        *self == TSEQ_INIT_A::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline(always)]
    pub fn is_ms_7(&self) -> bool {
        *self == TSEQ_INIT_A::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline(always)]
    pub fn is_ms_8(&self) -> bool {
        *self == TSEQ_INIT_A::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline(always)]
    pub fn is_ms_9(&self) -> bool {
        *self == TSEQ_INIT_A::MS_9
    }
}
#[doc = "Field `TSEQ_INIT` writer - Sequence Initiation Time"]
pub type TSEQ_INIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER0_SPEC, u16, TSEQ_INIT_A, 10, O>;
impl<'a, const O: u8> TSEQ_INIT_W<'a, O> {
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_0(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_0)
    }
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_1)
    }
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_2)
    }
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_3)
    }
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_4)
    }
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_5)
    }
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_6)
    }
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_7)
    }
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_8)
    }
    #[doc = "0ms - 1023ms"]
    #[inline(always)]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(TSEQ_INIT_A::MS_9)
    }
}
impl R {
    #[doc = "Bits 0:11 - Unit Connection Timer Elapse (in ms)"]
    #[inline(always)]
    pub fn tunitcon(&self) -> TUNITCON_R {
        TUNITCON_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - Sequence Initiation Time"]
    #[inline(always)]
    pub fn tseq_init(&self) -> TSEQ_INIT_R {
        TSEQ_INIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - Sequence Initiation Time"]
    #[inline(always)]
    #[must_use]
    pub fn tseq_init(&mut self) -> TSEQ_INIT_W<16> {
        TSEQ_INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0](index.html) module"]
pub struct TIMER0_SPEC;
impl crate::RegisterSpec for TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0::R](R) reader structure"]
impl crate::Readable for TIMER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0::W](W) writer structure"]
impl crate::Writable for TIMER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER0 to value 0x0010_0000"]
impl crate::Resettable for TIMER0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000;
}
