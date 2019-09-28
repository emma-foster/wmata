pub mod responses;
mod tests;

use crate::request_and_deserialize;
use crate::urls::URLs;
use crate::rail::Rail;
use crate::error::Error;
use std::{error, fmt, str::FromStr};

pub struct Station {
    pub api_key: String,
    pub station_code: StationCode,
}

impl Station {
    pub fn new(api_key: &str, station_code: StationCode) -> Self {
        Station {
            api_key: api_key.to_string(),
            station_code,
        }
    }

    pub fn next_trains(&self) -> Result<responses::RailPredictions, Error> {
        request_and_deserialize::<responses::RailPredictions, [(); 0]>(
            &self.api_key,
            &[URLs::NextTrains.to_string(), self.station_code.to_string()].join("/"),
            None,
        )
    }

    pub fn information(&self) -> Result<responses::StationInformation, Error> {
        request_and_deserialize(
            &self.api_key,
            &URLs::Information.to_string(),
            Some(&[("StationCode", self.station_code.to_string())]),
        )
    }

    pub fn parking_information(&self) -> Result<responses::StationsParking, Error> {
        request_and_deserialize(
            &self.api_key,
            &URLs::ParkingInformation.to_string(),
            Some(&[("StationCode", self.station_code.to_string())]),
        )
    }

    pub fn path(&self, to_station: StationCode) -> Result<responses::PathBetweenStations, Error> {
        request_and_deserialize(
            &self.api_key,
            &URLs::Path.to_string(),
            Some(&[
                ("FromStationCode", self.station_code.to_string()),
                ("ToStationCode", to_station.to_string()),
            ]),
        )
    }

    pub fn timings(&self) -> Result<responses::StationTimings, Error> {
        request_and_deserialize(
            &self.api_key,
            &URLs::Timings.to_string(),
            Some(&[("StationCode", self.station_code.to_string())]),
        )
    }

    pub fn to_station(
        &self,
        station: StationCode,
    ) -> Result<responses::StationToStationInfos, Error> {
        self.api_key
            .parse::<Rail>()
            .unwrap()
            .station(Some(self.station_code), Some(station))
    }
}

#[derive(Copy, Clone)]
pub enum StationCode {
    A01,
    A02,
    A03,
    A04,
    A05,
    A06,
    A07,
    A08,
    A09,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,
    B01,
    B02,
    B03,
    B04,
    B05,
    B06,
    B07,
    B08,
    B09,
    B10,
    B11,
    B35,
    C01,
    C02,
    C03,
    C04,
    C05,
    C06,
    C07,
    C08,
    C09,
    C10,
    C12,
    C13,
    C14,
    C15,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    E01,
    E02,
    E03,
    E04,
    E05,
    E06,
    E07,
    E08,
    E09,
    E10,
    F01,
    F02,
    F03,
    F04,
    F05,
    F06,
    F07,
    F08,
    F09,
    F10,
    F11,
    G01,
    G02,
    G03,
    G04,
    G05,
    J02,
    J03,
    K01,
    K02,
    K03,
    K04,
    K05,
    K06,
    K07,
    K08,
    N01,
    N02,
    N03,
    N04,
    N06,
}

