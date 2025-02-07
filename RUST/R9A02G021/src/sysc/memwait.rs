#[doc = "Register `MEMWAIT` reader"]
pub type R = crate::R<MemwaitSpec>;
#[doc = "Register `MEMWAIT` writer"]
pub type W = crate::W<MemwaitSpec>;
#[doc = "Memory Wait Cycle Select for Code Flash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memwait {
    #[doc = "0: No wait"]
    _0 = 0,
    #[doc = "1: Wait"]
    _1 = 1,
}
impl From<Memwait> for bool {
    #[inline(always)]
    fn from(variant: Memwait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMWAIT` reader - Memory Wait Cycle Select for Code Flash"]
pub type MemwaitR = crate::BitReader<Memwait>;
impl MemwaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memwait {
        match self.bits {
            false => Memwait::_0,
            true => Memwait::_1,
        }
    }
    #[doc = "No wait"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Memwait::_0
    }
    #[doc = "Wait"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Memwait::_1
    }
}
#[doc = "Field `MEMWAIT` writer - Memory Wait Cycle Select for Code Flash"]
pub type MemwaitW<'a, REG> = crate::BitWriter<'a, REG, Memwait>;
impl<'a, REG> MemwaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wait"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Memwait::_0)
    }
    #[doc = "Wait"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Memwait::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Wait Cycle Select for Code Flash"]
    #[inline(always)]
    pub fn memwait(&self) -> MemwaitR {
        MemwaitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Wait Cycle Select for Code Flash"]
    #[inline(always)]
    pub fn memwait(&mut self) -> MemwaitW<MemwaitSpec> {
        MemwaitW::new(self, 0)
    }
}
#[doc = "Memory Wait Cycle Control Register for Code Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`memwait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memwait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemwaitSpec;
impl crate::RegisterSpec for MemwaitSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`memwait::R`](R) reader structure"]
impl crate::Readable for MemwaitSpec {}
#[doc = "`write(|w| ..)` method takes [`memwait::W`](W) writer structure"]
impl crate::Writable for MemwaitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MEMWAIT to value 0"]
impl crate::Resettable for MemwaitSpec {
    const RESET_VALUE: u8 = 0;
}
