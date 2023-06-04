#[doc = "Register `POLSEL` reader"]
pub struct R(crate::R<POLSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POLSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POLSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POLSEL` writer"]
pub struct W(crate::W<POLSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POLSEL_SPEC>;
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
impl From<crate::W<POLSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POLSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG0_POLICY` reader - Policy Select For Region 0"]
pub type REG0_POLICY_R = crate::FieldReader<u8, REG0_POLICY_A>;
#[doc = "Policy Select For Region 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG0_POLICY_A {
    #[doc = "0: Noncacheable"]
    REG0_00 = 0,
    #[doc = "1: Write-through"]
    REG0_01 = 1,
    #[doc = "2: Write-back"]
    REG0_10 = 2,
    #[doc = "3: Invalid"]
    REG0_11 = 3,
}
impl From<REG0_POLICY_A> for u8 {
    #[inline(always)]
    fn from(variant: REG0_POLICY_A) -> Self {
        variant as _
    }
}
impl REG0_POLICY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG0_POLICY_A {
        match self.bits {
            0 => REG0_POLICY_A::REG0_00,
            1 => REG0_POLICY_A::REG0_01,
            2 => REG0_POLICY_A::REG0_10,
            3 => REG0_POLICY_A::REG0_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG0_00`"]
    #[inline(always)]
    pub fn is_reg0_00(&self) -> bool {
        *self == REG0_POLICY_A::REG0_00
    }
    #[doc = "Checks if the value of the field is `REG0_01`"]
    #[inline(always)]
    pub fn is_reg0_01(&self) -> bool {
        *self == REG0_POLICY_A::REG0_01
    }
    #[doc = "Checks if the value of the field is `REG0_10`"]
    #[inline(always)]
    pub fn is_reg0_10(&self) -> bool {
        *self == REG0_POLICY_A::REG0_10
    }
    #[doc = "Checks if the value of the field is `REG0_11`"]
    #[inline(always)]
    pub fn is_reg0_11(&self) -> bool {
        *self == REG0_POLICY_A::REG0_11
    }
}
#[doc = "Field `REG0_POLICY` writer - Policy Select For Region 0"]
pub type REG0_POLICY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, POLSEL_SPEC, u8, REG0_POLICY_A, 2, O>;
impl<'a, const O: u8> REG0_POLICY_W<'a, O> {
    #[doc = "Noncacheable"]
    #[inline(always)]
    pub fn reg0_00(self) -> &'a mut W {
        self.variant(REG0_POLICY_A::REG0_00)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn reg0_01(self) -> &'a mut W {
        self.variant(REG0_POLICY_A::REG0_01)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn reg0_10(self) -> &'a mut W {
        self.variant(REG0_POLICY_A::REG0_10)
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn reg0_11(self) -> &'a mut W {
        self.variant(REG0_POLICY_A::REG0_11)
    }
}
#[doc = "Field `REG1_POLICY` reader - Policy Select For Region 1"]
pub type REG1_POLICY_R = crate::FieldReader<u8, REG1_POLICY_A>;
#[doc = "Policy Select For Region 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG1_POLICY_A {
    #[doc = "0: Noncacheable"]
    REG1_00 = 0,
    #[doc = "1: Write-through"]
    REG1_01 = 1,
    #[doc = "2: Write-back"]
    REG1_10 = 2,
    #[doc = "3: Invalid"]
    REG1_11 = 3,
}
impl From<REG1_POLICY_A> for u8 {
    #[inline(always)]
    fn from(variant: REG1_POLICY_A) -> Self {
        variant as _
    }
}
impl REG1_POLICY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG1_POLICY_A {
        match self.bits {
            0 => REG1_POLICY_A::REG1_00,
            1 => REG1_POLICY_A::REG1_01,
            2 => REG1_POLICY_A::REG1_10,
            3 => REG1_POLICY_A::REG1_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG1_00`"]
    #[inline(always)]
    pub fn is_reg1_00(&self) -> bool {
        *self == REG1_POLICY_A::REG1_00
    }
    #[doc = "Checks if the value of the field is `REG1_01`"]
    #[inline(always)]
    pub fn is_reg1_01(&self) -> bool {
        *self == REG1_POLICY_A::REG1_01
    }
    #[doc = "Checks if the value of the field is `REG1_10`"]
    #[inline(always)]
    pub fn is_reg1_10(&self) -> bool {
        *self == REG1_POLICY_A::REG1_10
    }
    #[doc = "Checks if the value of the field is `REG1_11`"]
    #[inline(always)]
    pub fn is_reg1_11(&self) -> bool {
        *self == REG1_POLICY_A::REG1_11
    }
}
#[doc = "Field `REG1_POLICY` writer - Policy Select For Region 1"]
pub type REG1_POLICY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, POLSEL_SPEC, u8, REG1_POLICY_A, 2, O>;
impl<'a, const O: u8> REG1_POLICY_W<'a, O> {
    #[doc = "Noncacheable"]
    #[inline(always)]
    pub fn reg1_00(self) -> &'a mut W {
        self.variant(REG1_POLICY_A::REG1_00)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn reg1_01(self) -> &'a mut W {
        self.variant(REG1_POLICY_A::REG1_01)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn reg1_10(self) -> &'a mut W {
        self.variant(REG1_POLICY_A::REG1_10)
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn reg1_11(self) -> &'a mut W {
        self.variant(REG1_POLICY_A::REG1_11)
    }
}
#[doc = "Field `REG2_POLICY` reader - Policy Select For Region 2"]
pub type REG2_POLICY_R = crate::FieldReader<u8, REG2_POLICY_A>;
#[doc = "Policy Select For Region 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG2_POLICY_A {
    #[doc = "0: Noncacheable"]
    REG2_00 = 0,
    #[doc = "1: Write-through"]
    REG2_01 = 1,
    #[doc = "2: Write-back"]
    REG2_10 = 2,
    #[doc = "3: Invalid"]
    REG2_11 = 3,
}
impl From<REG2_POLICY_A> for u8 {
    #[inline(always)]
    fn from(variant: REG2_POLICY_A) -> Self {
        variant as _
    }
}
impl REG2_POLICY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG2_POLICY_A {
        match self.bits {
            0 => REG2_POLICY_A::REG2_00,
            1 => REG2_POLICY_A::REG2_01,
            2 => REG2_POLICY_A::REG2_10,
            3 => REG2_POLICY_A::REG2_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG2_00`"]
    #[inline(always)]
    pub fn is_reg2_00(&self) -> bool {
        *self == REG2_POLICY_A::REG2_00
    }
    #[doc = "Checks if the value of the field is `REG2_01`"]
    #[inline(always)]
    pub fn is_reg2_01(&self) -> bool {
        *self == REG2_POLICY_A::REG2_01
    }
    #[doc = "Checks if the value of the field is `REG2_10`"]
    #[inline(always)]
    pub fn is_reg2_10(&self) -> bool {
        *self == REG2_POLICY_A::REG2_10
    }
    #[doc = "Checks if the value of the field is `REG2_11`"]
    #[inline(always)]
    pub fn is_reg2_11(&self) -> bool {
        *self == REG2_POLICY_A::REG2_11
    }
}
#[doc = "Field `REG2_POLICY` writer - Policy Select For Region 2"]
pub type REG2_POLICY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, POLSEL_SPEC, u8, REG2_POLICY_A, 2, O>;
impl<'a, const O: u8> REG2_POLICY_W<'a, O> {
    #[doc = "Noncacheable"]
    #[inline(always)]
    pub fn reg2_00(self) -> &'a mut W {
        self.variant(REG2_POLICY_A::REG2_00)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn reg2_01(self) -> &'a mut W {
        self.variant(REG2_POLICY_A::REG2_01)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn reg2_10(self) -> &'a mut W {
        self.variant(REG2_POLICY_A::REG2_10)
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn reg2_11(self) -> &'a mut W {
        self.variant(REG2_POLICY_A::REG2_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Policy Select For Region 0"]
    #[inline(always)]
    pub fn reg0_policy(&self) -> REG0_POLICY_R {
        REG0_POLICY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Policy Select For Region 1"]
    #[inline(always)]
    pub fn reg1_policy(&self) -> REG1_POLICY_R {
        REG1_POLICY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Policy Select For Region 2"]
    #[inline(always)]
    pub fn reg2_policy(&self) -> REG2_POLICY_R {
        REG2_POLICY_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Policy Select For Region 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg0_policy(&mut self) -> REG0_POLICY_W<0> {
        REG0_POLICY_W::new(self)
    }
    #[doc = "Bits 2:3 - Policy Select For Region 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg1_policy(&mut self) -> REG1_POLICY_W<2> {
        REG1_POLICY_W::new(self)
    }
    #[doc = "Bits 4:5 - Policy Select For Region 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_policy(&mut self) -> REG2_POLICY_W<4> {
        REG2_POLICY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Policy Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [polsel](index.html) module"]
pub struct POLSEL_SPEC;
impl crate::RegisterSpec for POLSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [polsel::R](R) reader structure"]
impl crate::Readable for POLSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [polsel::W](W) writer structure"]
impl crate::Writable for POLSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POLSEL to value 0"]
impl crate::Resettable for POLSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
