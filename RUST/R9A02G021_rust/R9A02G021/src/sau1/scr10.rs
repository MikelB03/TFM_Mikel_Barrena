#[doc = "Register `SCR10` reader"]
pub type R = crate::R<Scr10Spec>;
#[doc = "Register `SCR10` writer"]
pub type W = crate::W<Scr10Spec>;
#[doc = "Setting of data length in simplified SPI and UART modes\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dls {
    #[doc = "0: Setting prohibited"]
    _00 = 0,
    #[doc = "1: 9-bit data length (stored in the DAT\\[8:0\\]
bits of the SDRm0 register) (settable in UART mode only)"]
    _01 = 1,
    #[doc = "2: 7-bit data length (stored in the DAT\\[6:0\\]
bits of the SDRm0 register)"]
    _10 = 2,
    #[doc = "3: 8-bit data length (stored in the DAT\\[7:0\\]
bits of the SDRm0 register)"]
    _11 = 3,
}
impl From<Dls> for u8 {
    #[inline(always)]
    fn from(variant: Dls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dls {
    type Ux = u8;
}
impl crate::IsEnum for Dls {}
#[doc = "Field `DLS` reader - Setting of data length in simplified SPI and UART modes"]
pub type DlsR = crate::FieldReader<Dls>;
impl DlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dls {
        match self.bits {
            0 => Dls::_00,
            1 => Dls::_01,
            2 => Dls::_10,
            3 => Dls::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dls::_00
    }
    #[doc = "9-bit data length (stored in the DAT\\[8:0\\]
bits of the SDRm0 register) (settable in UART mode only)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dls::_01
    }
    #[doc = "7-bit data length (stored in the DAT\\[6:0\\]
bits of the SDRm0 register)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dls::_10
    }
    #[doc = "8-bit data length (stored in the DAT\\[7:0\\]
bits of the SDRm0 register)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dls::_11
    }
}
#[doc = "Field `DLS` writer - Setting of data length in simplified SPI and UART modes"]
pub type DlsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dls, crate::Safe>;
impl<'a, REG> DlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dls::_00)
    }
    #[doc = "9-bit data length (stored in the DAT\\[8:0\\]
bits of the SDRm0 register) (settable in UART mode only)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dls::_01)
    }
    #[doc = "7-bit data length (stored in the DAT\\[6:0\\]
bits of the SDRm0 register)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dls::_10)
    }
    #[doc = "8-bit data length (stored in the DAT\\[7:0\\]
