//! Responses from the WMATA API
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Lines {
    /// See [`Line`].
    pub lines: Box<[Line]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    /// Two letter abbreviation of the line. See [`Line`].
    pub line_code: String,
    /// Full name of the Line.
    pub display_name: String,
    /// [`Station`] for start of the Line.
    pub start_station_code: String,
    /// [`Station`] for end of the Line.
    pub end_station_code: String,
    /// Intermediate terminal [`Station`]. Ex: Mt. Vernon for Yellow, Silver Spring for Red.
    #[serde(rename = "InternalDestination1")]
    pub first_internal_destination: String,
    /// Intermediate terminal [`Station`]. Ex: Mt. Vernon for Yellow, Silver Spring for Red.
    #[serde(rename = "InternalDestination2")]
    pub second_internal_destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationEntrances {
    /// See [`StationEntrance`].
    pub entrances: Box<[StationEntrance]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationEntrance {
    /// Additional information for the entrance.
    pub description: String,
    /// Warning: Deprecated.
    #[serde(rename = "ID")]
    pub id: String,
    /// Latitude of entrance.
    #[serde(rename = "Lat")]
    pub latitude: f64,
    /// Longitude of entrance.
    #[serde(rename = "Lon")]
    pub longitude: f64,
    /// Name of entrance.
    pub name: String,
    /// [`Station`] of this entrance.
    #[serde(rename = "StationCode1")]
    pub first_station_code: String,
    /// Second [`Station`] of this entrance.
    #[serde(rename = "StationCode2")]
    pub second_station_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrainPositions {
    /// See [`TrainPosition`].
    pub train_positions: Box<[TrainPosition]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrainPosition {
    /// Uniquely identifiable internal train identifier
    pub train_id: String,
    /// Non-unique train identifier, often used by WMATA's Rail Scheduling and Operations Teams, as well as over open radio communication.
    pub train_number: String,
    /// Number of cars. Can be 0.
    pub car_count: i32,
    /// The direction of movement regardless of which track the train is on.
    #[serde(rename = "DirectionNum")]
    pub direction_number: i32,
    /// The circuit identifier the train is currently on.
    pub circuit_id: i32,
    /// Destination [`Station`].
    pub destination_station_code: Option<String>,
    /// [`Line`] for this train.
    pub line_code: Option<String>,
    /// Approximate "dwell time". This is not an exact value, but can be used to determine how long a train has been reported at the same track circuit.
    pub seconds_at_location: i32,
    /// Service Type of a train.
    pub service_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StandardRoutes {
    /// See [`StandardRoute`].
    pub standard_routes: Box<[StandardRoute]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StandardRoute {
    /// [`Line`] for this route.
    pub line_code: String,
    #[serde(rename = "TrackNum")]
    /// Track number. 1 or 2.
    pub track_number: i32,
    /// See [`TrackCircuitWithStation`].
    pub track_circuits: Box<[TrackCircuitWithStation]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrackCircuitWithStation {
    /// Order in which the circuit appears for the given line and track.
    #[serde(rename = "SeqNum")]
    pub sequence_number: i32,
    /// An internal system-wide uniquely identifiable circuit number.
    pub circuit_id: i32,
    /// [`Station`] if this circuit is at a station.
    pub station_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrackCircuits {
    /// See [`TrackCircuit`].
    pub track_circuits: Box<[TrackCircuit]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrackCircuit {
    /// Track number. 1 and 2 denote "main" lines, while 0 and 3 are connectors (between different types of tracks) and pocket tracks, respectively.
    pub track: i32,
    /// An internal system-wide uniquely identifiable circuit number.
    pub circuit_id: i32,
    /// See [`TrackNeighbor`].
    pub neighbors: Box<[TrackNeighbor]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrackNeighbor {
    /// Left or Right neighbor group. Generally speaking, left neighbors are to the west and south, while right neighbors are to the east/north.
    pub neighbor_type: String,
    /// Neighboring circuit ids.
    pub circuit_ids: Box<[i32]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ElevatorAndEscalatorIncidents {
    /// See [`ElevatorAndEscalatorIncident`].
    #[serde(rename = "ElevatorIncidents")]
    pub incidents: Box<[ElevatorAndEscalatorIncident]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ElevatorAndEscalatorIncident {
    /// Unique identifier for unit, by type (a single elevator and escalator may have the same UnitName, but no two elevators or two escalators will have the same UnitName).
    pub unit_name: String,
    /// Type of unit. Will be ELEVATOR or ESCALATOR.
    pub unit_type: String,
    /// Warning: Deprecated. If listed here, the unit is inoperational or otherwise impaired.
    pub unit_status: Option<String>,
    /// [`Station`] of the incident.
    pub station_code: String,
    /// Full station name, may include entrance information (e.g.: Metro Center, G and 11th St Entrance).
    pub station_name: String,
    /// Free-text description of the unit location within a station (e.g.: Escalator between mezzanine and platform).
    pub location_description: String,
    /// Warning: Deprecated.
    pub symptom_code: Option<String>,
    /// Warning: Deprecated. Use the time portion of the DateOutOfServ element.
    pub time_out_of_service: String,
    /// Description for why the unit is out of service or otherwise in reduced operation.
    pub symptom_description: String,
    /// Warning: Deprecated.
    pub display_order: f64,
    /// Date and time (Eastern Standard Time) unit was reported out of service. Will be in YYYY-MM-DDTHH:mm:ss format (e.g.: 2014-10-27T15:17:00).
    #[serde(rename = "DateOutOfServ")]
    pub date_out_of_service: String,
    /// Date and time (Eastern Standard Time) outage details was last updated. Will be in YYYY-MM-DDTHH:mm:ss format (e.g.: 2014-10-27T15:17:00).
    pub date_updated: String,
    /// Estimated date and time (Eastern Standard Time) by when unit is expected to return to normal service. May be NULL, otherwise will be in YYYY-MM-DDTHH:mm:ss format (e.g.: 2014-10-27T23:59:59).
    pub estimated_return_to_service: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RailIncidents {
    /// See [`RailIncident`]
    pub incidents: Box<[RailIncident]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RailIncident {
    /// Unique identifier for an incident.
    #[serde(rename = "IncidentID")]
    pub incident_id: String,
    /// Free-text description of the incident.
    pub description: String,
    /// Warning: Deprecated.
    pub start_location_full_name: Option<String>,
    /// Warning: Deprecated.
    pub end_location_full_name: Option<String>,
    /// Warning: Deprecated.
    pub passenger_delay: f64,
    /// Warning: Deprecated.
    pub delay_severity: Option<String>,
    /// Free-text description of the incident type. Usually Delay or Alert but is subject to change at any time.
    pub incident_type: String,
    /// Warning: Deprecated.
    pub emergency_text: Option<String>,
    /// Semi-colon and space separated list of line codes (e.g.: RD; or BL; OR; or BL; OR; RD;). =(
    pub lines_affected: String,
    /// Date and time (Eastern Standard Time) of last update. Will be in YYYY-MM-DDTHH:mm:SS format (e.g.: 2010-07-29T14:21:28).
    pub date_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationToStationInfos {
    /// See [`StationToStationInfo`]
    pub station_to_station_infos: Box<[StationToStationInfo]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationToStationInfo {
    /// Average of distance traveled between two stations and straight-line distance (as used for WMATA fare calculations).
    pub composite_miles: f64,
    /// Destination [`Station`].
    pub destination_station: String,
    /// Structure containing fare information.
    pub rail_fare: RailFare,
    /// Estimated travel time (schedule time) in minutes between the source and destination station. This is not correlated to minutes (Min) in Real-Time Rail Predictions.
    pub rail_time: i32,
    /// Origin [`Station`].
    pub source_station: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RailFare {
    /// Fare during off-peak times.
    pub off_peak_time: f64,
    /// Fare during peak times (weekdays from opening to 9:30 AM and 3-7 PM, and weekends from midnight to closing).
    pub peak_time: f64,
    /// Reduced fare for senior citizens or people with disabilities.
    pub senior_disabled: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RailPredictions {
    /// See [`RailPrediction`].
    pub trains: Box<[RailPrediction]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RailPrediction {
    /// Number of cars on a train, usually 6 or 8, but might also return -.
    pub car: Option<String>,
    /// Abbreviated version of the final destination for a train. This is similar to what is displayed on the signs at stations.
    pub destination: String,
    /// [`Station`] of destination station.
    pub destination_code: Option<String>,
    /// When DestinationCode is populated, this is the full name of the destination station, as shown on the WMATA website.
    pub destination_name: String,
    /// Denotes the track this train is on, but does not necessarily equate to Track 1 or Track 2. With the exception of terminal stations, predictions at the same station with different Group values refer to trains on different tracks.
    pub group: String,
    /// Two-letter abbreviation for the line (e.g.: RD, BL, YL, OR, GR, or SV). May also be blank or No for trains with no passengers.
    pub line: String,
    /// [`Station`] for where the train is arriving.
    pub location_code: String,
    /// Full name of the station where the train is arriving.
    pub location_name: String,
    /// Minutes until arrival. Can be a numeric value, ARR (arriving), BRD (boarding), ---, or empty.
    #[serde(rename = "Min")]
    pub minutes: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationInformation {
    /// Structure describing address information.
    pub address: StationAddress,
    /// [`Station`] for this station.
    pub code: String,
    /// Latitude of this station.
    #[serde(rename = "Lat")]
    pub latitude: f64,
    /// Longitude of this station.
    #[serde(rename = "Lon")]
    pub longitude: f64,
    /// First [`Line`] for this station.
    #[serde(rename = "LineCode1")]
    pub first_line_code: String,
    /// Second [`Line`] for this station.
    #[serde(rename = "LineCode2")]
    pub second_line_code: Option<String>,
    /// Third [`Line`] for this station.
    #[serde(rename = "LineCode3")]
    pub third_line_code: Option<String>,
    /// Fourth [`Line`] for this station.
    #[serde(rename = "LineCode4")]
    pub fourth_line_code: Option<String>,
    /// Station name.
    pub name: String,
    /// For stations with multiple platforms (e.g.: Gallery Place, Fort Totten, L'Enfant Plaza, and Metro Center), the additional [`Station`] will be listed here.
    #[serde(rename = "StationTogether1")]
    pub first_station_together: String,
    /// Similar in function to first_station_together. Currently not in use.
    #[serde(rename = "StationTogether2")]
    pub second_station_together: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationAddress {
    /// City of this station.
    pub city: String,
    /// State of this station.
    pub state: String,
    /// Street address (for GPS use) of this station.
    pub street: String,
    /// Zip code of this station.
    pub zip: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationsParking {
    /// See [`StationParking`].
    pub stations_parking: Box<[StationParking]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationParking {
    /// [`Station`] of this station.
    pub code: String,
    /// When not None, provides additional parking resources such as nearby lots.
    pub notes: Option<String>,
    /// See [`AllDayParking`].
    pub all_day_parking: AllDayParking,
    /// See [`ShortTermParking`].
    pub short_term_parking: ShortTermParking,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AllDayParking {
    /// Number of all-day parking spots available at a station.
    pub total_count: i32,
    /// All-day cost per day (weekday) for Metro riders.
    pub rider_cost: Option<f64>,
    /// All-day cost per day (weekday) for non-Metro riders.
    pub non_rider_cost: Option<f64>,
    /// Similar to RiderCost, except denoting Saturday prices.
    pub saturday_rider_cost: Option<f64>,
    /// Similar to NonRiderCost, except denoting Saturday prices.
    pub saturday_non_rider_cost: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ShortTermParking {
    /// Number of short-term parking spots available at a station (parking meters).
    pub total_count: i32,
    /// Misc. information relating to short-term parking.
    pub notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PathBetweenStations {
    /// See [`Path`].
    pub path: Box<[Path]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Path {
    /// Distance in feet to the previous station in the list.
    #[serde(rename = "DistanceToPrev")]
    pub distance_to_previous_station: i32,
    /// [`Line`] of this station.
    pub line_code: String,
    /// Ordered sequence number.
    #[serde(rename = "SeqNum")]
    pub sequence_number: i32,
    /// [`Station`] of this station.
    pub station_code: String,
    /// Full name for this station, as shown on the WMATA website.
    pub station_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationTimings {
    /// See [`StationTime`]/
    pub station_times: Box<[StationTime]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationTime {
    /// [`Station`] of this station.
    pub code: String,
    /// Full name of the station.
    pub station_name: String,
    // You're gonna love this
    /// Container elements containing timing information based on day of the week. See [`StationFirstLastTrains`].
    pub monday: StationFirstLastTrains,
    /// Container elements containing timing information based on day of the week. See [`StationFirstLastTrains`].
    pub tuesday: StationFirstLastTrains,
    /// Container elements containing timing information based on day of the week. See [`StationFirstLastTrains`].
    pub wednesday: StationFirstLastTrains,
    /// Container elements containing timing information based on day of the week. See [`StationFirstLastTrains`].
    pub thursday: StationFirstLastTrains,
    /// Container elements containing timing information based on day of the week. See [`StationFirstLastTrains`].
    pub friday: StationFirstLastTrains,
    /// Container elements containing timing information based on day of the week. See [`StationFirstLastTrains`].
    pub saturday: StationFirstLastTrains,
    /// Container elements containing timing information based on day of the week. See [`StationFirstLastTrains`].
    pub sunday: StationFirstLastTrains,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationFirstLastTrains {
    /// Station opening time. Format is HH:mm.
    pub opening_time: String,
    /// See [`TrainTime`].
    pub first_trains: Box<[TrainTime]>,
    /// See [`TrainTime`].
    pub last_trains: Box<[TrainTime]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrainTime {
    /// Time the train leaves the station.
    pub time: String,
    /// [`Station`] for the destination station.
    pub destination_station: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Stations {
    /// See [`Station`].
    pub stations: Box<[Station]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Station {
    /// See [`Address`](./struct.Address.html).
    pub address: Address,
    /// [`Station`] of this station.
    pub code: String,
    /// Latitude of this station.
    #[serde(rename = "Lat")]
    pub latitude: f64,
    /// Longitude of this station.
    #[serde(rename = "Lon")]
    pub longitude: f64,
    /// First [`Station`] of this station.
    #[serde(rename = "LineCode1")]
    pub first_line_code: String,
    /// Second [`Station`] of this station.
    #[serde(rename = "LineCode2")]
    pub second_line_code: Option<String>,
    /// Third [`Station`] of this station.
    #[serde(rename = "LineCode3")]
    pub third_line_code: Option<String>,
    /// Fourth [`Station`] of this station.
    #[serde(rename = "LineCode4")]
    pub fourth_line_code: Option<String>,
    /// Station name.
    pub name: String,
    /// For stations with multiple platforms (e.g.: Gallery Place, Fort Totten, L'Enfant Plaza, and Metro Center), the additional [`Station`] will be listed here.
    #[serde(rename = "StationTogether1")]
    pub first_station_together: String,
    /// imilar in function to first_station_together. Currently not in use.
    #[serde(rename = "StationTogether2")]
    pub second_station_together: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    /// City of this station.
    pub city: String,
    /// State of this station.
    pub state: String,
    /// Street address (for GPS use) of this station.
    pub street: String,
    /// Zip code of this station.
    pub zip: String,
}
