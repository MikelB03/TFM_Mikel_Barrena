#[doc = "Register `REMSTC` reader"]
pub type R = crate::R<RemstcSpec>;
#[doc = "Register `REMSTC` writer"]
pub type W = crate::W<RemstcSpec>;
#[doc = "Snooze Mode Operation Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzon {
    #[doc = "0: Disables transition from Software Standby mode to Snooze mode"]
    _0 = 0,
    #[doc = "1: Enables transition from Software Standby mode to Snooze mode"]
    _1 = 1,
}
impl From<Snzon> for bool {
    #[inline(always)]
    fn from(variant: Snzon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZON` reader - Snooze Mode Operation Control"]
pub type SnzonR = crate::BitReader<Snzon>;
impl SnzonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzon {
        match self.bits {
            false => Snzon::_0,
            true => Snzon::_1,
        }
    }
    #[doc = "Disables transition from Software Standby mode to Snooze mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzon::_0
    }
    #[doc = "Enables transition from Software Standby mode to Snooze mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzon::_1
    }
}
#[doc = "Field `SNZON` writer - Snooze Mode Operation Control"]
pub type SnzonW<'a, REG> = crate::BitWriter<'a, REG, Snzon>;
impl<'a, REG> SnzonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables transition from Software Standby mode to Snooze mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzon::_0)
    }
    #[doc = "Enables transition from Software Standby mode to Snooze mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzon::_1)
    }
}
#[doc = "Digital Filter Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dnfsl {
    #[doc = "0: REMC operating clock is selected as a sampling clock"]
    _0 = 0,
    #[doc = "1: REMCLCLK/REMCSCLK is selected as a sampling clock"]
    _1 = 1,
}
impl From<Dnfsl> for bool {
    #[inline(always)]
    fn from(variant: Dnfsl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNFSL` reader - Digital Filter Clock Selection"]
pub type DnfslR = crate::BitReader<Dnfsl>;
impl DnfslR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dnfsl {
        match self.bits {
            false => Dnfsl::_0,
            true => Dnfsl::_1,
        }
    }
    #[doc = "REMC operating clock is selected as a sampling clock"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dnfsl::_0
    }
    #[doc = "REMCLCLK/REMCSCLK is selected as a sampling clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dnfsl::_1
    }
}
#[doc = "Field `DNFSL` writer - Digital Filter Clock Selection"]
pub type DnfslW<'a, REG> = crate::BitWriter<'a, REG, Dnfsl>;
impl<'a, REG> DnfslW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "REMC operating clock is selected as a sampling clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dnfsl::_0)
    }
    #[doc = "REMCLCLK/REMCSCLK is selected as a sampling clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dnfsl::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Snooze Mode Operation Control"]
    #[inline(always)]
    pub fn snzon(&self) -> SnzonR {
        SnzonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digital Filter Clock Selection"]
    #[inline(always)]
    pub fn dnfsl(&self) -> DnfslR {
        DnfslR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Snooze Mode Operation Control"]
    #[inline(always)]
    pub fn snzon(&mut self) -> SnzonW<RemstcSpec> {
        SnzonW::new(self, 0)
    }
    #[doc = "Bit 1 - Digital Filter Clock Selection"]
    #[inline(always)]
    pub fn dnfsl(&mut self) -> DnfslW<RemstcSpec> {
        DnfslW::new(self, 1)
    }
}
#[doc = "Receiver Standby Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remstc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remstc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemstcSpec;
impl crate::RegisterSpec for RemstcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`remstc::R`](R) reader structure"]
impl crate::Readable for RemstcSpec {}
#[doc = "`write(|w| ..)` method takes [`remstc::W`](W) writer structure"]
impl crate::Writable for RemstcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REMSTC to value 0"]
impl crate::Resettable for RemstcSpec {
    const RESET_VALUE: u8 = 0;
}
