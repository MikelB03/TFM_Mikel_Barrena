#[doc = "Register `TMR04` reader"]
pub type R = crate::R<Tmr04Spec>;
#[doc = "Register `TMR04` writer"]
pub type W = crate::W<Tmr04Spec>;
#[doc = "Field `OPIRQ` reader - Setting of starting count and interrupt"]
pub type OpirqR = crate::BitReader;
#[doc = "Field `OPIRQ` writer - Setting of starting count and interrupt"]
pub type OpirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selection of operation mode at channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Md {
    #[doc = "0: Interval timer mode"]
    _000 = 0,
    #[doc = "2: Capture mode"]
    _010 = 2,
    #[doc = "3: Event counter mode"]
    _011 = 3,
    #[doc = "4: One-count mode"]
    _100 = 4,
    #[doc = "6: Capture and one-count mode"]
    _110 = 6,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Md> for u8 {
    #[inline(always)]
    fn from(variant: Md) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Md {
    type Ux = u8;
}
impl crate::IsEnum for Md {}
#[doc = "Field `MD` reader - Selection of operation mode at channel n"]
pub type MdR = crate::FieldReader<Md>;
impl MdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Md {
        match self.bits {
            0 => Md::_000,
            2 => Md::_010,
            3 => Md::_011,
            4 => Md::_100,
            6 => Md::_110,
            _ => Md::Others,
        }
    }
    #[doc = "Interval timer mode"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Md::_000
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Md::_010
    }
    #[doc = "Event counter mode"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Md::_011
    }
    #[doc = "One-count mode"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Md::_100
    }
    #[doc = "Capture and one-count mode"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Md::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Md::Others)
    }
}
#[doc = "Field `MD` writer - Selection of operation mode at channel n"]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Md, crate::Safe>;
impl<'a, REG> MdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interval timer mode"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_000)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_010)
    }
    #[doc = "Event counter mode"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_011)
    }
    #[doc = "One-count mode"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_100)
    }
    #[doc = "Capture and one-count mode"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Md::Others)
    }
}
#[doc = "Selection of TI0n pin input valid edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cis {
    #[doc = "0: Falling edge"]
    _00 = 0,
    #[doc = "1: Rising edge"]
    _01 = 1,
    #[doc = "2: Both edges (when low-level width is measured) Start trigger: Falling edge, Capture trigger: Rising edge"]
    _10 = 2,
    #[doc = "3: Both edges (when high-level width is measured) Start trigger: Rising edge, Capture trigger: Falling edge"]
    _11 = 3,
}
impl From<Cis> for u8 {
    #[inline(always)]
    fn from(variant: Cis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cis {
    type Ux = u8;
}
impl crate::IsEnum for Cis {}
#[doc = "Field `CIS` reader - Selection of TI0n pin input valid edge"]
pub type CisR = crate::FieldReader<Cis>;
impl CisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cis {
        match self.bits {
            0 => Cis::_00,
            1 => Cis::_01,
            2 => Cis::_10,
            3 => Cis::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cis::_00
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cis::_01
    }
    #[doc = "Both edges (when low-level width is measured) Start trigger: Falling edge, Capture trigger: Rising edge"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cis::_10
    }
    #[doc = "Both edges (when high-level width is measured) Start trigger: Rising edge, Capture trigger: Falling edge"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cis::_11
    }
}
#[doc = "Field `CIS` writer - Selection of TI0n pin input valid edge"]
pub type CisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cis, crate::Safe>;
impl<'a, REG> CisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cis::_00)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cis::_01)
    }
    #[doc = "Both edges (when low-level width is measured) Start trigger: Falling edge, Capture trigger: Rising edge"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cis::_10)
    }
    #[doc = "Both edges (when high-level width is measured) Start trigger: Rising edge, Capture trigger: Falling edge"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cis::_11)
    }
}
#[doc = "Setting of start trigger or capture trigger of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sts {
    #[doc = "0: Only software trigger start is valid (other trigger sources are unselected)."]
    _000 = 0,
    #[doc = "1: Valid edge of the TI0n pin input is used as both the start trigger and capture trigger."]
    _001 = 1,
    #[doc = "2: Both the edges of the TI0n pin input are used as a start trigger and a capture trigger."]
    _010 = 2,
    #[doc = "4: Interrupt signal of the master channel is used (when the channel is used as a slave channel with the simultaneous channel operation function)."]
    _100 = 4,
    #[doc = "3: Setting prohibited"]
    Others = 3,
}
impl From<Sts> for u8 {
    #[inline(always)]
    fn from(variant: Sts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sts {
    type Ux = u8;
}
impl crate::IsEnum for Sts {}
#[doc = "Field `STS` reader - Setting of start trigger or capture trigger of channel n"]
pub type StsR = crate::FieldReader<Sts>;
impl StsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sts {
        match self.bits {
            0 => Sts::_000,
            1 => Sts::_001,
            2 => Sts::_010,
            4 => Sts::_100,
            _ => Sts::Others,
        }
    }
    #[doc = "Only software trigger start is valid (other trigger sources are unselected)."]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Sts::_000
    }
    #[doc = "Valid edge of the TI0n pin input is used as both the start trigger and capture trigger."]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Sts::_001
    }
    #[doc = "Both the edges of the TI0n pin input are used as a start trigger and a capture trigger."]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Sts::_010
    }
    #[doc = "Interrupt signal of the master channel is used (when the channel is used as a slave channel with the simultaneous channel operation function)."]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Sts::_100
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Sts::Others)
    }
}
#[doc = "Field `STS` writer - Setting of start trigger or capture trigger of channel n"]
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sts, crate::Safe>;
impl<'a, REG> StsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only software trigger start is valid (other trigger sources are unselected)."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::_000)
    }
    #[doc = "Valid edge of the TI0n pin input is used as both the start trigger and capture trigger."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::_001)
    }
    #[doc = "Both the edges of the TI0n pin input are used as a start trigger and a capture trigger."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::_010)
    }
    #[doc = "Interrupt signal of the master channel is used (when the channel is used as a slave channel with the simultaneous channel operation function)."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::_100)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Others)
    }
}
#[doc = "Selection between using channel n independently or simultaneously with another channel (as a slave or master)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Master {
    #[doc = "0: Operates in independent channel operation function or as slave channel in simultaneous channel operation function."]
    _0 = 0,
    #[doc = "1: Operates as master channel in simultaneous channel operation function."]
    _1 = 1,
}
impl From<Master> for bool {
    #[inline(always)]
    fn from(variant: Master) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER` reader - Selection between using channel n independently or simultaneously with another channel (as a slave or master)"]
pub type MasterR = crate::BitReader<Master>;
impl MasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Master {
        match self.bits {
            false => Master::_0,
            true => Master::_1,
        }
    }
    #[doc = "Operates in independent channel operation function or as slave channel in simultaneous channel operation function."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Master::_0
    }
    #[doc = "Operates as master channel in simultaneous channel operation function."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Master::_1
    }
}
#[doc = "Field `MASTER` writer - Selection between using channel n independently or simultaneously with another channel (as a slave or master)"]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG, Master>;
impl<'a, REG> MasterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operates in independent channel operation function or as slave channel in simultaneous channel operation function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Master::_0)
    }
    #[doc = "Operates as master channel in simultaneous channel operation function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Master::_1)
    }
}
#[doc = "Selection of counter clock (fTCLK) of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccs {
    #[doc = "0: Operating clock (fMCK) specified by the CKS\\[1:0\\]
bits"]
    _0 = 0,
    #[doc = "1: Valid edge of input signal input from the TI0n pin. In channel 5, valid edge of input signal selected by the TIS0 registerIn channel 7, valid edge of input signal selected by the ISC register"]
    _1 = 1,
}
impl From<Ccs> for bool {
    #[inline(always)]
    fn from(variant: Ccs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCS` reader - Selection of counter clock (fTCLK) of channel n"]
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
    #[doc = "Operating clock (fMCK) specified by the CKS\\[1:0\\]
bits"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ccs::_0
    }
    #[doc = "Valid edge of input signal input from the TI0n pin. In channel 5, valid edge of input signal selected by the TIS0 registerIn channel 7, valid edge of input signal selected by the ISC register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ccs::_1
    }
}
#[doc = "Field `CCS` writer - Selection of counter clock (fTCLK) of channel n"]
pub type CcsW<'a, REG> = crate::BitWriter<'a, REG, Ccs>;
impl<'a, REG> CcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operating clock (fMCK) specified by the CKS\\[1:0\\]
bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccs::_0)
    }
    #[doc = "Valid edge of input signal input from the TI0n pin. In channel 5, valid edge of input signal selected by the TIS0 registerIn channel 7, valid edge of input signal selected by the ISC register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccs::_1)
    }
}
#[doc = "Selection of operation clock (fMCK) of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    #[doc = "0: Operation clock CK00 set by timer clock select register 0 (TPS0)"]
    _00 = 0,
    #[doc = "1: Operation clock CK02 set by timer clock select register 0 (TPS0)"]
    _01 = 1,
    #[doc = "2: Operation clock CK01 set by timer clock select register 0 (TPS0)"]
    _10 = 2,
    #[doc = "3: Operation clock CK03 set by timer clock select register 0 (TPS0)"]
    _11 = 3,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
