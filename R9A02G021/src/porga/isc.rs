#[doc = "Register `ISC` reader"]
pub type R = crate::R<IscSpec>;
#[doc = "Register `ISC` writer"]
pub type W = crate::W<IscSpec>;
#[doc = "Switching channel 7 input of timer array unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isc1 {
    #[doc = "0: Uses the input signal of the TI07 pin as a timer input (normal operation)"]
    _0 = 0,
    #[doc = "1: Input signal of the RxD2 pin is used as timer input (detects the wakeup signal and measures the low width of the break field and the pulse width of the sync field)."]
    _1 = 1,
}
impl From<Isc1> for bool {
    #[inline(always)]
    fn from(variant: Isc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISC1` reader - Switching channel 7 input of timer array unit"]
pub type Isc1R = crate::BitReader<Isc1>;
impl Isc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isc1 {
        match self.bits {
            false => Isc1::_0,
            true => Isc1::_1,
        }
    }
    #[doc = "Uses the input signal of the TI07 pin as a timer input (normal operation)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isc1::_0
    }
    #[doc = "Input signal of the RxD2 pin is used as timer input (detects the wakeup signal and measures the low width of the break field and the pulse width of the sync field)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isc1::_1
    }
}
#[doc = "Field `ISC1` writer - Switching channel 7 input of timer array unit"]
pub type Isc1W<'a, REG> = crate::BitWriter<'a, REG, Isc1>;
impl<'a, REG> Isc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Uses the input signal of the TI07 pin as a timer input (normal operation)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isc1::_0)
    }
    #[doc = "Input signal of the RxD2 pin is used as timer input (detects the wakeup signal and measures the low width of the break field and the pulse width of the sync field)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isc1::_1)
    }
}
#[doc = "Switch of the serial clock input source of SPI00\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Isc43 {
    #[doc = "0: Input signal of the SCK00 pin (normal operation)"]
    _00 = 0,
    #[doc = "2: TO01 output signal"]
    _10 = 2,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Isc43> for u8 {
    #[inline(always)]
    fn from(variant: Isc43) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Isc43 {
    type Ux = u8;
}
impl crate::IsEnum for Isc43 {}
#[doc = "Field `ISC43` reader - Switch of the serial clock input source of SPI00"]
pub type Isc43R = crate::FieldReader<Isc43>;
impl Isc43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isc43 {
        match self.bits {
            0 => Isc43::_00,
            2 => Isc43::_10,
            _ => Isc43::Others,
        }
    }
    #[doc = "Input signal of the SCK00 pin (normal operation)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Isc43::_00
    }
    #[doc = "TO01 output signal"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Isc43::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Isc43::Others)
    }
}
#[doc = "Field `ISC43` writer - Switch of the serial clock input source of SPI00"]
pub type Isc43W<'a, REG> = crate::FieldWriter<'a, REG, 2, Isc43, crate::Safe>;
impl<'a, REG> Isc43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input signal of the SCK00 pin (normal operation)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Isc43::_00)
    }
    #[doc = "TO01 output signal"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Isc43::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Isc43::Others)
    }
}
#[doc = "Switch of the serial clock input source of SPI01\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Isc76 {
    #[doc = "0: Input signal of the SCK01 pin (normal operation)"]
    _00 = 0,
    #[doc = "2: TO01 output signal"]
    _10 = 2,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Isc76> for u8 {
    #[inline(always)]
    fn from(variant: Isc76) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Isc76 {
    type Ux = u8;
}
impl crate::IsEnum for Isc76 {}
#[doc = "Field `ISC76` reader - Switch of the serial clock input source of SPI01"]
pub type Isc76R = crate::FieldReader<Isc76>;
impl Isc76R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isc76 {
        match self.bits {
            0 => Isc76::_00,
            2 => Isc76::_10,
            _ => Isc76::Others,
        }
    }
    #[doc = "Input signal of the SCK01 pin (normal operation)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Isc76::_00
    }
    #[doc = "TO01 output signal"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Isc76::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Isc76::Others)
    }
}
#[doc = "Field `ISC76` writer - Switch of the serial clock input source of SPI01"]
pub type Isc76W<'a, REG> = crate::FieldWriter<'a, REG, 2, Isc76, crate::Safe>;
impl<'a, REG> Isc76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input signal of the SCK01 pin (normal operation)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Isc76::_00)
    }
    #[doc = "TO01 output signal"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Isc76::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Isc76::Others)
    }
}
impl R {
    #[doc = "Bit 1 - Switching channel 7 input of timer array unit"]
    #[inline(always)]
    pub fn isc1(&self) -> Isc1R {
        Isc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Switch of the serial clock input source of SPI00"]
    #[inline(always)]
    pub fn isc43(&self) -> Isc43R {
        Isc43R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bits 6:7 - Switch of the serial clock input source of SPI01"]
    #[inline(always)]
    pub fn isc76(&self) -> Isc76R {
        Isc76R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 1 - Switching channel 7 input of timer array unit"]
    #[inline(always)]
    pub fn isc1(&mut self) -> Isc1W<IscSpec> {
        Isc1W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Switch of the serial clock input source of SPI00"]
    #[inline(always)]
    pub fn isc43(&mut self) -> Isc43W<IscSpec> {
        Isc43W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Switch of the serial clock input source of SPI01"]
    #[inline(always)]
    pub fn isc76(&mut self) -> Isc76W<IscSpec> {
        Isc76W::new(self, 6)
    }
}
#[doc = "Input Switch Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IscSpec;
impl crate::RegisterSpec for IscSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`isc::R`](R) reader structure"]
impl crate::Readable for IscSpec {}
#[doc = "`write(|w| ..)` method takes [`isc::W`](W) writer structure"]
impl crate::Writable for IscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ISC to value 0"]
impl crate::Resettable for IscSpec {
    const RESET_VALUE: u8 = 0;
}
