#[doc = "Register `msip` reader"]
pub type R = crate::R<MsipSpec>;
#[doc = "Register `msip` writer"]
pub type W = crate::W<MsipSpec>;
#[doc = "Machine software interrupt control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msip {
    #[doc = "1: Generate a software interrupt."]
    _1 = 1,
    #[doc = "0: Clear the pending software interrupt."]
    _0 = 0,
}
impl From<Msip> for bool {
    #[inline(always)]
    fn from(variant: Msip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIP` reader - Machine software interrupt control"]
pub type MsipR = crate::BitReader<Msip>;
impl MsipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msip {
        match self.bits {
            true => Msip::_1,
            false => Msip::_0,
        }
    }
    #[doc = "Generate a software interrupt."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Msip::_1
    }
    #[doc = "Clear the pending software interrupt."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Msip::_0
    }
}
#[doc = "Field `MSIP` writer - Machine software interrupt control"]
pub type MsipW<'a, REG> = crate::BitWriter<'a, REG, Msip>;
impl<'a, REG> MsipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a software interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Msip::_1)
    }
    #[doc = "Clear the pending software interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Msip::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Machine software interrupt control"]
    #[inline(always)]
    pub fn msip(&self) -> MsipR {
        MsipR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Machine software interrupt control"]
    #[inline(always)]
    pub fn msip(&mut self) -> MsipW<MsipSpec> {
        MsipW::new(self, 0)
    }
}
#[doc = "Triggering Software Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`msip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsipSpec;
impl crate::RegisterSpec for MsipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msip::R`](R) reader structure"]
impl crate::Readable for MsipSpec {}
#[doc = "`write(|w| ..)` method takes [`msip::W`](W) writer structure"]
impl crate::Writable for MsipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets msip to value 0"]
impl crate::Resettable for MsipSpec {
    const RESET_VALUE: u32 = 0;
}
