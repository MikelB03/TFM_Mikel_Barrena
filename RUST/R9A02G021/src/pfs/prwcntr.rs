#[doc = "Register `PRWCNTR` writer"]
pub type W = crate::W<PrwcntrSpec>;
#[doc = "Wait Cycle Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wait {
    #[doc = "0: Setting prohibited"]
    _00 = 0,
    #[doc = "1: Insert a 1-cycle wait"]
    _01 = 1,
    #[doc = "2: Insert a 2-cycle wait"]
    _10 = 2,
    #[doc = "3: Insert a 3-cycle wait"]
    _11 = 3,
}
impl From<Wait> for u8 {
    #[inline(always)]
    fn from(variant: Wait) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wait {
    type Ux = u8;
}
impl crate::IsEnum for Wait {}
#[doc = "Field `WAIT` writer - Wait Cycle Control"]
pub type WaitW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wait, crate::Safe>;
impl<'a, REG> WaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::_00)
    }
    #[doc = "Insert a 1-cycle wait"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::_01)
    }
    #[doc = "Insert a 2-cycle wait"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::_10)
    }
    #[doc = "Insert a 3-cycle wait"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::_11)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wait Cycle Control"]
    #[inline(always)]
    pub fn wait(&mut self) -> WaitW<PrwcntrSpec> {
        WaitW::new(self, 0)
    }
}
#[doc = "Port Read Wait Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prwcntr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrwcntrSpec;
impl crate::RegisterSpec for PrwcntrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`prwcntr::W`](W) writer structure"]
impl crate::Writable for PrwcntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PRWCNTR to value 0x01"]
impl crate::Resettable for PrwcntrSpec {
    const RESET_VALUE: u8 = 0x01;
}
