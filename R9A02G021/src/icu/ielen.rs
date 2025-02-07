#[doc = "Register `IELEN` reader"]
pub type R = crate::R<IelenSpec>;
#[doc = "Register `IELEN` writer"]
pub type W = crate::W<IelenSpec>;
#[doc = "RTC Interrupt Enable (when LPOPT.LPOPTEN bit = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcen {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Rtcen> for bool {
    #[inline(always)]
    fn from(variant: Rtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC Interrupt Enable (when LPOPT.LPOPTEN bit = 1)"]
pub type RtcenR = crate::BitReader<Rtcen>;
impl RtcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcen {
        match self.bits {
            false => Rtcen::_0,
            true => Rtcen::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcen::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcen::_1
    }
}
#[doc = "Field `RTCEN` writer - RTC Interrupt Enable (when LPOPT.LPOPTEN bit = 1)"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG, Rtcen>;
impl<'a, REG> RtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::_1)
    }
}
#[doc = "Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ielen {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Ielen> for bool {
    #[inline(always)]
    fn from(variant: Ielen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IELEN` reader - Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)"]
pub type IelenR = crate::BitReader<Ielen>;
impl IelenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ielen {
        match self.bits {
            false => Ielen::_0,
            true => Ielen::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ielen::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ielen::_1
    }
}
#[doc = "Field `IELEN` writer - Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)"]
pub type IelenW<'a, REG> = crate::BitWriter<'a, REG, Ielen>;
impl<'a, REG> IelenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ielen::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ielen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RTC Interrupt Enable (when LPOPT.LPOPTEN bit = 1)"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)"]
    #[inline(always)]
    pub fn ielen(&self) -> IelenR {
        IelenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Interrupt Enable (when LPOPT.LPOPTEN bit = 1)"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<IelenSpec> {
        RtcenW::new(self, 0)
    }
    #[doc = "Bit 1 - Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)"]
    #[inline(always)]
    pub fn ielen(&mut self) -> IelenW<IelenSpec> {
        IelenW::new(self, 1)
    }
}
#[doc = "ICU Event Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ielen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ielen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IelenSpec;
impl crate::RegisterSpec for IelenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ielen::R`](R) reader structure"]
impl crate::Readable for IelenSpec {}
#[doc = "`write(|w| ..)` method takes [`ielen::W`](W) writer structure"]
impl crate::Writable for IelenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IELEN to value 0"]
impl crate::Resettable for IelenSpec {
    const RESET_VALUE: u8 = 0;
}
