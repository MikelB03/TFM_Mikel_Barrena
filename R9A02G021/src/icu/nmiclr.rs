#[doc = "Register `NMICLR` reader"]
pub type R = crate::R<NmiclrSpec>;
#[doc = "Register `NMICLR` writer"]
pub type W = crate::W<NmiclrSpec>;
#[doc = "IWDT Underflow/Refresh Error Interrupt Status Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtclr {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.IWDTST flag"]
    _1 = 1,
}
impl From<Iwdtclr> for bool {
    #[inline(always)]
    fn from(variant: Iwdtclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTCLR` reader - IWDT Underflow/Refresh Error Interrupt Status Flag Clear"]
pub type IwdtclrR = crate::BitReader<Iwdtclr>;
impl IwdtclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtclr {
        match self.bits {
            false => Iwdtclr::_0,
            true => Iwdtclr::_1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtclr::_0
    }
    #[doc = "Clear the NMISR.IWDTST flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtclr::_1
    }
}
#[doc = "Field `IWDTCLR` writer - IWDT Underflow/Refresh Error Interrupt Status Flag Clear"]
pub type IwdtclrW<'a, REG> = crate::BitWriter<'a, REG, Iwdtclr>;
impl<'a, REG> IwdtclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtclr::_0)
    }
    #[doc = "Clear the NMISR.IWDTST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtclr::_1)
    }
}
#[doc = "WDT Underflow/Refresh Error Interrupt Status Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtclr {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.WDTST flag"]
    _1 = 1,
}
impl From<Wdtclr> for bool {
    #[inline(always)]
    fn from(variant: Wdtclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCLR` reader - WDT Underflow/Refresh Error Interrupt Status Flag Clear"]
pub type WdtclrR = crate::BitReader<Wdtclr>;
impl WdtclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtclr {
        match self.bits {
            false => Wdtclr::_0,
            true => Wdtclr::_1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdtclr::_0
    }
    #[doc = "Clear the NMISR.WDTST flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdtclr::_1
    }
}
#[doc = "Field `WDTCLR` writer - WDT Underflow/Refresh Error Interrupt Status Flag Clear"]
pub type WdtclrW<'a, REG> = crate::BitWriter<'a, REG, Wdtclr>;
impl<'a, REG> WdtclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtclr::_0)
    }
    #[doc = "Clear the NMISR.WDTST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtclr::_1)
    }
}
#[doc = "Voltage Monitor 1 Interrupt Status Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1clr {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.LVD1ST flag"]
    _1 = 1,
}
impl From<Lvd1clr> for bool {
    #[inline(always)]
    fn from(variant: Lvd1clr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1CLR` reader - Voltage Monitor 1 Interrupt Status Flag Clear"]
pub type Lvd1clrR = crate::BitReader<Lvd1clr>;
impl Lvd1clrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1clr {
        match self.bits {
            false => Lvd1clr::_0,
            true => Lvd1clr::_1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1clr::_0
    }
    #[doc = "Clear the NMISR.LVD1ST flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1clr::_1
    }
}
#[doc = "Field `LVD1CLR` writer - Voltage Monitor 1 Interrupt Status Flag Clear"]
pub type Lvd1clrW<'a, REG> = crate::BitWriter<'a, REG, Lvd1clr>;
impl<'a, REG> Lvd1clrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1clr::_0)
    }
    #[doc = "Clear the NMISR.LVD1ST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1clr::_1)
    }
}
#[doc = "Voltage Monitor 2 Interrupt Status Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2clr {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.LVD2ST flag."]
    _1 = 1,
}
impl From<Lvd2clr> for bool {
    #[inline(always)]
    fn from(variant: Lvd2clr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2CLR` reader - Voltage Monitor 2 Interrupt Status Flag Clear"]
pub type Lvd2clrR = crate::BitReader<Lvd2clr>;
impl Lvd2clrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2clr {
        match self.bits {
            false => Lvd2clr::_0,
            true => Lvd2clr::_1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2clr::_0
    }
    #[doc = "Clear the NMISR.LVD2ST flag."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2clr::_1
    }
}
#[doc = "Field `LVD2CLR` writer - Voltage Monitor 2 Interrupt Status Flag Clear"]
pub type Lvd2clrW<'a, REG> = crate::BitWriter<'a, REG, Lvd2clr>;
impl<'a, REG> Lvd2clrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2clr::_0)
    }
    #[doc = "Clear the NMISR.LVD2ST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2clr::_1)
    }
}
#[doc = "NMI Pin Interrupt Status Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmiclr {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.NMIST flag"]
    _1 = 1,
}
impl From<Nmiclr> for bool {
    #[inline(always)]
    fn from(variant: Nmiclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMICLR` reader - NMI Pin Interrupt Status Flag Clear"]
pub type NmiclrR = crate::BitReader<Nmiclr>;
impl NmiclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmiclr {
        match self.bits {
            false => Nmiclr::_0,
            true => Nmiclr::_1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nmiclr::_0
    }
    #[doc = "Clear the NMISR.NMIST flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nmiclr::_1
    }
}
#[doc = "Field `NMICLR` writer - NMI Pin Interrupt Status Flag Clear"]
pub type NmiclrW<'a, REG> = crate::BitWriter<'a, REG, Nmiclr>;
impl<'a, REG> NmiclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiclr::_0)
    }
    #[doc = "Clear the NMISR.NMIST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiclr::_1)
    }
}
#[doc = "SRAM Parity Error Interrupt Status Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpeclr {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.RPEST flag"]
    _1 = 1,
}
impl From<Rpeclr> for bool {
    #[inline(always)]
    fn from(variant: Rpeclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPECLR` reader - SRAM Parity Error Interrupt Status Flag Clear"]
pub type RpeclrR = crate::BitReader<Rpeclr>;
impl RpeclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpeclr {
        match self.bits {
            false => Rpeclr::_0,
            true => Rpeclr::_1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpeclr::_0
    }
    #[doc = "Clear the NMISR.RPEST flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpeclr::_1
    }
}
#[doc = "Field `RPECLR` writer - SRAM Parity Error Interrupt Status Flag Clear"]
pub type RpeclrW<'a, REG> = crate::BitWriter<'a, REG, Rpeclr>;
impl<'a, REG> RpeclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeclr::_0)
    }
    #[doc = "Clear the NMISR.RPEST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeclr::_1)
    }
}
#[doc = "SRAM ECC Error Interrupt Status Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reccclr {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.RECCST flag"]
    _1 = 1,
}
impl From<Reccclr> for bool {
    #[inline(always)]
    fn from(variant: Reccclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECCCLR` reader - SRAM ECC Error Interrupt Status Flag Clear"]
pub type ReccclrR = crate::BitReader<Reccclr>;
impl ReccclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reccclr {
        match self.bits {
            false => Reccclr::_0,
            true => Reccclr::_1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reccclr::_0
    }
    #[doc = "Clear the NMISR.RECCST flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reccclr::_1
    }
}
#[doc = "Field `RECCCLR` writer - SRAM ECC Error Interrupt Status Flag Clear"]
pub type ReccclrW<'a, REG> = crate::BitWriter<'a, REG, Reccclr>;
impl<'a, REG> ReccclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reccclr::_0)
    }
    #[doc = "Clear the NMISR.RECCST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reccclr::_1)
    }
}
#[doc = "Bus Error Interrupt Status Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busclr {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.BUSST flag"]
    _1 = 1,
}
impl From<Busclr> for bool {
    #[inline(always)]
    fn from(variant: Busclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSCLR` reader - Bus Error Interrupt Status Flag Clear"]
pub type BusclrR = crate::BitReader<Busclr>;
impl BusclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busclr {
        match self.bits {
            false => Busclr::_0,
            true => Busclr::_1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busclr::_0
    }
    #[doc = "Clear the NMISR.BUSST flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busclr::_1
    }
}
#[doc = "Field `BUSCLR` writer - Bus Error Interrupt Status Flag Clear"]
pub type BusclrW<'a, REG> = crate::BitWriter<'a, REG, Busclr>;
impl<'a, REG> BusclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busclr::_0)
    }
    #[doc = "Clear the NMISR.BUSST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busclr::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IWDT Underflow/Refresh Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn iwdtclr(&self) -> IwdtclrR {
        IwdtclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT Underflow/Refresh Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn wdtclr(&self) -> WdtclrR {
        WdtclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn lvd1clr(&self) -> Lvd1clrR {
        Lvd1clrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Monitor 2 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn lvd2clr(&self) -> Lvd2clrR {
        Lvd2clrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - NMI Pin Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn nmiclr(&self) -> NmiclrR {
        NmiclrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Parity Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn rpeclr(&self) -> RpeclrR {
        RpeclrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM ECC Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn reccclr(&self) -> ReccclrR {
        ReccclrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Bus Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn busclr(&self) -> BusclrR {
        BusclrR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IWDT Underflow/Refresh Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn iwdtclr(&mut self) -> IwdtclrW<NmiclrSpec> {
        IwdtclrW::new(self, 0)
    }
    #[doc = "Bit 1 - WDT Underflow/Refresh Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn wdtclr(&mut self) -> WdtclrW<NmiclrSpec> {
        WdtclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn lvd1clr(&mut self) -> Lvd1clrW<NmiclrSpec> {
        Lvd1clrW::new(self, 2)
    }
    #[doc = "Bit 3 - Voltage Monitor 2 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn lvd2clr(&mut self) -> Lvd2clrW<NmiclrSpec> {
        Lvd2clrW::new(self, 3)
    }
    #[doc = "Bit 7 - NMI Pin Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn nmiclr(&mut self) -> NmiclrW<NmiclrSpec> {
        NmiclrW::new(self, 7)
    }
    #[doc = "Bit 8 - SRAM Parity Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn rpeclr(&mut self) -> RpeclrW<NmiclrSpec> {
        RpeclrW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM ECC Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn reccclr(&mut self) -> ReccclrW<NmiclrSpec> {
        ReccclrW::new(self, 9)
    }
    #[doc = "Bit 12 - Bus Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn busclr(&mut self) -> BusclrW<NmiclrSpec> {
        BusclrW::new(self, 12)
    }
}
#[doc = "Non-Maskable Interrupt Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiclrSpec;
impl crate::RegisterSpec for NmiclrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nmiclr::R`](R) reader structure"]
impl crate::Readable for NmiclrSpec {}
#[doc = "`write(|w| ..)` method takes [`nmiclr::W`](W) writer structure"]
impl crate::Writable for NmiclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets NMICLR to value 0"]
impl crate::Resettable for NmiclrSpec {
    const RESET_VALUE: u16 = 0;
}
