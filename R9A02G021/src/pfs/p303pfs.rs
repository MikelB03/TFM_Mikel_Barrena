#[doc = "Register `P303PFS` reader"]
pub type R = crate::R<P303pfsSpec>;
#[doc = "Register `P303PFS` writer"]
pub type W = crate::W<P303pfsSpec>;
#[doc = "Port Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Podr> for bool {
    #[inline(always)]
    fn from(variant: Podr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PODR` reader - Port Output Data"]
pub type PodrR = crate::BitReader<Podr>;
impl PodrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Podr {
        match self.bits {
            false => Podr::_0,
            true => Podr::_1,
        }
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr::_1
    }
}
#[doc = "Field `PODR` writer - Port Output Data"]
pub type PodrW<'a, REG> = crate::BitWriter<'a, REG, Podr>;
impl<'a, REG> PodrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_1)
    }
}
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr> for bool {
    #[inline(always)]
    fn from(variant: Pidr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR` reader - Pmn State"]
pub type PidrR = crate::BitReader<Pidr>;
impl PidrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr {
        match self.bits {
            false => Pidr::_0,
            true => Pidr::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr::_1
    }
}
#[doc = "Port Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr> for bool {
    #[inline(always)]
    fn from(variant: Pdr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR` reader - Port Direction"]
pub type PdrR = crate::BitReader<Pdr>;
impl PdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr {
        match self.bits {
            false => Pdr::_0,
            true => Pdr::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr::_1
    }
}
#[doc = "Field `PDR` writer - Port Direction"]
pub type PdrW<'a, REG> = crate::BitWriter<'a, REG, Pdr>;
impl<'a, REG> PdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_1)
    }
}
#[doc = "Pull-up Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcr {
    #[doc = "0: Disable input pull-up"]
    _0 = 0,
    #[doc = "1: Enable input pull-up"]
    _1 = 1,
}
impl From<Pcr> for bool {
    #[inline(always)]
    fn from(variant: Pcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCR` reader - Pull-up Control"]
pub type PcrR = crate::BitReader<Pcr>;
impl PcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcr {
        match self.bits {
            false => Pcr::_0,
            true => Pcr::_1,
        }
    }
    #[doc = "Disable input pull-up"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pcr::_0
    }
    #[doc = "Enable input pull-up"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pcr::_1
    }
}
#[doc = "Field `PCR` writer - Pull-up Control"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG, Pcr>;
impl<'a, REG> PcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable input pull-up"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::_0)
    }
    #[doc = "Enable input pull-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::_1)
    }
}
#[doc = "N-Channel Open-Drain Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncodr {
    #[doc = "0: CMOS output"]
    _0 = 0,
    #[doc = "1: NMOS open-drain output"]
    _1 = 1,
}
impl From<Ncodr> for bool {
    #[inline(always)]
    fn from(variant: Ncodr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCODR` reader - N-Channel Open-Drain Control"]
pub type NcodrR = crate::BitReader<Ncodr>;
impl NcodrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncodr {
        match self.bits {
            false => Ncodr::_0,
            true => Ncodr::_1,
        }
    }
    #[doc = "CMOS output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ncodr::_0
    }
    #[doc = "NMOS open-drain output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ncodr::_1
    }
}
#[doc = "Field `NCODR` writer - N-Channel Open-Drain Control"]
pub type NcodrW<'a, REG> = crate::BitWriter<'a, REG, Ncodr>;
impl<'a, REG> NcodrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMOS output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncodr::_0)
    }
    #[doc = "NMOS open-drain output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncodr::_1)
    }
}
#[doc = "Port Output Current Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dscr {
    #[doc = "0: 2 mA"]
    _00 = 0,
    #[doc = "1: 5 mA"]
    _01 = 1,
    #[doc = "2: 10 mA"]
    _1x = 2,
}
impl From<Dscr> for u8 {
    #[inline(always)]
    fn from(variant: Dscr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dscr {
    type Ux = u8;
}
impl crate::IsEnum for Dscr {}
#[doc = "Field `DSCR` reader - Port Output Current Control"]
pub type DscrR = crate::FieldReader<Dscr>;
impl DscrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dscr> {
        match self.bits {
            0 => Some(Dscr::_00),
            1 => Some(Dscr::_01),
            2 => Some(Dscr::_1x),
            _ => None,
        }
    }
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dscr::_00
    }
    #[doc = "5 mA"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dscr::_01
    }
    #[doc = "10 mA"]
    #[inline(always)]
    pub fn is_1x(&self) -> bool {
        *self == Dscr::_1x
    }
}
#[doc = "Field `DSCR` writer - Port Output Current Control"]
pub type DscrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dscr>;
impl<'a, REG> DscrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dscr::_00)
    }
    #[doc = "5 mA"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dscr::_01)
    }
    #[doc = "10 mA"]
    #[inline(always)]
    pub fn _1x(self) -> &'a mut crate::W<REG> {
        self.variant(Dscr::_1x)
    }
}
#[doc = "Event on Falling (EOF)/Event on Rising (EOR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eofr {
    #[doc = "0: Don't care"]
    _00 = 0,
    #[doc = "1: Detect rising edge"]
    _01 = 1,
    #[doc = "2: Detect falling edge"]
    _10 = 2,
    #[doc = "3: Detect both edges"]
    _11 = 3,
}
impl From<Eofr> for u8 {
    #[inline(always)]
    fn from(variant: Eofr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eofr {
    type Ux = u8;
}
impl crate::IsEnum for Eofr {}
#[doc = "Field `EOFR` reader - Event on Falling (EOF)/Event on Rising (EOR)"]
pub type EofrR = crate::FieldReader<Eofr>;
impl EofrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eofr {
        match self.bits {
            0 => Eofr::_00,
            1 => Eofr::_01,
            2 => Eofr::_10,
            3 => Eofr::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Don't care"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Eofr::_00
    }
    #[doc = "Detect rising edge"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Eofr::_01
    }
    #[doc = "Detect falling edge"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Eofr::_10
    }
    #[doc = "Detect both edges"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Eofr::_11
    }
}
#[doc = "Field `EOFR` writer - Event on Falling (EOF)/Event on Rising (EOR)"]
pub type EofrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eofr, crate::Safe>;
impl<'a, REG> EofrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Don't care"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Eofr::_00)
    }
    #[doc = "Detect rising edge"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Eofr::_01)
    }
    #[doc = "Detect falling edge"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Eofr::_10)
    }
    #[doc = "Detect both edges"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Eofr::_11)
    }
}
#[doc = "IRQ Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isel {
    #[doc = "0: Not used as an IRQn input pin"]
    _0 = 0,
    #[doc = "1: Used as an IRQn input pin"]
    _1 = 1,
}
impl From<Isel> for bool {
    #[inline(always)]
    fn from(variant: Isel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISEL` reader - IRQ Input Enable"]
pub type IselR = crate::BitReader<Isel>;
impl IselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isel {
        match self.bits {
            false => Isel::_0,
            true => Isel::_1,
        }
    }
    #[doc = "Not used as an IRQn input pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isel::_0
    }
    #[doc = "Used as an IRQn input pin"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isel::_1
    }
}
#[doc = "Field `ISEL` writer - IRQ Input Enable"]
pub type IselW<'a, REG> = crate::BitWriter<'a, REG, Isel>;
impl<'a, REG> IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not used as an IRQn input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_0)
    }
    #[doc = "Used as an IRQn input pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_1)
    }
}
#[doc = "Analog Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asel {
    #[doc = "0: Not used as an analog pin"]
    _0 = 0,
    #[doc = "1: Used as an analog pin"]
    _1 = 1,
}
impl From<Asel> for bool {
    #[inline(always)]
    fn from(variant: Asel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASEL` reader - Analog Input Enable"]
pub type AselR = crate::BitReader<Asel>;
impl AselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asel {
        match self.bits {
            false => Asel::_0,
            true => Asel::_1,
        }
    }
    #[doc = "Not used as an analog pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asel::_0
    }
    #[doc = "Used as an analog pin"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asel::_1
    }
}
#[doc = "Field `ASEL` writer - Analog Input Enable"]
pub type AselW<'a, REG> = crate::BitWriter<'a, REG, Asel>;
impl<'a, REG> AselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not used as an analog pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asel::_0)
    }
    #[doc = "Used as an analog pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asel::_1)
    }
}
#[doc = "Port Mode Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmr {
    #[doc = "0: Used as a general I/O pin"]
    _0 = 0,
    #[doc = "1: Used as an I/O port for peripheral functions"]
    _1 = 1,
}
impl From<Pmr> for bool {
    #[inline(always)]
    fn from(variant: Pmr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMR` reader - Port Mode Control"]
pub type PmrR = crate::BitReader<Pmr>;
impl PmrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmr {
        match self.bits {
            false => Pmr::_0,
            true => Pmr::_1,
        }
    }
    #[doc = "Used as a general I/O pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pmr::_0
    }
    #[doc = "Used as an I/O port for peripheral functions"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pmr::_1
    }
}
#[doc = "Field `PMR` writer - Port Mode Control"]
pub type PmrW<'a, REG> = crate::BitWriter<'a, REG, Pmr>;
impl<'a, REG> PmrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Used as a general I/O pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmr::_0)
    }
    #[doc = "Used as an I/O port for peripheral functions"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmr::_1)
    }
}
#[doc = "Field `PSEL` reader - Peripheral Select"]
pub type PselR = crate::FieldReader;
#[doc = "Field `PSEL` writer - Peripheral Select"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PodrR {
        PodrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pmn State"]
    #[inline(always)]
    pub fn pidr(&self) -> PidrR {
        PidrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PdrR {
        PdrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(&self) -> NcodrR {
        NcodrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port Output Current Control"]
    #[inline(always)]
    pub fn dscr(&self) -> DscrR {
        DscrR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Event on Falling (EOF)/Event on Rising (EOR)"]
    #[inline(always)]
    pub fn eofr(&self) -> EofrR {
        EofrR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Input Enable"]
    #[inline(always)]
    pub fn asel(&self) -> AselR {
        AselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Mode Control"]
    #[inline(always)]
    pub fn pmr(&self) -> PmrR {
        PmrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Peripheral Select"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    pub fn podr(&mut self) -> PodrW<P303pfsSpec> {
        PodrW::new(self, 0)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    pub fn pdr(&mut self) -> PdrW<P303pfsSpec> {
        PdrW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    pub fn pcr(&mut self) -> PcrW<P303pfsSpec> {
        PcrW::new(self, 4)
    }
    #[doc = "Bit 6 - N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(&mut self) -> NcodrW<P303pfsSpec> {
        NcodrW::new(self, 6)
    }
    #[doc = "Bits 10:11 - Port Output Current Control"]
    #[inline(always)]
    pub fn dscr(&mut self) -> DscrW<P303pfsSpec> {
        DscrW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Event on Falling (EOF)/Event on Rising (EOR)"]
    #[inline(always)]
    pub fn eofr(&mut self) -> EofrW<P303pfsSpec> {
        EofrW::new(self, 12)
    }
    #[doc = "Bit 14 - IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<P303pfsSpec> {
        IselW::new(self, 14)
    }
    #[doc = "Bit 15 - Analog Input Enable"]
    #[inline(always)]
    pub fn asel(&mut self) -> AselW<P303pfsSpec> {
        AselW::new(self, 15)
    }
    #[doc = "Bit 16 - Port Mode Control"]
    #[inline(always)]
    pub fn pmr(&mut self) -> PmrW<P303pfsSpec> {
        PmrW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Peripheral Select"]
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<P303pfsSpec> {
        PselW::new(self, 24)
    }
}
#[doc = "Port 303 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p303pfs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p303pfs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P303pfsSpec;
impl crate::RegisterSpec for P303pfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p303pfs::R`](R) reader structure"]
impl crate::Readable for P303pfsSpec {}
#[doc = "`write(|w| ..)` method takes [`p303pfs::W`](W) writer structure"]
impl crate::Writable for P303pfsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets P303PFS to value 0x0001_0010"]
impl crate::Resettable for P303pfsSpec {
    const RESET_VALUE: u32 = 0x0001_0010;
}