impl ToString for StationCode {
    fn to_string(&self) -> String {
        match self {
            StationCode::A01 => "A01".to_string(),
            StationCode::A02 => "A02".to_string(),
            StationCode::A03 => "A03".to_string(),
            StationCode::A04 => "A04".to_string(),
            StationCode::A05 => "A05".to_string(),
            StationCode::A06 => "A06".to_string(),
            StationCode::A07 => "A07".to_string(),
            StationCode::A08 => "A08".to_string(),
            StationCode::A09 => "A09".to_string(),
            StationCode::A10 => "A10".to_string(),
            StationCode::A11 => "A11".to_string(),
            StationCode::A12 => "A12".to_string(),
            StationCode::A13 => "A13".to_string(),
            StationCode::A14 => "A14".to_string(),
            StationCode::A15 => "A15".to_string(),
            StationCode::B01 => "B01".to_string(),
            StationCode::B02 => "B02".to_string(),
            StationCode::B03 => "B03".to_string(),
            StationCode::B04 => "B04".to_string(),
            StationCode::B05 => "B05".to_string(),
            StationCode::B06 => "B06".to_string(),
            StationCode::B07 => "B07".to_string(),
            StationCode::B08 => "B08".to_string(),
            StationCode::B09 => "B09".to_string(),
            StationCode::B10 => "B10".to_string(),
            StationCode::B11 => "B11".to_string(),
            StationCode::B35 => "B35".to_string(),
            StationCode::C01 => "C01".to_string(),
            StationCode::C02 => "C02".to_string(),
            StationCode::C03 => "C03".to_string(),
            StationCode::C04 => "C04".to_string(),
            StationCode::C05 => "C05".to_string(),
            StationCode::C06 => "C06".to_string(),
            StationCode::C07 => "C07".to_string(),
            StationCode::C08 => "C08".to_string(),
            StationCode::C09 => "C09".to_string(),
            StationCode::C10 => "C10".to_string(),
            StationCode::C12 => "C12".to_string(),
            StationCode::C13 => "C13".to_string(),
            StationCode::C14 => "C14".to_string(),
            StationCode::C15 => "C15".to_string(),
            StationCode::D01 => "D01".to_string(),
            StationCode::D02 => "D02".to_string(),
            StationCode::D03 => "D03".to_string(),
            StationCode::D04 => "D04".to_string(),
            StationCode::D05 => "D05".to_string(),
            StationCode::D06 => "D06".to_string(),
            StationCode::D07 => "D07".to_string(),
            StationCode::D08 => "D08".to_string(),
            StationCode::D09 => "D09".to_string(),
            StationCode::D10 => "D10".to_string(),
            StationCode::D11 => "D11".to_string(),
            StationCode::D12 => "D12".to_string(),
            StationCode::D13 => "D13".to_string(),
            StationCode::E01 => "E01".to_string(),
            StationCode::E02 => "E02".to_string(),
            StationCode::E03 => "E03".to_string(),
            StationCode::E04 => "E04".to_string(),
            StationCode::E05 => "E05".to_string(),
            StationCode::E06 => "E06".to_string(),
            StationCode::E07 => "E07".to_string(),
            StationCode::E08 => "E08".to_string(),
            StationCode::E09 => "E09".to_string(),
            StationCode::E10 => "E10".to_string(),
            StationCode::F01 => "F01".to_string(),
            StationCode::F02 => "F02".to_string(),
            StationCode::F03 => "F03".to_string(),
            StationCode::F04 => "F04".to_string(),
            StationCode::F05 => "F05".to_string(),
            StationCode::F06 => "F06".to_string(),
            StationCode::F07 => "F07".to_string(),
            StationCode::F08 => "F08".to_string(),
            StationCode::F09 => "F09".to_string(),
            StationCode::F10 => "F10".to_string(),
            StationCode::F11 => "F11".to_string(),
            StationCode::G01 => "G01".to_string(),
            StationCode::G02 => "G02".to_string(),
            StationCode::G03 => "G03".to_string(),
            StationCode::G04 => "G04".to_string(),
            StationCode::G05 => "G05".to_string(),
            StationCode::J02 => "J02".to_string(),
            StationCode::J03 => "J03".to_string(),
            StationCode::K01 => "K01".to_string(),
            StationCode::K02 => "K02".to_string(),
            StationCode::K03 => "K03".to_string(),
            StationCode::K04 => "K04".to_string(),
            StationCode::K05 => "K05".to_string(),
            StationCode::K06 => "K06".to_string(),
            StationCode::K07 => "K07".to_string(),
            StationCode::K08 => "K08".to_string(),
            StationCode::N01 => "N01".to_string(),
            StationCode::N02 => "N02".to_string(),
            StationCode::N03 => "N03".to_string(),
            StationCode::N04 => "N04".to_string(),
            StationCode::N06 => "N06".to_string(),
        }
    }
}