#[doc = "Field `CKS` reader - Selection of operation clock (fMCK) of channel n"]
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            0 => Cks::_00,
            1 => Cks::_01,
            2 => Cks::_10,
            3 => Cks::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Operation clock CK00 set by timer clock select register 0 (TPS0)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cks::_00
    }
    #[doc = "Operation clock CK02 set by timer clock select register 0 (TPS0)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cks::_01
    }
    #[doc = "Operation clock CK01 set by timer clock select register 0 (TPS0)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cks::_10
    }
    #[doc = "Operation clock CK03 set by timer clock select register 0 (TPS0)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cks::_11
    }
}
#[doc = "Field `CKS` writer - Selection of operation clock (fMCK) of channel n"]
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cks, crate::Safe>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Operation clock CK00 set by timer clock select register 0 (TPS0)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_00)
    }
    #[doc = "Operation clock CK02 set by timer clock select register 0 (TPS0)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_01)
    }
    #[doc = "Operation clock CK01 set by timer clock select register 0 (TPS0)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_10)
    }
    #[doc = "Operation clock CK03 set by timer clock select register 0 (TPS0)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Setting of starting count and interrupt"]
    #[inline(always)]
    pub fn opirq(&self) -> OpirqR {
        OpirqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Selection of operation mode at channel n"]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Selection of TI0n pin input valid edge"]
    #[inline(always)]
    pub fn cis(&self) -> CisR {
        CisR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Setting of start trigger or capture trigger of channel n"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Selection between using channel n independently or simultaneously with another channel (as a slave or master)"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selection of counter clock (fTCLK) of channel n"]
    #[inline(always)]
    pub fn ccs(&self) -> CcsR {
        CcsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Selection of operation clock (fMCK) of channel n"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Setting of starting count and interrupt"]
    #[inline(always)]
    pub fn opirq(&mut self) -> OpirqW<Tmr04Spec> {
        OpirqW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Selection of operation mode at channel n"]
    #[inline(always)]
    pub fn md(&mut self) -> MdW<Tmr04Spec> {
        MdW::new(self, 1)
    }
    #[doc = "Bits 6:7 - Selection of TI0n pin input valid edge"]
    #[inline(always)]
    pub fn cis(&mut self) -> CisW<Tmr04Spec> {
        CisW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Setting of start trigger or capture trigger of channel n"]
    #[inline(always)]
    pub fn sts(&mut self) -> StsW<Tmr04Spec> {
        StsW::new(self, 8)
    }
    #[doc = "Bit 11 - Selection between using channel n independently or simultaneously with another channel (as a slave or master)"]
    #[inline(always)]
    pub fn master(&mut self) -> MasterW<Tmr04Spec> {
        MasterW::new(self, 11)
    }
    #[doc = "Bit 12 - Selection of counter clock (fTCLK) of channel n"]
    #[inline(always)]
    pub fn ccs(&mut self) -> CcsW<Tmr04Spec> {
        CcsW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Selection of operation clock (fMCK) of channel n"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<Tmr04Spec> {
        CksW::new(self, 14)
    }
}
#[doc = "Timer Mode Register 04\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr04Spec;
impl crate::RegisterSpec for Tmr04Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tmr04::R`](R) reader structure"]
impl crate::Readable for Tmr04Spec {}
#[doc = "`write(|w| ..)` method takes [`tmr04::W`](W) writer structure"]
impl crate::Writable for Tmr04Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TMR04 to value 0"]
impl crate::Resettable for Tmr04Spec {
    const RESET_VALUE: u16 = 0;
}
