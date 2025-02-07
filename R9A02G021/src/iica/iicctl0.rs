#[doc = "Register `IICCTL%s0` reader"]
pub type R = crate::R<Iicctl0Spec>;
#[doc = "Register `IICCTL%s0` writer"]
pub type W = crate::W<Iicctl0Spec>;
#[doc = "Stop condition trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spt {
    #[doc = "0: Stop condition is not generated"]
    _0 = 0,
    #[doc = "1: Stop condition is generated (termination of master device transfer)"]
    _1 = 1,
}
impl From<Spt> for bool {
    #[inline(always)]
    fn from(variant: Spt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPT` reader - Stop condition trigger"]
pub type SptR = crate::BitReader<Spt>;
impl SptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spt {
        match self.bits {
            false => Spt::_0,
            true => Spt::_1,
        }
    }
    #[doc = "Stop condition is not generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spt::_0
    }
    #[doc = "Stop condition is generated (termination of master device transfer)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spt::_1
    }
}
#[doc = "Field `SPT` writer - Stop condition trigger"]
pub type SptW<'a, REG> = crate::BitWriter<'a, REG, Spt>;
impl<'a, REG> SptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop condition is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spt::_0)
    }
    #[doc = "Stop condition is generated (termination of master device transfer)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spt::_1)
    }
}
#[doc = "Start condition trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stt {
    #[doc = "0: Do not generate a start condition"]
    _0 = 0,
    #[doc = "1: When bus is released (in standby state, when IICFn.IICBSY = 0): If this bit is set to 1, a start condition is generated (startup as the master). When a 3rd-party is communicating: When communication reservation function is enabled (IICFn.IICRSV = 0):This bit functions as the start condition reservation flag. When set to 1, automatically generates a start condition after the bus is released.When communication reservation function is disabled (IICFn.IICRSV = 1):Even if this bit is set to 1, the STT bit is cleared and the STT clear flag (IICFn.STCF) is set to 1. No start condition is generated.In the clock stretch state (for a master device): Generates a restart condition after release from the clock stretch state."]
    _1 = 1,
}
impl From<Stt> for bool {
    #[inline(always)]
    fn from(variant: Stt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STT` reader - Start condition trigger"]
pub type SttR = crate::BitReader<Stt>;
impl SttR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stt {
        match self.bits {
            false => Stt::_0,
            true => Stt::_1,
        }
    }
    #[doc = "Do not generate a start condition"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stt::_0
    }
    #[doc = "When bus is released (in standby state, when IICFn.IICBSY = 0): If this bit is set to 1, a start condition is generated (startup as the master). When a 3rd-party is communicating: When communication reservation function is enabled (IICFn.IICRSV = 0):This bit functions as the start condition reservation flag. When set to 1, automatically generates a start condition after the bus is released.When communication reservation function is disabled (IICFn.IICRSV = 1):Even if this bit is set to 1, the STT bit is cleared and the STT clear flag (IICFn.STCF) is set to 1. No start condition is generated.In the clock stretch state (for a master device): Generates a restart condition after release from the clock stretch state."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stt::_1
    }
}
#[doc = "Field `STT` writer - Start condition trigger"]
pub type SttW<'a, REG> = crate::BitWriter<'a, REG, Stt>;
impl<'a, REG> SttW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not generate a start condition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stt::_0)
    }
    #[doc = "When bus is released (in standby state, when IICFn.IICBSY = 0): If this bit is set to 1, a start condition is generated (startup as the master). When a 3rd-party is communicating: When communication reservation function is enabled (IICFn.IICRSV = 0):This bit functions as the start condition reservation flag. When set to 1, automatically generates a start condition after the bus is released.When communication reservation function is disabled (IICFn.IICRSV = 1):Even if this bit is set to 1, the STT bit is cleared and the STT clear flag (IICFn.STCF) is set to 1. No start condition is generated.In the clock stretch state (for a master device): Generates a restart condition after release from the clock stretch state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stt::_1)
    }
}
#[doc = "Acknowledgment control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acke {
    #[doc = "0: Disable acknowledgment"]
    _0 = 0,
    #[doc = "1: Enable acknowledgment. During the 9th clock period, the SDAAn line is set to low level."]
    _1 = 1,
}
impl From<Acke> for bool {
    #[inline(always)]
    fn from(variant: Acke) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKE` reader - Acknowledgment control"]
pub type AckeR = crate::BitReader<Acke>;
impl AckeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acke {
        match self.bits {
            false => Acke::_0,
            true => Acke::_1,
        }
    }
    #[doc = "Disable acknowledgment"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acke::_0
    }
    #[doc = "Enable acknowledgment. During the 9th clock period, the SDAAn line is set to low level."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acke::_1
    }
}
#[doc = "Field `ACKE` writer - Acknowledgment control"]
pub type AckeW<'a, REG> = crate::BitWriter<'a, REG, Acke>;
impl<'a, REG> AckeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable acknowledgment"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acke::_0)
    }
    #[doc = "Enable acknowledgment. During the 9th clock period, the SDAAn line is set to low level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acke::_1)
    }
}
#[doc = "Control of clock stretching and interrupt request generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wtim {
    #[doc = "0: An interrupt request is generated on the falling edge of the 8th clock cycle. Master mode: After the output of eight clock pulses, the clock output is set to the low level and clock stretching is set. Slave mode: After the input of eight clock pulses, the clock is set to the low level and clock stretching is set for the master device."]
    _0 = 0,
    #[doc = "1: An interrupt request is generated on the falling edge of the 9th clock cycle. Master mode: After the output of nine clock pulses, the clock output is set to the low level and clock stretching is set. Slave mode: After the input of 9 clock pulses, the clock is set to the low level and clock stretching is set for the master device."]
    _1 = 1,
}
impl From<Wtim> for bool {
    #[inline(always)]
    fn from(variant: Wtim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTIM` reader - Control of clock stretching and interrupt request generation"]
pub type WtimR = crate::BitReader<Wtim>;
impl WtimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wtim {
        match self.bits {
            false => Wtim::_0,
            true => Wtim::_1,
        }
    }
    #[doc = "An interrupt request is generated on the falling edge of the 8th clock cycle. Master mode: After the output of eight clock pulses, the clock output is set to the low level and clock stretching is set. Slave mode: After the input of eight clock pulses, the clock is set to the low level and clock stretching is set for the master device."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wtim::_0
    }
    #[doc = "An interrupt request is generated on the falling edge of the 9th clock cycle. Master mode: After the output of nine clock pulses, the clock output is set to the low level and clock stretching is set. Slave mode: After the input of 9 clock pulses, the clock is set to the low level and clock stretching is set for the master device."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wtim::_1
    }
}
#[doc = "Field `WTIM` writer - Control of clock stretching and interrupt request generation"]
pub type WtimW<'a, REG> = crate::BitWriter<'a, REG, Wtim>;
impl<'a, REG> WtimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An interrupt request is generated on the falling edge of the 8th clock cycle. Master mode: After the output of eight clock pulses, the clock output is set to the low level and clock stretching is set. Slave mode: After the input of eight clock pulses, the clock is set to the low level and clock stretching is set for the master device."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wtim::_0)
    }
    #[doc = "An interrupt request is generated on the falling edge of the 9th clock cycle. Master mode: After the output of nine clock pulses, the clock output is set to the low level and clock stretching is set. Slave mode: After the input of 9 clock pulses, the clock is set to the low level and clock stretching is set for the master device."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wtim::_1)
    }
}
#[doc = "Enable and disable generation of interrupt request when stop condition is detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spie {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<Spie> for bool {
    #[inline(always)]
    fn from(variant: Spie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIE` reader - Enable and disable generation of interrupt request when stop condition is detected"]
pub type SpieR = crate::BitReader<Spie>;
impl SpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spie {
        match self.bits {
            false => Spie::_0,
            true => Spie::_1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spie::_0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spie::_1
    }
}
#[doc = "Field `SPIE` writer - Enable and disable generation of interrupt request when stop condition is detected"]
pub type SpieW<'a, REG> = crate::BitWriter<'a, REG, Spie>;
impl<'a, REG> SpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::_1)
    }
}
#[doc = "Release from the clock stretch state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrel {
    #[doc = "0: The interface is not released from the clock stretch state."]
    _0 = 0,
    #[doc = "1: The interface is released from the clock stretch state. After release from the clock stretch state, this bit is automatically cleared to 0."]
    _1 = 1,
}
impl From<Wrel> for bool {
    #[inline(always)]
    fn from(variant: Wrel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WREL` reader - Release from the clock stretch state"]
pub type WrelR = crate::BitReader<Wrel>;
impl WrelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrel {
        match self.bits {
            false => Wrel::_0,
            true => Wrel::_1,
        }
    }
    #[doc = "The interface is not released from the clock stretch state."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wrel::_0
    }
    #[doc = "The interface is released from the clock stretch state. After release from the clock stretch state, this bit is automatically cleared to 0."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wrel::_1
    }
}
#[doc = "Field `WREL` writer - Release from the clock stretch state"]
pub type WrelW<'a, REG> = crate::BitWriter<'a, REG, Wrel>;
impl<'a, REG> WrelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interface is not released from the clock stretch state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wrel::_0)
    }
    #[doc = "The interface is released from the clock stretch state. After release from the clock stretch state, this bit is automatically cleared to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrel::_1)
    }
}
#[doc = "Exit from communications\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrel {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: IICA exits from the current communications and sets standby mode. This setting is automatically cleared to 0 after being executed. Its uses include cases in which a locally irrelevant extension code has been received. The SCLAn and SDAAn lines are set to high impedance. The following flags of IICA control register n0 (IICCTLn0) and the IICA status register n (IICSn) are cleared to 0.IICCTLn0.STT, SPTIICSn.MSTS, EXC, COI, TRC, ACKD, STD"]
    _1 = 1,
}
impl From<Lrel> for bool {
    #[inline(always)]
    fn from(variant: Lrel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LREL` reader - Exit from communications"]
pub type LrelR = crate::BitReader<Lrel>;
impl LrelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrel {
        match self.bits {
            false => Lrel::_0,
            true => Lrel::_1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lrel::_0
    }
    #[doc = "IICA exits from the current communications and sets standby mode. This setting is automatically cleared to 0 after being executed. Its uses include cases in which a locally irrelevant extension code has been received. The SCLAn and SDAAn lines are set to high impedance. The following flags of IICA control register n0 (IICCTLn0) and the IICA status register n (IICSn) are cleared to 0.IICCTLn0.STT, SPTIICSn.MSTS, EXC, COI, TRC, ACKD, STD"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lrel::_1
    }
}
#[doc = "Field `LREL` writer - Exit from communications"]
pub type LrelW<'a, REG> = crate::BitWriter<'a, REG, Lrel>;
impl<'a, REG> LrelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lrel::_0)
    }
    #[doc = "IICA exits from the current communications and sets standby mode. This setting is automatically cleared to 0 after being executed. Its uses include cases in which a locally irrelevant extension code has been received. The SCLAn and SDAAn lines are set to high impedance. The following flags of IICA control register n0 (IICCTLn0) and the IICA status register n (IICSn) are cleared to 0.IICCTLn0.STT, SPTIICSn.MSTS, EXC, COI, TRC, ACKD, STD"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrel::_1)
    }
}
#[doc = "I2C operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iice {
    #[doc = "0: Stop operation. Reset the IICA status register n (IICSn). Stop internal operation."]
    _0 = 0,
    #[doc = "1: Enable operation."]
    _1 = 1,
}
impl From<Iice> for bool {
    #[inline(always)]
    fn from(variant: Iice) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICE` reader - I2C operation enable"]
pub type IiceR = crate::BitReader<Iice>;
impl IiceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iice {
        match self.bits {
            false => Iice::_0,
            true => Iice::_1,
        }
    }
    #[doc = "Stop operation. Reset the IICA status register n (IICSn). Stop internal operation."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iice::_0
    }
    #[doc = "Enable operation."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iice::_1
    }
}
#[doc = "Field `IICE` writer - I2C operation enable"]
pub type IiceW<'a, REG> = crate::BitWriter<'a, REG, Iice>;
impl<'a, REG> IiceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop operation. Reset the IICA status register n (IICSn). Stop internal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iice::_0)
    }
    #[doc = "Enable operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iice::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Stop condition trigger"]
    #[inline(always)]
    pub fn spt(&self) -> SptR {
        SptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start condition trigger"]
    #[inline(always)]
    pub fn stt(&self) -> SttR {
        SttR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledgment control"]
    #[inline(always)]
    pub fn acke(&self) -> AckeR {
        AckeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control of clock stretching and interrupt request generation"]
    #[inline(always)]
    pub fn wtim(&self) -> WtimR {
        WtimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable and disable generation of interrupt request when stop condition is detected"]
    #[inline(always)]
    pub fn spie(&self) -> SpieR {
        SpieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Release from the clock stretch state"]
    #[inline(always)]
    pub fn wrel(&self) -> WrelR {
        WrelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Exit from communications"]
    #[inline(always)]
    pub fn lrel(&self) -> LrelR {
        LrelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C operation enable"]
    #[inline(always)]
    pub fn iice(&self) -> IiceR {
        IiceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop condition trigger"]
    #[inline(always)]
    pub fn spt(&mut self) -> SptW<Iicctl0Spec> {
        SptW::new(self, 0)
    }
    #[doc = "Bit 1 - Start condition trigger"]
    #[inline(always)]
    pub fn stt(&mut self) -> SttW<Iicctl0Spec> {
        SttW::new(self, 1)
    }
    #[doc = "Bit 2 - Acknowledgment control"]
    #[inline(always)]
    pub fn acke(&mut self) -> AckeW<Iicctl0Spec> {
        AckeW::new(self, 2)
    }
    #[doc = "Bit 3 - Control of clock stretching and interrupt request generation"]
    #[inline(always)]
    pub fn wtim(&mut self) -> WtimW<Iicctl0Spec> {
        WtimW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable and disable generation of interrupt request when stop condition is detected"]
    #[inline(always)]
    pub fn spie(&mut self) -> SpieW<Iicctl0Spec> {
        SpieW::new(self, 4)
    }
    #[doc = "Bit 5 - Release from the clock stretch state"]
    #[inline(always)]
    pub fn wrel(&mut self) -> WrelW<Iicctl0Spec> {
        WrelW::new(self, 5)
    }
    #[doc = "Bit 6 - Exit from communications"]
    #[inline(always)]
    pub fn lrel(&mut self) -> LrelW<Iicctl0Spec> {
        LrelW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C operation enable"]
    #[inline(always)]
    pub fn iice(&mut self) -> IiceW<Iicctl0Spec> {
        IiceW::new(self, 7)
    }
}
#[doc = "IICA Control Register n0\n\nYou can [`read`](crate::Reg::read) this register and get [`iicctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iicctl0Spec;
impl crate::RegisterSpec for Iicctl0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iicctl0::R`](R) reader structure"]
impl crate::Readable for Iicctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`iicctl0::W`](W) writer structure"]
impl crate::Writable for Iicctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IICCTL%s0 to value 0"]
impl crate::Resettable for Iicctl0Spec {
    const RESET_VALUE: u8 = 0;
}
