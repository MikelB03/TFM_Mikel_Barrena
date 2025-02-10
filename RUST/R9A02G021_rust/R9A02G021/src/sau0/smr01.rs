#[doc = "Register `SMR01` reader"]
pub type R = crate::R<Smr01Spec>;
#[doc = "Register `SMR01` writer"]
pub type W = crate::W<Smr01Spec>;
#[doc = "Selection of channel n interrupt source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Md0 {
    #[doc = "0: Transfer end interrupt"]
    _0 = 0,
    #[doc = "1: Buffer empty interrupt (occurs when data is transferred from the SDRmn register to the shift register.)"]
    _1 = 1,
}
impl From<Md0> for bool {
    #[inline(always)]
    fn from(variant: Md0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MD0` reader - Selection of channel n interrupt source"]
pub type Md0R = crate::BitReader<Md0>;
impl Md0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Md0 {
        match self.bits {
            false => Md0::_0,
            true => Md0::_1,
        }
    }
    #[doc = "Transfer end interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Md0::_0
    }
    #[doc = "Buffer empty interrupt (occurs when data is transferred from the SDRmn register to the shift register.)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Md0::_1
    }
}
#[doc = "Field `MD0` writer - Selection of channel n interrupt source"]
pub type Md0W<'a, REG> = crate::BitWriter<'a, REG, Md0>;
impl<'a, REG> Md0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Md0::_0)
    }
    #[doc = "Buffer empty interrupt (occurs when data is transferred from the SDRmn register to the shift register.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Md0::_1)
    }
}
#[doc = "Setting of channel n operation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Md1 {
    #[doc = "0: Simplified SPI mode"]
    _00 = 0,
    #[doc = "1: UART mode"]
    _01 = 1,
    #[doc = "2: Simplified I2C mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Md1> for u8 {
    #[inline(always)]
    fn from(variant: Md1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Md1 {
    type Ux = u8;
}
impl crate::IsEnum for Md1 {}
#[doc = "Field `MD1` reader - Setting of channel n operation mode"]
pub type Md1R = crate::FieldReader<Md1>;
impl Md1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Md1 {
        match self.bits {
            0 => Md1::_00,
            1 => Md1::_01,
            2 => Md1::_10,
            3 => Md1::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Simplified SPI mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Md1::_00
    }
    #[doc = "UART mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Md1::_01
    }
    #[doc = "Simplified I2C mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Md1::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Md1::_11
    }
}
#[doc = "Field `MD1` writer - Setting of channel n operation mode"]
pub type Md1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Md1, crate::Safe>;
impl<'a, REG> Md1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Simplified SPI mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Md1::_00)
    }
    #[doc = "UART mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Md1::_01)
    }
    #[doc = "Simplified I2C mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Md1::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Md1::_11)
    }
}
#[doc = "Controls inversion of level of channel n receive data in UART mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sis0 {
    #[doc = "0: Falling edge is detected as the start bit. The input communication data is captured as is."]
    _0 = 0,
    #[doc = "1: Rising edge is detected as the start bit. The input communication data is inverted and captured."]
    _1 = 1,
}
impl From<Sis0> for bool {
    #[inline(always)]
    fn from(variant: Sis0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIS0` reader - Controls inversion of level of channel n receive data in UART mode"]
pub type Sis0R = crate::BitReader<Sis0>;
impl Sis0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sis0 {
        match self.bits {
            false => Sis0::_0,
            true => Sis0::_1,
        }
    }
    #[doc = "Falling edge is detected as the start bit. The input communication data is captured as is."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sis0::_0
    }
    #[doc = "Rising edge is detected as the start bit. The input communication data is inverted and captured."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sis0::_1
    }
}
#[doc = "Field `SIS0` writer - Controls inversion of level of channel n receive data in UART mode"]
pub type Sis0W<'a, REG> = crate::BitWriter<'a, REG, Sis0>;
impl<'a, REG> Sis0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge is detected as the start bit. The input communication data is captured as is."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sis0::_0)
    }
    #[doc = "Rising edge is detected as the start bit. The input communication data is inverted and captured."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sis0::_1)
    }
}
#[doc = "Selection of start trigger source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sts {
    #[doc = "0: Only software trigger is valid (selected for simplified SPI, UART transmission, and simplified I2C)"]
    _0 = 0,
    #[doc = "1: Valid edge of the RxDq pin (selected for UART reception)"]
    _1 = 1,
}
impl From<Sts> for bool {
    #[inline(always)]
    fn from(variant: Sts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STS` reader - Selection of start trigger source"]
pub type StsR = crate::BitReader<Sts>;
impl StsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sts {
        match self.bits {
            false => Sts::_0,
            true => Sts::_1,
        }
    }
    #[doc = "Only software trigger is valid (selected for simplified SPI, UART transmission, and simplified I2C)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sts::_0
    }
    #[doc = "Valid edge of the RxDq pin (selected for UART reception)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sts::_1
    }
}
#[doc = "Field `STS` writer - Selection of start trigger source"]
pub type StsW<'a, REG> = crate::BitWriter<'a, REG, Sts>;
impl<'a, REG> StsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only software trigger is valid (selected for simplified SPI, UART transmission, and simplified I2C)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::_0)
    }
    #[doc = "Valid edge of the RxDq pin (selected for UART reception)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::_1)
    }
}
#[doc = "Selection of transfer clock (fTCLK) of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccs {
    #[doc = "0: Divided operation clock fMCK specified by the CKS bit"]
    _0 = 0,
    #[doc = "1: Clock input fSCK from the SCKp pin (slave transfer in simplified SPI mode)"]
    _1 = 1,
}
impl From<Ccs> for bool {
    #[inline(always)]
    fn from(variant: Ccs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCS` reader - Selection of transfer clock (fTCLK) of channel n"]
pub type CcsR = crate::BitReader<Ccs>;
impl CcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccs {
        match self.bits {
            false => Ccs::_0,
            true => Ccs::_1,
        }
    }
    #[doc = "Divided operation clock fMCK specified by the CKS bit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ccs::_0
    }
    #[doc = "Clock input fSCK from the SCKp pin (slave transfer in simplified SPI mode)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ccs::_1
    }
}
#[doc = "Field `CCS` writer - Selection of transfer clock (fTCLK) of channel n"]
pub type CcsW<'a, REG> = crate::BitWriter<'a, REG, Ccs>;
impl<'a, REG> CcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divided operation clock fMCK specified by the CKS bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccs::_0)
    }
    #[doc = "Clock input fSCK from the SCKp pin (slave transfer in simplified SPI mode)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccs::_1)
    }
}
#[doc = "Selection of operation clock (fMCK) of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cks {
    #[doc = "0: Operation clock CKm0 set by the SPSm register"]
    _0 = 0,
    #[doc = "1: Operation clock CKm1 set by the SPSm register"]
    _1 = 1,
}
impl From<Cks> for bool {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKS` reader - Selection of operation clock (fMCK) of channel n"]
pub type CksR = crate::BitReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            false => Cks::_0,
            true => Cks::_1,
        }
    }
    #[doc = "Operation clock CKm0 set by the SPSm register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cks::_0
    }
    #[doc = "Operation clock CKm1 set by the SPSm register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cks::_1
    }
}
#[doc = "Field `CKS` writer - Selection of operation clock (fMCK) of channel n"]
pub type CksW<'a, REG> = crate::BitWriter<'a, REG, Cks>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation clock CKm0 set by the SPSm register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0)
    }
    #[doc = "Operation clock CKm1 set by the SPSm register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Selection of channel n interrupt source"]
    #[inline(always)]
    pub fn md0(&self) -> Md0R {
        Md0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Setting of channel n operation mode"]
    #[inline(always)]
    pub fn md1(&self) -> Md1R {
        Md1R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 6 - Controls inversion of level of channel n receive data in UART mode"]
    #[inline(always)]
    pub fn sis0(&self) -> Sis0R {
        Sis0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Selection of start trigger source"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - Selection of transfer clock (fTCLK) of channel n"]
    #[inline(always)]
    pub fn ccs(&self) -> CcsR {
        CcsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selection of operation clock (fMCK) of channel n"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selection of channel n interrupt source"]
    #[inline(always)]
    pub fn md0(&mut self) -> Md0W<Smr01Spec> {
        Md0W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Setting of channel n operation mode"]
    #[inline(always)]
    pub fn md1(&mut self) -> Md1W<Smr01Spec> {
        Md1W::new(self, 1)
    }
    #[doc = "Bit 6 - Controls inversion of level of channel n receive data in UART mode"]
    #[inline(always)]
    pub fn sis0(&mut self) -> Sis0W<Smr01Spec> {
        Sis0W::new(self, 6)
    }
    #[doc = "Bit 8 - Selection of start trigger source"]
    #[inline(always)]
    pub fn sts(&mut self) -> StsW<Smr01Spec> {
        StsW::new(self, 8)
    }
    #[doc = "Bit 14 - Selection of transfer clock (fTCLK) of channel n"]
    #[inline(always)]
    pub fn ccs(&mut self) -> CcsW<Smr01Spec> {
        CcsW::new(self, 14)
    }
    #[doc = "Bit 15 - Selection of operation clock (fMCK) of channel n"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<Smr01Spec> {
        CksW::new(self, 15)
    }
}
#[doc = "Serial Mode Register 01\n\nYou can [`read`](crate::Reg::read) this register and get [`smr01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smr01Spec;
impl crate::RegisterSpec for Smr01Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`smr01::R`](R) reader structure"]
impl crate::Readable for Smr01Spec {}
#[doc = "`write(|w| ..)` method takes [`smr01::W`](W) writer structure"]
impl crate::Writable for Smr01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SMR01 to value 0x20"]
impl crate::Resettable for Smr01Spec {
    const RESET_VALUE: u16 = 0x20;
}