bits of the SDRm0 register)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dls::_11)
    }
}
#[doc = "Setting of stop bit in UART mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slc {
    #[doc = "0: No stop bit"]
    _00 = 0,
    #[doc = "1: Stop bit length = 1 bit"]
    _01 = 1,
    #[doc = "2: Stop bit length = 2 bits"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Slc> for u8 {
    #[inline(always)]
    fn from(variant: Slc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slc {
    type Ux = u8;
}
impl crate::IsEnum for Slc {}
#[doc = "Field `SLC` reader - Setting of stop bit in UART mode"]
pub type SlcR = crate::FieldReader<Slc>;
impl SlcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slc {
        match self.bits {
            0 => Slc::_00,
            1 => Slc::_01,
            2 => Slc::_10,
            3 => Slc::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No stop bit"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Slc::_00
    }
    #[doc = "Stop bit length = 1 bit"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Slc::_01
    }
    #[doc = "Stop bit length = 2 bits"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Slc::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Slc::_11
    }
}
#[doc = "Field `SLC` writer - Setting of stop bit in UART mode"]
pub type SlcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Slc, crate::Safe>;
impl<'a, REG> SlcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No stop bit"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Slc::_00)
    }
    #[doc = "Stop bit length = 1 bit"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Slc::_01)
    }
    #[doc = "Stop bit length = 2 bits"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Slc::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Slc::_11)
    }
}
#[doc = "Selection of data transfer sequence in simplified SPI and UART modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Inputs or outputs data with MSB first"]
    _0 = 0,
    #[doc = "1: Inputs or outputs data with LSB first"]
    _1 = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Selection of data transfer sequence in simplified SPI and UART modes"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::_0,
            true => Dir::_1,
        }
    }
    #[doc = "Inputs or outputs data with MSB first"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dir::_0
    }
    #[doc = "Inputs or outputs data with LSB first"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dir::_1
    }
}
#[doc = "Field `DIR` writer - Selection of data transfer sequence in simplified SPI and UART modes"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inputs or outputs data with MSB first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_0)
    }
    #[doc = "Inputs or outputs data with LSB first"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_1)
    }
}
#[doc = "Setting of parity bit in UART mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ptc {
    #[doc = "0: Transmission: Does not output the parity bit Reception: Receives without parity"]
    _00 = 0,
    #[doc = "1: Transmission: Outputs 0 parity Reception: No parity determination"]
    _01 = 1,
    #[doc = "2: Transmission: Outputs even parity Reception: Determines as even parity"]
    _10 = 2,
    #[doc = "3: Transmission: Outputs odd parity Reception: Determines as odd parity"]
    _11 = 3,
}
impl From<Ptc> for u8 {
    #[inline(always)]
    fn from(variant: Ptc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptc {
    type Ux = u8;
}
impl crate::IsEnum for Ptc {}
#[doc = "Field `PTC` reader - Setting of parity bit in UART mode"]
pub type PtcR = crate::FieldReader<Ptc>;
impl PtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptc {
        match self.bits {
            0 => Ptc::_00,
            1 => Ptc::_01,
            2 => Ptc::_10,
            3 => Ptc::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Transmission: Does not output the parity bit Reception: Receives without parity"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ptc::_00
    }
    #[doc = "Transmission: Outputs 0 parity Reception: No parity determination"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ptc::_01
    }
    #[doc = "Transmission: Outputs even parity Reception: Determines as even parity"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ptc::_10
    }
    #[doc = "Transmission: Outputs odd parity Reception: Determines as odd parity"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ptc::_11
    }
}
#[doc = "Field `PTC` writer - Setting of parity bit in UART mode"]
pub type PtcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ptc, crate::Safe>;
impl<'a, REG> PtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmission: Does not output the parity bit Reception: Receives without parity"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ptc::_00)
    }
    #[doc = "Transmission: Outputs 0 parity Reception: No parity determination"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ptc::_01)
    }
    #[doc = "Transmission: Outputs even parity Reception: Determines as even parity"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ptc::_10)
    }
    #[doc = "Transmission: Outputs odd parity Reception: Determines as odd parity"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ptc::_11)
    }
}
#[doc = "Selection of data and clock phase in simplified SPI mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcp {
    #[doc = "0: Type1 (SCK: inverted, Input timing: rising edge)"]
    _00 = 0,
    #[doc = "1: Type2 (SCK: non-inverted, Input timing: falling edge)"]
    _01 = 1,
    #[doc = "2: Type3 (SCK: inverted, Input timing: falling edge)"]
    _10 = 2,
    #[doc = "3: Type4 (SCK: non-inverted, Input timing: rising edge)"]
    _11 = 3,
}
impl From<Dcp> for u8 {
    #[inline(always)]
    fn from(variant: Dcp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcp {
    type Ux = u8;
}
impl crate::IsEnum for Dcp {}
#[doc = "Field `DCP` reader - Selection of data and clock phase in simplified SPI mode"]
pub type DcpR = crate::FieldReader<Dcp>;
impl DcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcp {
        match self.bits {
            0 => Dcp::_00,
            1 => Dcp::_01,
            2 => Dcp::_10,
            3 => Dcp::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Type1 (SCK: inverted, Input timing: rising edge)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dcp::_00
    }
    #[doc = "Type2 (SCK: non-inverted, Input timing: falling edge)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dcp::_01
    }
    #[doc = "Type3 (SCK: inverted, Input timing: falling edge)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dcp::_10
    }
    #[doc = "Type4 (SCK: non-inverted, Input timing: rising edge)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dcp::_11
    }
}
#[doc = "Field `DCP` writer - Selection of data and clock phase in simplified SPI mode"]
pub type DcpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dcp, crate::Safe>;
impl<'a, REG> DcpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Type1 (SCK: inverted, Input timing: rising edge)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dcp::_00)
    }
    #[doc = "Type2 (SCK: non-inverted, Input timing: falling edge)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dcp::_01)
    }
    #[doc = "Type3 (SCK: inverted, Input timing: falling edge)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dcp::_10)
    }
    #[doc = "Type4 (SCK: non-inverted, Input timing: rising edge)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dcp::_11)
    }
}
#[doc = "Setting of channel 0 operation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trxe {
    #[doc = "0: Disable communication"]
    _00 = 0,
    #[doc = "1: Reception only"]
    _01 = 1,
    #[doc = "2: Transmission only"]
    _10 = 2,
    #[doc = "3: Transmission and reception"]
    _11 = 3,
}
impl From<Trxe> for u8 {
    #[inline(always)]
    fn from(variant: Trxe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trxe {
    type Ux = u8;
}
impl crate::IsEnum for Trxe {}
#[doc = "Field `TRXE` reader - Setting of channel 0 operation mode"]
pub type TrxeR = crate::FieldReader<Trxe>;
impl TrxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trxe {
        match self.bits {
            0 => Trxe::_00,
            1 => Trxe::_01,
            2 => Trxe::_10,
            3 => Trxe::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable communication"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Trxe::_00
    }
    #[doc = "Reception only"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Trxe::_01
    }
    #[doc = "Transmission only"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Trxe::_10
    }
    #[doc = "Transmission and reception"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Trxe::_11
    }
}
#[doc = "Field `TRXE` writer - Setting of channel 0 operation mode"]
pub type TrxeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trxe, crate::Safe>;
impl<'a, REG> TrxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable communication"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Trxe::_00)
    }
    #[doc = "Reception only"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Trxe::_01)
    }
    #[doc = "Transmission only"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Trxe::_10)
    }
    #[doc = "Transmission and reception"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Trxe::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Setting of data length in simplified SPI and UART modes"]
    #[inline(always)]
    pub fn dls(&self) -> DlsR {
        DlsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Setting of stop bit in UART mode"]
    #[inline(always)]
    pub fn slc(&self) -> SlcR {
        SlcR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Selection of data transfer sequence in simplified SPI and UART modes"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Setting of parity bit in UART mode"]
    #[inline(always)]
    pub fn ptc(&self) -> PtcR {
        PtcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Selection of data and clock phase in simplified SPI mode"]
    #[inline(always)]
    pub fn dcp(&self) -> DcpR {
        DcpR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Setting of channel 0 operation mode"]
    #[inline(always)]
    pub fn trxe(&self) -> TrxeR {
        TrxeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Setting of data length in simplified SPI and UART modes"]
    #[inline(always)]
    pub fn dls(&mut self) -> DlsW<Scr10Spec> {
        DlsW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Setting of stop bit in UART mode"]
    #[inline(always)]
    pub fn slc(&mut self) -> SlcW<Scr10Spec> {
        SlcW::new(self, 4)
    }
    #[doc = "Bit 7 - Selection of data transfer sequence in simplified SPI and UART modes"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<Scr10Spec> {
        DirW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Setting of parity bit in UART mode"]
    #[inline(always)]
    pub fn ptc(&mut self) -> PtcW<Scr10Spec> {
        PtcW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Selection of data and clock phase in simplified SPI mode"]
    #[inline(always)]
    pub fn dcp(&mut self) -> DcpW<Scr10Spec> {
        DcpW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Setting of channel 0 operation mode"]
    #[inline(always)]
    pub fn trxe(&mut self) -> TrxeW<Scr10Spec> {
        TrxeW::new(self, 14)
    }
}
#[doc = "Serial Communication Operation Setting Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`scr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr10Spec;
impl crate::RegisterSpec for Scr10Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`scr10::R`](R) reader structure"]
impl crate::Readable for Scr10Spec {}
#[doc = "`write(|w| ..)` method takes [`scr10::W`](W) writer structure"]
impl crate::Writable for Scr10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SCR10 to value 0x87"]
impl crate::Resettable for Scr10Spec {
    const RESET_VALUE: u16 = 0x87;
}
