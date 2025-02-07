#[doc = "Register `REMCON0` reader"]
pub type R = crate::R<Remcon0Spec>;
#[doc = "Register `REMCON0` writer"]
pub type W = crate::W<Remcon0Spec>;
#[doc = "Remote Control Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enflg {
    #[doc = "0: Stopped"]
    _0 = 0,
    #[doc = "1: Operating"]
    _1 = 1,
}
impl From<Enflg> for bool {
    #[inline(always)]
    fn from(variant: Enflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENFLG` reader - Remote Control Status Flag"]
pub type EnflgR = crate::BitReader<Enflg>;
impl EnflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enflg {
        match self.bits {
            false => Enflg::_0,
            true => Enflg::_1,
        }
    }
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enflg::_0
    }
    #[doc = "Operating"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enflg::_1
    }
}
#[doc = "Input Signal Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: Not inverted"]
    _0 = 0,
    #[doc = "1: Inverted"]
    _1 = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Input Signal Inversion"]
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::_0,
            true => Inv::_1,
        }
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inv::_0
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inv::_1
    }
}
#[doc = "Field `INV` writer - Input Signal Inversion"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_1)
    }
}
#[doc = "Digital Filter Enable or Disable Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fil {
    #[doc = "0: Disables the digital filter for matching three or two times"]
    _0 = 0,
    #[doc = "1: Enables the digital filter for matching three or two times"]
    _1 = 1,
}
impl From<Fil> for bool {
    #[inline(always)]
    fn from(variant: Fil) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIL` reader - Digital Filter Enable or Disable Setting"]
pub type FilR = crate::BitReader<Fil>;
impl FilR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fil {
        match self.bits {
            false => Fil::_0,
            true => Fil::_1,
        }
    }
    #[doc = "Disables the digital filter for matching three or two times"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fil::_0
    }
    #[doc = "Enables the digital filter for matching three or two times"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fil::_1
    }
}
#[doc = "Field `FIL` writer - Digital Filter Enable or Disable Setting"]
pub type FilW<'a, REG> = crate::BitWriter<'a, REG, Fil>;
impl<'a, REG> FilW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the digital filter for matching three or two times"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fil::_0)
    }
    #[doc = "Enables the digital filter for matching three or two times"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fil::_1)
    }
}
#[doc = "Input Signal Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inflg {
    #[doc = "0: The level of the internal input signal of the remote control signal receiver is low"]
    _0 = 0,
    #[doc = "1: The level of the internal input signal of the remote control signal receiver is high"]
    _1 = 1,
}
impl From<Inflg> for bool {
    #[inline(always)]
    fn from(variant: Inflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INFLG` reader - Input Signal Flag"]
pub type InflgR = crate::BitReader<Inflg>;
impl InflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inflg {
        match self.bits {
            false => Inflg::_0,
            true => Inflg::_1,
        }
    }
    #[doc = "The level of the internal input signal of the remote control signal receiver is low"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inflg::_0
    }
    #[doc = "The level of the internal input signal of the remote control signal receiver is high"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inflg::_1
    }
}
#[doc = "Receive Error Capture Operation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ec {
    #[doc = "0: Captures the data after an error pattern is received"]
    _0 = 0,
    #[doc = "1: Does not capture the data after an error pattern is received"]
    _1 = 1,
}
impl From<Ec> for bool {
    #[inline(always)]
    fn from(variant: Ec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EC` reader - Receive Error Capture Operation Select"]
pub type EcR = crate::BitReader<Ec>;
impl EcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ec {
        match self.bits {
            false => Ec::_0,
            true => Ec::_1,
        }
    }
    #[doc = "Captures the data after an error pattern is received"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ec::_0
    }
    #[doc = "Does not capture the data after an error pattern is received"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ec::_1
    }
}
#[doc = "Field `EC` writer - Receive Error Capture Operation Select"]
pub type EcW<'a, REG> = crate::BitWriter<'a, REG, Ec>;
impl<'a, REG> EcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Captures the data after an error pattern is received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ec::_0)
    }
    #[doc = "Does not capture the data after an error pattern is received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ec::_1)
    }
}
#[doc = "Digital Filter Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filsel {
    #[doc = "0: Digital filter for matching three times"]
    _0 = 0,
    #[doc = "1: Digital filter for matching two times"]
    _1 = 1,
}
impl From<Filsel> for bool {
    #[inline(always)]
    fn from(variant: Filsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILSEL` reader - Digital Filter Function Select"]
pub type FilselR = crate::BitReader<Filsel>;
impl FilselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filsel {
        match self.bits {
            false => Filsel::_0,
            true => Filsel::_1,
        }
    }
    #[doc = "Digital filter for matching three times"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Filsel::_0
    }
    #[doc = "Digital filter for matching two times"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Filsel::_1
    }
}
#[doc = "Field `FILSEL` writer - Digital Filter Function Select"]
pub type FilselW<'a, REG> = crate::BitWriter<'a, REG, Filsel>;
impl<'a, REG> FilselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital filter for matching three times"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Filsel::_0)
    }
    #[doc = "Digital filter for matching two times"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Filsel::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Remote Control Status Flag"]
    #[inline(always)]
    pub fn enflg(&self) -> EnflgR {
        EnflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Signal Inversion"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Digital Filter Enable or Disable Setting"]
    #[inline(always)]
    pub fn fil(&self) -> FilR {
        FilR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input Signal Flag"]
    #[inline(always)]
    pub fn inflg(&self) -> InflgR {
        InflgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Error Capture Operation Select"]
    #[inline(always)]
    pub fn ec(&self) -> EcR {
        EcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Digital Filter Function Select"]
    #[inline(always)]
    pub fn filsel(&self) -> FilselR {
        FilselR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Input Signal Inversion"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<Remcon0Spec> {
        InvW::new(self, 1)
    }
    #[doc = "Bit 2 - Digital Filter Enable or Disable Setting"]
    #[inline(always)]
    pub fn fil(&mut self) -> FilW<Remcon0Spec> {
        FilW::new(self, 2)
    }
    #[doc = "Bit 4 - Receive Error Capture Operation Select"]
    #[inline(always)]
    pub fn ec(&mut self) -> EcW<Remcon0Spec> {
        EcW::new(self, 4)
    }
    #[doc = "Bit 6 - Digital Filter Function Select"]
    #[inline(always)]
    pub fn filsel(&mut self) -> FilselW<Remcon0Spec> {
        FilselW::new(self, 6)
    }
}
#[doc = "Function Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`remcon0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remcon0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remcon0Spec;
impl crate::RegisterSpec for Remcon0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`remcon0::R`](R) reader structure"]
impl crate::Readable for Remcon0Spec {}
#[doc = "`write(|w| ..)` method takes [`remcon0::W`](W) writer structure"]
impl crate::Writable for Remcon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REMCON0 to value 0"]
impl crate::Resettable for Remcon0Spec {
    const RESET_VALUE: u8 = 0;
}
