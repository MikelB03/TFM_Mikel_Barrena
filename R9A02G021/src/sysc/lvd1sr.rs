#[doc = "Register `LVD1SR` reader"]
pub type R = crate::R<Lvd1srSpec>;
#[doc = "Register `LVD1SR` writer"]
pub type W = crate::W<Lvd1srSpec>;
#[doc = "Voltage Monitor 1 Voltage Variation Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Det {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Vdet1 crossing is detected"]
    _1 = 1,
}
impl From<Det> for bool {
    #[inline(always)]
    fn from(variant: Det) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DET` reader - Voltage Monitor 1 Voltage Variation Detection Flag"]
pub type DetR = crate::BitReader<Det>;
impl DetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Det {
        match self.bits {
            false => Det::_0,
            true => Det::_1,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Det::_0
    }
    #[doc = "Vdet1 crossing is detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Det::_1
    }
}
#[doc = "Field `DET` writer - Voltage Monitor 1 Voltage Variation Detection Flag"]
pub type DetW<'a, REG> = crate::BitWriter<'a, REG, Det>;
impl<'a, REG> DetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Det::_0)
    }
    #[doc = "Vdet1 crossing is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Det::_1)
    }
}
#[doc = "Voltage Monitor 1 Signal Monitor Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mon {
    #[doc = "0: VCC < Vdet1"]
    _0 = 0,
    #[doc = "1: VCC >= Vdet1 or MON is disabled"]
    _1 = 1,
}
impl From<Mon> for bool {
    #[inline(always)]
    fn from(variant: Mon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MON` reader - Voltage Monitor 1 Signal Monitor Flag"]
pub type MonR = crate::BitReader<Mon>;
impl MonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mon {
        match self.bits {
            false => Mon::_0,
            true => Mon::_1,
        }
    }
    #[doc = "VCC < Vdet1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mon::_0
    }
    #[doc = "VCC >= Vdet1 or MON is disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mon::_1
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Monitor 1 Voltage Variation Detection Flag"]
    #[inline(always)]
    pub fn det(&self) -> DetR {
        DetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage Monitor 1 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Monitor 1 Voltage Variation Detection Flag"]
    #[inline(always)]
    pub fn det(&mut self) -> DetW<Lvd1srSpec> {
        DetW::new(self, 0)
    }
}
#[doc = "Voltage Monitor 1 Circuit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd1sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lvd1srSpec;
impl crate::RegisterSpec for Lvd1srSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvd1sr::R`](R) reader structure"]
impl crate::Readable for Lvd1srSpec {}
#[doc = "`write(|w| ..)` method takes [`lvd1sr::W`](W) writer structure"]
impl crate::Writable for Lvd1srSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LVD1SR to value 0x02"]
impl crate::Resettable for Lvd1srSpec {
    const RESET_VALUE: u8 = 0x02;
}
