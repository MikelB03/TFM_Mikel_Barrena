#[doc = "Register `OSMCR` reader"]
pub type R = crate::R<OsmcrSpec>;
#[doc = "Register `OSMCR` writer"]
pub type W = crate::W<OsmcrSpec>;
#[doc = "Selection of the operating clock for the realtime clock, 32-bit interval timer, serial interfaces UARTA0 and UARTA1, remote control signal receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wutmmck0 {
    #[doc = "0: Subsystem clock (SOSC)"]
    _0 = 0,
    #[doc = "1: Low-speed on-chip oscillator clock (LOCO)"]
    _1 = 1,
}
impl From<Wutmmck0> for bool {
    #[inline(always)]
    fn from(variant: Wutmmck0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTMMCK0` reader - Selection of the operating clock for the realtime clock, 32-bit interval timer, serial interfaces UARTA0 and UARTA1, remote control signal receiver"]
pub type Wutmmck0R = crate::BitReader<Wutmmck0>;
impl Wutmmck0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wutmmck0 {
        match self.bits {
            false => Wutmmck0::_0,
            true => Wutmmck0::_1,
        }
    }
    #[doc = "Subsystem clock (SOSC)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wutmmck0::_0
    }
    #[doc = "Low-speed on-chip oscillator clock (LOCO)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wutmmck0::_1
    }
}
#[doc = "Field `WUTMMCK0` writer - Selection of the operating clock for the realtime clock, 32-bit interval timer, serial interfaces UARTA0 and UARTA1, remote control signal receiver"]
pub type Wutmmck0W<'a, REG> = crate::BitWriter<'a, REG, Wutmmck0>;
impl<'a, REG> Wutmmck0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Subsystem clock (SOSC)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wutmmck0::_0)
    }
    #[doc = "Low-speed on-chip oscillator clock (LOCO)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wutmmck0::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Selection of the operating clock for the realtime clock, 32-bit interval timer, serial interfaces UARTA0 and UARTA1, remote control signal receiver"]
    #[inline(always)]
    pub fn wutmmck0(&self) -> Wutmmck0R {
        Wutmmck0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selection of the operating clock for the realtime clock, 32-bit interval timer, serial interfaces UARTA0 and UARTA1, remote control signal receiver"]
    #[inline(always)]
    pub fn wutmmck0(&mut self) -> Wutmmck0W<OsmcrSpec> {
        Wutmmck0W::new(self, 0)
    }
}
#[doc = "Subsystem Clock Supply Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`osmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsmcrSpec;
impl crate::RegisterSpec for OsmcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osmcr::R`](R) reader structure"]
impl crate::Readable for OsmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`osmcr::W`](W) writer structure"]
impl crate::Writable for OsmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OSMCR to value 0"]
impl crate::Resettable for OsmcrSpec {
    const RESET_VALUE: u8 = 0;
}
