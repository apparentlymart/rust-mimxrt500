#[doc = "Register `GATE6` reader"]
pub struct R(crate::R<GATE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GATE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GATE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GATE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GATE6` writer"]
pub struct W(crate::W<GATE6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GATE6_SPEC>;
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
impl From<crate::W<GATE6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GATE6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTFSM` reader - Gate Finite State Machine"]
pub type GTFSM_R = crate::FieldReader<u8, GTFSM_A>;
#[doc = "Gate Finite State Machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GTFSM_A {
    #[doc = "0: The gate is unlocked (free)."]
    UNLOCKED = 0,
    #[doc = "1: Domain 0 locked the gate."]
    LOCKED_BY_D0 = 1,
    #[doc = "2: Domain 1 locked the gate."]
    LOCKED_BY_D1 = 2,
    #[doc = "3: Domain 2 locked the gate."]
    LOCKED_BY_D2 = 3,
    #[doc = "4: Domain 3 locked the gate."]
    LOCKED_BY_D3 = 4,
    #[doc = "5: Domain 4 locked the gate."]
    LOCKED_BY_D4 = 5,
    #[doc = "6: Domain 5 locked the gate."]
    LOCKED_BY_D5 = 6,
    #[doc = "7: Domain 6 locked the gate."]
    LOCKED_BY_D6 = 7,
    #[doc = "8: Domain 7 locked the gate."]
    LOCKED_BY_D7 = 8,
    #[doc = "9: Domain 8 locked the gate."]
    LOCKED_BY_D8 = 9,
    #[doc = "10: Domain 9 locked the gate."]
    LOCKED_BY_D9 = 10,
    #[doc = "11: Domain 10 locked the gate."]
    LOCKED_BY_D10 = 11,
    #[doc = "12: Domain 11 locked the gate."]
    LOCKED_BY_D11 = 12,
    #[doc = "13: Domain 12 locked the gate."]
    LOCKED_BY_D12 = 13,
    #[doc = "14: Domain 13 locked the gate."]
    LOCKED_BY_D13 = 14,
    #[doc = "15: Domain 14 locked the gate."]
    LOCKED_BY_D14 = 15,
}
impl From<GTFSM_A> for u8 {
    #[inline(always)]
    fn from(variant: GTFSM_A) -> Self {
        variant as _
    }
}
impl GTFSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTFSM_A {
        match self.bits {
            0 => GTFSM_A::UNLOCKED,
            1 => GTFSM_A::LOCKED_BY_D0,
            2 => GTFSM_A::LOCKED_BY_D1,
            3 => GTFSM_A::LOCKED_BY_D2,
            4 => GTFSM_A::LOCKED_BY_D3,
            5 => GTFSM_A::LOCKED_BY_D4,
            6 => GTFSM_A::LOCKED_BY_D5,
            7 => GTFSM_A::LOCKED_BY_D6,
            8 => GTFSM_A::LOCKED_BY_D7,
            9 => GTFSM_A::LOCKED_BY_D8,
            10 => GTFSM_A::LOCKED_BY_D9,
            11 => GTFSM_A::LOCKED_BY_D10,
            12 => GTFSM_A::LOCKED_BY_D11,
            13 => GTFSM_A::LOCKED_BY_D12,
            14 => GTFSM_A::LOCKED_BY_D13,
            15 => GTFSM_A::LOCKED_BY_D14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == GTFSM_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D0`"]
    #[inline(always)]
    pub fn is_locked_by_d0(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D0
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D1`"]
    #[inline(always)]
    pub fn is_locked_by_d1(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D1
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D2`"]
    #[inline(always)]
    pub fn is_locked_by_d2(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D2
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D3`"]
    #[inline(always)]
    pub fn is_locked_by_d3(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D3
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D4`"]
    #[inline(always)]
    pub fn is_locked_by_d4(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D4
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D5`"]
    #[inline(always)]
    pub fn is_locked_by_d5(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D5
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D6`"]
    #[inline(always)]
    pub fn is_locked_by_d6(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D6
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D7`"]
    #[inline(always)]
    pub fn is_locked_by_d7(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D7
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D8`"]
    #[inline(always)]
    pub fn is_locked_by_d8(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D8
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D9`"]
    #[inline(always)]
    pub fn is_locked_by_d9(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D9
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D10`"]
    #[inline(always)]
    pub fn is_locked_by_d10(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D10
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D11`"]
    #[inline(always)]
    pub fn is_locked_by_d11(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D11
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D12`"]
    #[inline(always)]
    pub fn is_locked_by_d12(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D12
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D13`"]
    #[inline(always)]
    pub fn is_locked_by_d13(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D13
    }
    #[doc = "Checks if the value of the field is `LOCKED_BY_D14`"]
    #[inline(always)]
    pub fn is_locked_by_d14(&self) -> bool {
        *self == GTFSM_A::LOCKED_BY_D14
    }
}
#[doc = "Field `GTFSM` writer - Gate Finite State Machine"]
pub type GTFSM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, GATE6_SPEC, u8, GTFSM_A, 4, O>;
impl<'a, const O: u8> GTFSM_W<'a, O> {
    #[doc = "The gate is unlocked (free)."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(GTFSM_A::UNLOCKED)
    }
    #[doc = "Domain 0 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d0(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D0)
    }
    #[doc = "Domain 1 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d1(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D1)
    }
    #[doc = "Domain 2 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d2(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D2)
    }
    #[doc = "Domain 3 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d3(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D3)
    }
    #[doc = "Domain 4 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d4(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D4)
    }
    #[doc = "Domain 5 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d5(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D5)
    }
    #[doc = "Domain 6 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d6(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D6)
    }
    #[doc = "Domain 7 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d7(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D7)
    }
    #[doc = "Domain 8 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d8(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D8)
    }
    #[doc = "Domain 9 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d9(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D9)
    }
    #[doc = "Domain 10 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d10(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D10)
    }
    #[doc = "Domain 11 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d11(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D11)
    }
    #[doc = "Domain 12 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d12(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D12)
    }
    #[doc = "Domain 13 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d13(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D13)
    }
    #[doc = "Domain 14 locked the gate."]
    #[inline(always)]
    pub fn locked_by_d14(self) -> &'a mut W {
        self.variant(GTFSM_A::LOCKED_BY_D14)
    }
}
impl R {
    #[doc = "Bits 0:3 - Gate Finite State Machine"]
    #[inline(always)]
    pub fn gtfsm(&self) -> GTFSM_R {
        GTFSM_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Gate Finite State Machine"]
    #[inline(always)]
    #[must_use]
    pub fn gtfsm(&mut self) -> GTFSM_W<0> {
        GTFSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gate6](index.html) module"]
pub struct GATE6_SPEC;
impl crate::RegisterSpec for GATE6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gate6::R](R) reader structure"]
impl crate::Readable for GATE6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gate6::W](W) writer structure"]
impl crate::Writable for GATE6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GATE6 to value 0"]
impl crate::Resettable for GATE6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
