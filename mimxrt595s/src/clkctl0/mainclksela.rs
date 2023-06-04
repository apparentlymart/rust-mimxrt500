#[doc = "Register `MAINCLKSELA` reader"]
pub struct R(crate::R<MAINCLKSELA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKSELA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKSELA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKSELA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKSELA` writer"]
pub struct W(crate::W<MAINCLKSELA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKSELA_SPEC>;
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
impl From<crate::W<MAINCLKSELA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKSELA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Control Main 1st Stage Control Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Control Main 1st Stage Control Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Low Power Oscillator Clock (LPOSC)"]
    LPOSC = 0,
    #[doc = "1: FRODIV which is the output of the FRODIVSEL mux"]
    FRO_8 = 1,
    #[doc = "2: OSC_CLK clock"]
    OSC_CLK = 2,
    #[doc = "3: FRO_DIV1 clock"]
    FRO = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::LPOSC,
            1 => SEL_A::FRO_8,
            2 => SEL_A::OSC_CLK,
            3 => SEL_A::FRO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LPOSC`"]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == SEL_A::LPOSC
    }
    #[doc = "Checks if the value of the field is `FRO_8`"]
    #[inline(always)]
    pub fn is_fro_8(&self) -> bool {
        *self == SEL_A::FRO_8
    }
    #[doc = "Checks if the value of the field is `OSC_CLK`"]
    #[inline(always)]
    pub fn is_osc_clk(&self) -> bool {
        *self == SEL_A::OSC_CLK
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline(always)]
    pub fn is_fro(&self) -> bool {
        *self == SEL_A::FRO
    }
}
#[doc = "Field `SEL` writer - Control Main 1st Stage Control Clock Source"]
pub type SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MAINCLKSELA_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Low Power Oscillator Clock (LPOSC)"]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut W {
        self.variant(SEL_A::LPOSC)
    }
    #[doc = "FRODIV which is the output of the FRODIVSEL mux"]
    #[inline(always)]
    pub fn fro_8(self) -> &'a mut W {
        self.variant(SEL_A::FRO_8)
    }
    #[doc = "OSC_CLK clock"]
    #[inline(always)]
    pub fn osc_clk(self) -> &'a mut W {
        self.variant(SEL_A::OSC_CLK)
    }
    #[doc = "FRO_DIV1 clock"]
    #[inline(always)]
    pub fn fro(self) -> &'a mut W {
        self.variant(SEL_A::FRO)
    }
}
impl R {
    #[doc = "Bits 0:1 - Control Main 1st Stage Control Clock Source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control Main 1st Stage Control Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Select A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclksela](index.html) module"]
pub struct MAINCLKSELA_SPEC;
impl crate::RegisterSpec for MAINCLKSELA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclksela::R](R) reader structure"]
impl crate::Readable for MAINCLKSELA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclksela::W](W) writer structure"]
impl crate::Writable for MAINCLKSELA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAINCLKSELA to value 0"]
impl crate::Resettable for MAINCLKSELA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