impl FromStr for StationCode {
    type Err = StringIsNotStationCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A01" => Ok(StationCode::A01),
            "A02" => Ok(StationCode::A02),
            "A03" => Ok(StationCode::A03),
            "A04" => Ok(StationCode::A04),
            "A05" => Ok(StationCode::A05),
            "A06" => Ok(StationCode::A06),
            "A07" => Ok(StationCode::A07),
            "A08" => Ok(StationCode::A08),
            "A09" => Ok(StationCode::A09),
            "A10" => Ok(StationCode::A10),
            "A11" => Ok(StationCode::A11),
            "A12" => Ok(StationCode::A12),
            "A13" => Ok(StationCode::A13),
            "A14" => Ok(StationCode::A14),
            "A15" => Ok(StationCode::A15),
            "B01" => Ok(StationCode::B01),
            "B02" => Ok(StationCode::B02),
            "B03" => Ok(StationCode::B03),
            "B04" => Ok(StationCode::B04),
            "B05" => Ok(StationCode::B05),
            "B06" => Ok(StationCode::B06),
            "B07" => Ok(StationCode::B07),
            "B08" => Ok(StationCode::B08),
            "B09" => Ok(StationCode::B09),
            "B10" => Ok(StationCode::B10),
            "B11" => Ok(StationCode::B11),
            "B35" => Ok(StationCode::B35),
            "C01" => Ok(StationCode::C01),
            "C02" => Ok(StationCode::C02),
            "C03" => Ok(StationCode::C03),
            "C04" => Ok(StationCode::C04),
            "C05" => Ok(StationCode::C05),
            "C06" => Ok(StationCode::C06),
            "C07" => Ok(StationCode::C07),
            "C08" => Ok(StationCode::C08),
            "C09" => Ok(StationCode::C09),
            "C10" => Ok(StationCode::C10),
            "C12" => Ok(StationCode::C12),
            "C13" => Ok(StationCode::C13),
            "C14" => Ok(StationCode::C14),
            "C15" => Ok(StationCode::C15),
            "D01" => Ok(StationCode::D01),
            "D02" => Ok(StationCode::D02),
            "D03" => Ok(StationCode::D03),
            "D04" => Ok(StationCode::D04),
            "D05" => Ok(StationCode::D05),
            "D06" => Ok(StationCode::D06),
            "D07" => Ok(StationCode::D07),
            "D08" => Ok(StationCode::D08),
            "D09" => Ok(StationCode::D09),
            "D10" => Ok(StationCode::D10),
            "D11" => Ok(StationCode::D11),
            "D12" => Ok(StationCode::D12),
            "D13" => Ok(StationCode::D13),
            "E01" => Ok(StationCode::E01),
            "E02" => Ok(StationCode::E02),
            "E03" => Ok(StationCode::E03),
            "E04" => Ok(StationCode::E04),
            "E05" => Ok(StationCode::E05),
            "E06" => Ok(StationCode::E06),
            "E07" => Ok(StationCode::E07),
            "E08" => Ok(StationCode::E08),
            "E09" => Ok(StationCode::E09),
            "E10" => Ok(StationCode::E10),
            "F01" => Ok(StationCode::F01),
            "F02" => Ok(StationCode::F02),
            "F03" => Ok(StationCode::F03),
            "F04" => Ok(StationCode::F04),
            "F05" => Ok(StationCode::F05),
            "F06" => Ok(StationCode::F06),
            "F07" => Ok(StationCode::F07),
            "F08" => Ok(StationCode::F08),
            "F09" => Ok(StationCode::F09),
            "F10" => Ok(StationCode::F10),
            "F11" => Ok(StationCode::F11),
            "G01" => Ok(StationCode::G01),
            "G02" => Ok(StationCode::G02),
            "G03" => Ok(StationCode::G03),
            "G04" => Ok(StationCode::G04),
            "G05" => Ok(StationCode::G05),
            "J02" => Ok(StationCode::J02),
            "J03" => Ok(StationCode::J03),
            "K01" => Ok(StationCode::K01),
            "K02" => Ok(StationCode::K02),
            "K03" => Ok(StationCode::K03),
            "K04" => Ok(StationCode::K04),
            "K05" => Ok(StationCode::K05),
            "K06" => Ok(StationCode::K06),
            "K07" => Ok(StationCode::K07),
            "K08" => Ok(StationCode::K08),
            "N01" => Ok(StationCode::N01),
            "N02" => Ok(StationCode::N02),
            "N03" => Ok(StationCode::N03),
            "N04" => Ok(StationCode::N04),
            "N06" => Ok(StationCode::N06),
            _ => Err(StringIsNotStationCodeError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringIsNotStationCodeError;

impl fmt::Display for StringIsNotStationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Provided string is not a valid station code.")
    }
}

impl error::Error for StringIsNotStationCodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
