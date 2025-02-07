#[doc = "Register `FLDWAITR` reader"]
pub type R = crate::R<FldwaitrSpec>;
#[doc = "Register `FLDWAITR` writer"]
pub type W = crate::W<FldwaitrSpec>;
#[doc = "Memory Wait Cycle Select for Data Flash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fldwait1 {
    #[doc = "0: 1 wait access (Default)"]
    _0 = 0,
    #[doc = "1: 2 wait access"]
    _1 = 1,
}
impl From<Fldwait1> for bool {
    #[inline(always)]
    fn from(variant: Fldwait1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLDWAIT1` reader - Memory Wait Cycle Select for Data Flash"]
pub type Fldwait1R = crate::BitReader<Fldwait1>;
impl Fldwait1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fldwait1 {
        match self.bits {
            false => Fldwait1::_0,
            true => Fldwait1::_1,
        }
    }
    #[doc = "1 wait access (Default)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fldwait1::_0
    }
    #[doc = "2 wait access"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fldwait1::_1
    }
}
#[doc = "Field `FLDWAIT1` writer - Memory Wait Cycle Select for Data Flash"]
pub type Fldwait1W<'a, REG> = crate::BitWriter<'a, REG, Fldwait1>;
impl<'a, REG> Fldwait1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 wait access (Default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fldwait1::_0)
    }
    #[doc = "2 wait access"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fldwait1::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Wait Cycle Select for Data Flash"]
    #[inline(always)]
    pub fn fldwait1(&self) -> Fldwait1R {
        Fldwait1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Wait Cycle Select for Data Flash"]
    #[inline(always)]
    pub fn fldwait1(&mut self) -> Fldwait1W<FldwaitrSpec> {
        Fldwait1W::new(self, 0)
    }
}
#[doc = "Memory Wait Cycle Control Register for Data Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`fldwaitr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fldwaitr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FldwaitrSpec;
impl crate::RegisterSpec for FldwaitrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fldwaitr::R`](R) reader structure"]
impl crate::Readable for FldwaitrSpec {}
#[doc = "`write(|w| ..)` method takes [`fldwaitr::W`](W) writer structure"]
impl crate::Writable for FldwaitrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FLDWAITR to value 0"]
impl crate::Resettable for FldwaitrSpec {
    const RESET_VALUE: u8 = 0;
}
