#[doc = "Register `DOSCR` writer"]
pub type W = crate::W<DoscrSpec>;
#[doc = "DOPCF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dopcfcl {
    #[doc = "0: Maintains the DOPCF flag state."]
    _0 = 0,
    #[doc = "1: Clears the DOPCF flag."]
    _1 = 1,
}
impl From<Dopcfcl> for bool {
    #[inline(always)]
    fn from(variant: Dopcfcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOPCFCL` writer - DOPCF Clear"]
pub type DopcfclW<'a, REG> = crate::BitWriter<'a, REG, Dopcfcl>;
impl<'a, REG> DopcfclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Maintains the DOPCF flag state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dopcfcl::_0)
    }
    #[doc = "Clears the DOPCF flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dopcfcl::_1)
    }
}
impl W {
    #[doc = "Bit 0 - DOPCF Clear"]
    #[inline(always)]
    pub fn dopcfcl(&mut self) -> DopcfclW<DoscrSpec> {
        DopcfclW::new(self, 0)
    }
}
#[doc = "DOC Flag Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoscrSpec;
impl crate::RegisterSpec for DoscrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`doscr::W`](W) writer structure"]
impl crate::Writable for DoscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DOSCR to value 0"]
impl crate::Resettable for DoscrSpec {
    const RESET_VALUE: u8 = 0;
}
