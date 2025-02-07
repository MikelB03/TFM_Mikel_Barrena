#[doc = "Register `ASIMA%s0` reader"]
pub type R = crate::R<Asima0Spec>;
#[doc = "Register `ASIMA%s0` writer"]
pub type W = crate::W<Asima0Spec>;
#[doc = "Receive interrupt mode select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isrma {
    #[doc = "0: The UARTA_RX_ERIn interrupt is generated when a reception error occurs (UARTA_RX_ENDIn is not generated)"]
    _0 = 0,
    #[doc = "1: The UARTA_RX_ENDIn interrupt is generated when a reception error occurs (UARTA_RX_ERIn is not generated)"]
    _1 = 1,
}
impl From<Isrma> for bool {
    #[inline(always)]
    fn from(variant: Isrma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISRMA` reader - Receive interrupt mode select"]
pub type IsrmaR = crate::BitReader<Isrma>;
impl IsrmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isrma {
        match self.bits {
            false => Isrma::_0,
            true => Isrma::_1,
        }
    }
    #[doc = "The UARTA_RX_ERIn interrupt is generated when a reception error occurs (UARTA_RX_ENDIn is not generated)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isrma::_0
    }
    #[doc = "The UARTA_RX_ENDIn interrupt is generated when a reception error occurs (UARTA_RX_ERIn is not generated)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isrma::_1
    }
}
#[doc = "Field `ISRMA` writer - Receive interrupt mode select"]
pub type IsrmaW<'a, REG> = crate::BitWriter<'a, REG, Isrma>;
impl<'a, REG> IsrmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The UARTA_RX_ERIn interrupt is generated when a reception error occurs (UARTA_RX_ENDIn is not generated)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isrma::_0)
    }
    #[doc = "The UARTA_RX_ENDIn interrupt is generated when a reception error occurs (UARTA_RX_ERIn is not generated)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isrma::_1)
    }
}
#[doc = "Transmit interrupt mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Issma {
    #[doc = "0: The UARTA_TX_ENDIn interrupt is generated on completion of transmission"]
    _0 = 0,
    #[doc = "1: The UARTA_TX_ENDIn interrupt is generated when the transmit buffer becomes empty (for continuous transmission)"]
    _1 = 1,
}
impl From<Issma> for bool {
    #[inline(always)]
    fn from(variant: Issma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISSMA` reader - Transmit interrupt mode select"]
pub type IssmaR = crate::BitReader<Issma>;
impl IssmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Issma {
        match self.bits {
            false => Issma::_0,
            true => Issma::_1,
        }
    }
    #[doc = "The UARTA_TX_ENDIn interrupt is generated on completion of transmission"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Issma::_0
    }
    #[doc = "The UARTA_TX_ENDIn interrupt is generated when the transmit buffer becomes empty (for continuous transmission)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Issma::_1
    }
}
#[doc = "Field `ISSMA` writer - Transmit interrupt mode select"]
pub type IssmaW<'a, REG> = crate::BitWriter<'a, REG, Issma>;
impl<'a, REG> IssmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The UARTA_TX_ENDIn interrupt is generated on completion of transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Issma::_0)
    }
    #[doc = "The UARTA_TX_ENDIn interrupt is generated when the transmit buffer becomes empty (for continuous transmission)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Issma::_1)
    }
}
#[doc = "Reception enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxea {
    #[doc = "0: Disables reception (reset the reception circuit)"]
    _0 = 0,
    #[doc = "1: Enables reception"]
    _1 = 1,
}
impl From<Rxea> for bool {
    #[inline(always)]
    fn from(variant: Rxea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEA` reader - Reception enable"]
pub type RxeaR = crate::BitReader<Rxea>;
impl RxeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxea {
        match self.bits {
            false => Rxea::_0,
            true => Rxea::_1,
        }
    }
    #[doc = "Disables reception (reset the reception circuit)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxea::_0
    }
    #[doc = "Enables reception"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxea::_1
    }
}
#[doc = "Field `RXEA` writer - Reception enable"]
pub type RxeaW<'a, REG> = crate::BitWriter<'a, REG, Rxea>;
impl<'a, REG> RxeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables reception (reset the reception circuit)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxea::_0)
    }
    #[doc = "Enables reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxea::_1)
    }
}
#[doc = "Transmission enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txea {
    #[doc = "0: Disables transmission (resets the transmission circuit)"]
    _0 = 0,
    #[doc = "1: Enables transmission"]
    _1 = 1,
}
impl From<Txea> for bool {
    #[inline(always)]
    fn from(variant: Txea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEA` reader - Transmission enable"]
pub type TxeaR = crate::BitReader<Txea>;
impl TxeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txea {
        match self.bits {
            false => Txea::_0,
            true => Txea::_1,
        }
    }
    #[doc = "Disables transmission (resets the transmission circuit)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txea::_0
    }
    #[doc = "Enables transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txea::_1
    }
}
#[doc = "Field `TXEA` writer - Transmission enable"]
pub type TxeaW<'a, REG> = crate::BitWriter<'a, REG, Txea>;
impl<'a, REG> TxeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables transmission (resets the transmission circuit)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txea::_0)
    }
    #[doc = "Enables transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txea::_1)
    }
}
#[doc = "UART operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disables the UART operation clock (resets the internal circuits)"]
    _0 = 0,
    #[doc = "1: Enables the UART operation clock"]
    _1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - UART operation enable"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::_0,
            true => En::_1,
        }
    }
    #[doc = "Disables the UART operation clock (resets the internal circuits)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En::_0
    }
    #[doc = "Enables the UART operation clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En::_1
    }
}
#[doc = "Field `EN` writer - UART operation enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the UART operation clock (resets the internal circuits)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En::_0)
    }
    #[doc = "Enables the UART operation clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive interrupt mode select"]
    #[inline(always)]
    pub fn isrma(&self) -> IsrmaR {
        IsrmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt mode select"]
    #[inline(always)]
    pub fn issma(&self) -> IssmaR {
        IssmaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Reception enable"]
    #[inline(always)]
    pub fn rxea(&self) -> RxeaR {
        RxeaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission enable"]
    #[inline(always)]
    pub fn txea(&self) -> TxeaR {
        TxeaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART operation enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt mode select"]
    #[inline(always)]
    pub fn isrma(&mut self) -> IsrmaW<Asima0Spec> {
        IsrmaW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit interrupt mode select"]
    #[inline(always)]
    pub fn issma(&mut self) -> IssmaW<Asima0Spec> {
        IssmaW::new(self, 1)
    }
    #[doc = "Bit 5 - Reception enable"]
    #[inline(always)]
    pub fn rxea(&mut self) -> RxeaW<Asima0Spec> {
        RxeaW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission enable"]
    #[inline(always)]
    pub fn txea(&mut self) -> TxeaW<Asima0Spec> {
        TxeaW::new(self, 6)
    }
    #[doc = "Bit 7 - UART operation enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Asima0Spec> {
        EnW::new(self, 7)
    }
}
#[doc = "Operation Mode Setting Register n0\n\nYou can [`read`](crate::Reg::read) this register and get [`asima0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asima0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Asima0Spec;
impl crate::RegisterSpec for Asima0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`asima0::R`](R) reader structure"]
impl crate::Readable for Asima0Spec {}
#[doc = "`write(|w| ..)` method takes [`asima0::W`](W) writer structure"]
impl crate::Writable for Asima0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ASIMA%s0 to value 0x01"]
impl crate::Resettable for Asima0Spec {
    const RESET_VALUE: u8 = 0x01;
}
