#[doc = "Register `DbiCmd0` writer"]
pub struct W(crate::W<DBI_CMD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_CMD0_SPEC>;
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
impl From<crate::W<DBI_CMD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_CMD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBI_COMMAND_WORD` writer - DBI Command Word"]
pub type DBI_COMMAND_WORD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_CMD0_SPEC, u16, u16, 16, O>;
#[doc = "DBI Command Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBI_COMMANDFLAG_AW {
    #[doc = "0: ADDRESS"]
    VAL0 = 0,
    #[doc = "1: WRITE_MEM_START"]
    VAL1 = 1,
    #[doc = "2: PARAMETER_OR_DATA"]
    VAL2 = 2,
    #[doc = "3: READ"]
    VAL3 = 3,
}
impl From<DBI_COMMANDFLAG_AW> for u8 {
    #[inline(always)]
    fn from(variant: DBI_COMMANDFLAG_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `DBI_COMMANDFLAG` writer - DBI Command Flag"]
pub type DBI_COMMANDFLAG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DBI_CMD0_SPEC, u8, DBI_COMMANDFLAG_AW, 2, O>;
impl<'a, const O: u8> DBI_COMMANDFLAG_W<'a, O> {
    #[doc = "ADDRESS"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(DBI_COMMANDFLAG_AW::VAL0)
    }
    #[doc = "WRITE_MEM_START"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(DBI_COMMANDFLAG_AW::VAL1)
    }
    #[doc = "PARAMETER_OR_DATA"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut W {
        self.variant(DBI_COMMANDFLAG_AW::VAL2)
    }
    #[doc = "READ"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut W {
        self.variant(DBI_COMMANDFLAG_AW::VAL3)
    }
}
impl W {
    #[doc = "Bits 0:15 - DBI Command Word"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_command_word(&mut self) -> DBI_COMMAND_WORD_W<0> {
        DBI_COMMAND_WORD_W::new(self)
    }
    #[doc = "Bits 30:31 - DBI Command Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_commandflag(&mut self) -> DBI_COMMANDFLAG_W<30> {
        DBI_COMMANDFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Command In/Out Port\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_cmd0](index.html) module"]
pub struct DBI_CMD0_SPEC;
impl crate::RegisterSpec for DBI_CMD0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dbi_cmd0::W](W) writer structure"]
impl crate::Writable for DBI_CMD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DbiCmd0 to value 0"]
impl crate::Resettable for DBI_CMD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
