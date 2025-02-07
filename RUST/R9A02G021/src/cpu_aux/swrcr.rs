#[doc = "Register `SWRCR` reader"]
pub type R = crate::R<SwrcrSpec>;
#[doc = "Register `SWRCR` writer"]
pub type W = crate::W<SwrcrSpec>;
#[doc = "System reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysresetreq {
    #[doc = "0: Do not request a system reset."]
    _0 = 0,
    #[doc = "1: Request a system reset."]
    _1 = 1,
}
impl From<Sysresetreq> for bool {
    #[inline(always)]
    fn from(variant: Sysresetreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRESETREQ` reader - System reset request"]
pub type SysresetreqR = crate::BitReader<Sysresetreq>;
impl SysresetreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysresetreq {
        match self.bits {
            false => Sysresetreq::_0,
            true => Sysresetreq::_1,
        }
    }
    #[doc = "Do not request a system reset."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sysresetreq::_0
    }
    #[doc = "Request a system reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sysresetreq::_1
    }
}
#[doc = "Field `SYSRESETREQ` writer - System reset request"]
pub type SysresetreqW<'a, REG> = crate::BitWriter<'a, REG, Sysresetreq>;
impl<'a, REG> SysresetreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not request a system reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sysresetreq::_0)
    }
    #[doc = "Request a system reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sysresetreq::_1)
    }
}
impl R {
    #[doc = "Bit 0 - System reset request"]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SysresetreqR {
        SysresetreqR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System reset request"]
    #[inline(always)]
    pub fn sysresetreq(&mut self) -> SysresetreqW<SwrcrSpec> {
        SysresetreqW::new(self, 0)
    }
}
#[doc = "Software Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrcrSpec;
impl crate::RegisterSpec for SwrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrcr::R`](R) reader structure"]
impl crate::Readable for SwrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`swrcr::W`](W) writer structure"]
impl crate::Writable for SwrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRCR to value 0"]
impl crate::Resettable for SwrcrSpec {
    const RESET_VALUE: u32 = 0;
}
