#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sec: Sec,
    min: Min,
    hour: Hour,
    week: Week,
    day: Day,
    month: Month,
    year: Year,
    subcud: Subcud,
    alarmwm: Alarmwm,
    alarmwh: Alarmwh,
    alarmww: Alarmww,
    rtcc0: Rtcc0,
    rtcc1: Rtcc1,
}
impl RegisterBlock {
    #[doc = "0x00 - Second Count Register"]
    #[inline(always)]
    pub const fn sec(&self) -> &Sec {
        &self.sec
    }
    #[doc = "0x01 - Minute Count Register"]
    #[inline(always)]
    pub const fn min(&self) -> &Min {
        &self.min
    }
    #[doc = "0x02 - Hour Count Register"]
    #[inline(always)]
    pub const fn hour(&self) -> &Hour {
        &self.hour
    }
    #[doc = "0x03 - Day-of-Week Count Register"]
    #[inline(always)]
    pub const fn week(&self) -> &Week {
        &self.week
    }
    #[doc = "0x04 - Day Count Register"]
    #[inline(always)]
    pub const fn day(&self) -> &Day {
        &self.day
    }
    #[doc = "0x05 - Month Count Register"]
    #[inline(always)]
    pub const fn month(&self) -> &Month {
        &self.month
    }
    #[doc = "0x06 - Year Count Register"]
    #[inline(always)]
    pub const fn year(&self) -> &Year {
        &self.year
    }
    #[doc = "0x07 - Time Error Correction Register"]
    #[inline(always)]
    pub const fn subcud(&self) -> &Subcud {
        &self.subcud
    }
    #[doc = "0x08 - Alarm Minute Register"]
    #[inline(always)]
    pub const fn alarmwm(&self) -> &Alarmwm {
        &self.alarmwm
    }
    #[doc = "0x09 - Alarm Hour Register"]
    #[inline(always)]
    pub const fn alarmwh(&self) -> &Alarmwh {
        &self.alarmwh
    }
    #[doc = "0x0a - Alarm Day-of-Week Register"]
    #[inline(always)]
    pub const fn alarmww(&self) -> &Alarmww {
        &self.alarmww
    }
    #[doc = "0x0b - Realtime Clock Control Register 0"]
    #[inline(always)]
    pub const fn rtcc0(&self) -> &Rtcc0 {
        &self.rtcc0
    }
    #[doc = "0x0c - Realtime Clock Control Register 1"]
    #[inline(always)]
    pub const fn rtcc1(&self) -> &Rtcc1 {
        &self.rtcc1
    }
}
#[doc = "SEC (rw) register accessor: Second Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`]
module"]
#[doc(alias = "SEC")]
pub type Sec = crate::Reg<sec::SecSpec>;
#[doc = "Second Count Register"]
pub mod sec;
#[doc = "MIN (rw) register accessor: Minute Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@min`]
module"]
#[doc(alias = "MIN")]
pub type Min = crate::Reg<min::MinSpec>;
#[doc = "Minute Count Register"]
pub mod min;
#[doc = "HOUR (rw) register accessor: Hour Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hour`]
module"]
#[doc(alias = "HOUR")]
pub type Hour = crate::Reg<hour::HourSpec>;
#[doc = "Hour Count Register"]
pub mod hour;
#[doc = "WEEK (rw) register accessor: Day-of-Week Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`week::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`week::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@week`]
module"]
#[doc(alias = "WEEK")]
pub type Week = crate::Reg<week::WeekSpec>;
#[doc = "Day-of-Week Count Register"]
pub mod week;
#[doc = "DAY (rw) register accessor: Day Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`day::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`day::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@day`]
module"]
#[doc(alias = "DAY")]
pub type Day = crate::Reg<day::DaySpec>;
#[doc = "Day Count Register"]
pub mod day;
#[doc = "MONTH (rw) register accessor: Month Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`month::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`month::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@month`]
module"]
#[doc(alias = "MONTH")]
pub type Month = crate::Reg<month::MonthSpec>;
#[doc = "Month Count Register"]
pub mod month;
#[doc = "YEAR (rw) register accessor: Year Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`year::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`year::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@year`]
module"]
#[doc(alias = "YEAR")]
pub type Year = crate::Reg<year::YearSpec>;
#[doc = "Year Count Register"]
pub mod year;
#[doc = "SUBCUD (rw) register accessor: Time Error Correction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`subcud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subcud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subcud`]
module"]
#[doc(alias = "SUBCUD")]
pub type Subcud = crate::Reg<subcud::SubcudSpec>;
#[doc = "Time Error Correction Register"]
pub mod subcud;
#[doc = "ALARMWM (rw) register accessor: Alarm Minute Register\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmwm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmwm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmwm`]
module"]
#[doc(alias = "ALARMWM")]
pub type Alarmwm = crate::Reg<alarmwm::AlarmwmSpec>;
#[doc = "Alarm Minute Register"]
pub mod alarmwm;
#[doc = "ALARMWH (rw) register accessor: Alarm Hour Register\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmwh`]
module"]
#[doc(alias = "ALARMWH")]
pub type Alarmwh = crate::Reg<alarmwh::AlarmwhSpec>;
#[doc = "Alarm Hour Register"]
pub mod alarmwh;
#[doc = "ALARMWW (rw) register accessor: Alarm Day-of-Week Register\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmww::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmww::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmww`]
module"]
#[doc(alias = "ALARMWW")]
pub type Alarmww = crate::Reg<alarmww::AlarmwwSpec>;
#[doc = "Alarm Day-of-Week Register"]
pub mod alarmww;
#[doc = "RTCC0 (rw) register accessor: Realtime Clock Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcc0`]
module"]
#[doc(alias = "RTCC0")]
pub type Rtcc0 = crate::Reg<rtcc0::Rtcc0Spec>;
#[doc = "Realtime Clock Control Register 0"]
pub mod rtcc0;
#[doc = "RTCC1 (rw) register accessor: Realtime Clock Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcc1`]
module"]
#[doc(alias = "RTCC1")]
pub type Rtcc1 = crate::Reg<rtcc1::Rtcc1Spec>;
#[doc = "Realtime Clock Control Register 1"]
pub mod rtcc1;
