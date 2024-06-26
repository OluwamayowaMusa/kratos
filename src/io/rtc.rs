use thiserror_no_std::Error;
use crate::io::port_manager::{Port, PortManager};

const NMI_ENABLE: bool = true;

#[derive(Debug)]
pub struct DateTime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub weekday: u8,
    pub day_of_moth: u8,
    pub month: u8,
    pub year: u8,
    pub century: u8,
}

enum TimeRegister {
    Seconds = 0x00, // 0-59 
    Minutes = 0x02, // 0-59
    Hours = 0x04, // 0-23 in 24H, 1-12 in 12H highest bit set if pm
    Weekday = 0x06, // 1-7, Sunday = 1
    DayOfMonth = 0x07, // 1-31
    Month = 0x08, // 1-12
    Year = 0x09, // 0-99
    Century = 0x32, // 19-20
    StatusRegisterA = 0x0A,
    StatusRegisterB = 0x0B,
}

enum UpdateStatus {
    Clear = 0,
    Set = 1,
}

impl UpdateStatus {
    fn new(progress_bit: u8) -> UpdateStatus {
        match progress_bit {
            0 => UpdateStatus::Clear,
            _ => UpdateStatus::Set,
        }
    }
}

#[derive(Debug, Error)]
pub enum RtcInitError {
    #[error("Failed to get cmos control port")]
    FailedToGetCmosControlPort,
    #[error("Failed to get cmos data port")]
    FailedToGetCmosDataPort
}

pub struct Rtc {
    cmos_control_port: Port,
    cmos_data_port: Port,
}

impl Rtc {
    pub fn new(port_manager: &mut PortManager) -> Result<Rtc, RtcInitError> {
        use RtcInitError::*;

        let cmos_control_port = port_manager.request_port(0x70).ok_or(FailedToGetCmosControlPort)?;
        let cmos_data_port = port_manager.request_port(0x71).ok_or(FailedToGetCmosDataPort)?;

        set_data_format(&cmos_control_port, &cmos_data_port, NMI_ENABLE, TimeRegister::StatusRegisterB as u8);

        Ok(Rtc {
            cmos_control_port,
            cmos_data_port
        })
    }

    pub fn read(&self) -> DateTime {
        use TimeRegister::*;
        update_guarded_op(&self.cmos_control_port, &self.cmos_data_port, StatusRegisterA as u8, |control_port, data_port| {

            let seconds = read_cmos_reg(control_port, data_port, NMI_ENABLE, Seconds as u8);
            let minutes = read_cmos_reg(control_port, data_port, NMI_ENABLE, Minutes as u8);
            let hours = read_cmos_reg(control_port, data_port, NMI_ENABLE, Hours as u8);
            let weekday = read_cmos_reg(control_port, data_port, NMI_ENABLE, Weekday as u8);
            let day_of_moth = read_cmos_reg(control_port, data_port, NMI_ENABLE, DayOfMonth as u8);
            let month = read_cmos_reg(control_port, data_port, NMI_ENABLE, Month as u8);
            let year = read_cmos_reg(control_port, data_port, NMI_ENABLE, Year as u8);
            let century = read_cmos_reg(control_port, data_port, NMI_ENABLE, Century as u8);

            DateTime {
                seconds,
                minutes,
                hours,
                weekday,
                day_of_moth,
                month,
                year,
                century,
            }
        })
    }

    pub fn write(&self, date_time: &DateTime) {
        use TimeRegister::*;
        update_guarded_op(&self.cmos_control_port, &self.cmos_data_port, StatusRegisterA as u8, |control_port, data_port| {
            write_cmos_reg(control_port, data_port, NMI_ENABLE, Seconds as u8, date_time.seconds);
            write_cmos_reg(control_port, data_port, NMI_ENABLE, Minutes as u8, date_time.minutes);
            write_cmos_reg(control_port, data_port, NMI_ENABLE, Hours as u8, date_time.hours);
            write_cmos_reg(control_port, data_port, NMI_ENABLE, Weekday as u8, date_time.weekday);
            write_cmos_reg(control_port, data_port, NMI_ENABLE, DayOfMonth as u8, date_time.day_of_moth);
            write_cmos_reg(control_port, data_port, NMI_ENABLE, Month as u8, date_time.month);
            write_cmos_reg(control_port, data_port, NMI_ENABLE, Year as u8, date_time.year);
            write_cmos_reg(control_port, data_port, NMI_ENABLE, Century as u8, date_time.century);
        })
    }
}

fn update_guarded_op<F, R>(control_port: &Port, data_port: &Port, register: u8, f: F) -> R
where
    F: Fn(&Port, &Port) -> R
{
    let mut date_time;

    loop {
        while update_in_progress(control_port, data_port, NMI_ENABLE, register) {
            continue;
        }


        date_time = f(control_port, data_port);

        // Update may have started while reading, in which case we need to read again.
        // Start loop over
        if update_in_progress(control_port, data_port, NMI_ENABLE, register) {
            continue;
        }

        break;
        
    }

    date_time

}

fn set_data_format(control_port: &Port, data_port: &Port, nmi_enable: bool, register: u8) {
    let mut status_reg = read_cmos_reg(control_port, data_port, nmi_enable, register);
    status_reg |= 1 << 1; // Enables 24H mode
    status_reg |= 1 << 2; // Enables Binary mode

    write_cmos_reg(control_port, data_port, nmi_enable, register, status_reg)
}

fn update_in_progress(control_port: &Port, data_port: &Port, nmi_enable: bool, register: u8) -> bool {
    select_reg(control_port, nmi_enable, register);
    let progress_status = UpdateStatus::new(data_port.readb() >> 7);

    match progress_status {
        UpdateStatus::Set => true,
        UpdateStatus::Clear => false
    }
}

fn read_cmos_reg(control_port: &Port, data_port: &Port, nmi_enable: bool, register: u8) -> u8 {
    select_reg(control_port, nmi_enable, register);
    data_port.readb()
}

pub fn write_cmos_reg(control_port: &Port, data_port: &Port, nmi_enable: bool, register: u8, val: u8) {
    select_reg(control_port, nmi_enable, register);
    data_port.writeb(val);
}

fn select_reg(control_port: &Port, nmi_enable: bool, register: u8) {
    control_port.writeb(get_nmi_mask(nmi_enable) | register)
}

fn get_nmi_mask(nmi_enable: bool) -> u8 {
    if nmi_enable {
        0
    } else {
        1 << 7
    }
}
