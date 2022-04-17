use sxd_xpath::nodeset::Node;
use crate::data_types::FromNode;

pub enum AirTransportMode {
    Unknown,
    Undefined,
    InternationalFlight,
    DomesticFlight,
    IntercontinentalFlight,
    DomesticScheduledFlight,
    ShuttleFlight,
    IntercontinentalCharterFlight,
    InternationalCharterFlight,
    RoundTripCharterFlight,
    SightseeingFlight,
    HelicopterService,
    DomesticCharterFlight,
    SchengenAreaFlight,
    AirshipService,
    ShortHaulInternationalFlight,
    CanalBarge,
}

impl AirTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"undefined" => Some(Self::Undefined),
			"InternationalFlight" => Some(Self::InternationalFlight),
			"domesticFlight" => Some(Self::DomesticFlight),
			"intercontinentalFlight" => Some(Self::IntercontinentalFlight),
			"domesticScheduledFlight" => Some(Self::DomesticScheduledFlight),
			"shuttleFlight" => Some(Self::ShuttleFlight),
			"intercontinentalCharterFlight" => Some(Self::IntercontinentalCharterFlight),
			"internationalCharterFlight" => Some(Self::InternationalCharterFlight),
			"roundTripCharterFlight" => Some(Self::RoundTripCharterFlight),
			"sightseeingFlight" => Some(Self::SightseeingFlight),
			"helicopterService" => Some(Self::HelicopterService),
			"domesticCharterFlight" => Some(Self::DomesticCharterFlight),
			"SchengenAreaFlight" => Some(Self::SchengenAreaFlight),
			"AirshipService" => Some(Self::AirshipService),
			"ShortHaulInternationalFlight" => Some(Self::ShortHaulInternationalFlight),
			"canalBarge" => Some(Self::CanalBarge),
            _ => None
        }
    }
}

pub enum BusTransportMode {
    Unknown,
    Undefined,
    LocalBus,
    RegionalBus,
    ExpressBus,
    NightBus,
    PostBus,
    SpecialNeedsBus,
    MobilityBus,
    MobilityBusForRegisteredDisabled,
    SightseeingBus,
    ShuttleBus,
    SchoolBus,
    SchoolAndPublicServiceBus,
    RailReplacementBus,
    DemandAndResponseBus,
    AirportLinkBus,
}

impl BusTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"undefined" => Some(Self::Undefined),
			"localBus" => Some(Self::LocalBus),
			"regionalBus" => Some(Self::RegionalBus),
			"expressBus" => Some(Self::ExpressBus),
			"nightBus" => Some(Self::NightBus),
			"postBus" => Some(Self::PostBus),
			"specialNeedsBus" => Some(Self::SpecialNeedsBus),
			"mobilityBus" => Some(Self::MobilityBus),
			"mobilityBusForRegisteredDisabled" => Some(Self::MobilityBusForRegisteredDisabled),
			"sightseeingBus" => Some(Self::SightseeingBus),
			"shuttleBus" => Some(Self::ShuttleBus),
			"schoolBus" => Some(Self::SchoolBus),
			"schoolAndPublicServiceBus" => Some(Self::SchoolAndPublicServiceBus),
			"railReplacementBus" => Some(Self::RailReplacementBus),
			"demandAndResponseBus" => Some(Self::DemandAndResponseBus),
			"airportLinkBus" => Some(Self::AirportLinkBus),
            _ => None
        }
    }
}

pub enum CoachTransportMode {
    Unknown,
    Undefined,
    InternationalCoach,
    NationalCoach,
    ShuttleCoach,
    RegionalCoach,
    SpecialCoach,
    SightseeingCoach,
    TouristCoach,
    CommuterCoach,
}

impl CoachTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"undefined" => Some(Self::Undefined),
			"internationalCoach" => Some(Self::InternationalCoach),
			"nationalCoach" => Some(Self::NationalCoach),
			"shuttleCoach" => Some(Self::ShuttleCoach),
			"regionalCoach" => Some(Self::RegionalCoach),
			"specialCoach" => Some(Self::SpecialCoach),
			"sightseeingCoach" => Some(Self::SightseeingCoach),
			"touristCoach" => Some(Self::TouristCoach),
			"commuterCoach" => Some(Self::CommuterCoach),
            _ => None
        }
    }
}

pub enum FunicularTransportMode {
    Unknown,
    Funicular,
    AllFunicularServices,
    UndefinedFunicular,
}

impl FunicularTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"funicular" => Some(Self::Funicular),
			"allFunicularServices" => Some(Self::AllFunicularServices),
			"undefinedFunicular" => Some(Self::UndefinedFunicular),
            _ => None
        }
    }
}

pub enum MetroTransportMode {
    Unknown,
    Undefined,
    Metro,
    Tube,
    UrbanRailway,
}

impl MetroTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"undefined" => Some(Self::Undefined),
			"metro" => Some(Self::Metro),
			"tube" => Some(Self::Tube),
			"urbanRailway" => Some(Self::UrbanRailway),
            _ => None
        }
    }
}

pub enum RailTransportMode {
    Unknown,
    Undefined,
    Local,
    HighSpeedRail,
    SuburbanRailway,
    RegionalRail,
    InterregionalRail,
    LongDistance,
    International,
    SleeperRailService,
    NightRail,
    CarTransportRailService,
    TouristRailway,
    RailShuttle,
    ReplacementRailService,
    SpecialTrain,
    CrossCountryRail,
    RackAndPinionRailway,
}

impl RailTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"undefined" => Some(Self::Undefined),
			"local" => Some(Self::Local),
			"highSpeedRail" => Some(Self::HighSpeedRail),
			"suburbanRailway" => Some(Self::SuburbanRailway),
			"regionalRail" => Some(Self::RegionalRail),
			"interregionalRail" => Some(Self::InterregionalRail),
			"longDistance" => Some(Self::LongDistance),
			"international" => Some(Self::International),
			"sleeperRailService" => Some(Self::SleeperRailService),
			"nightRail" => Some(Self::NightRail),
			"carTransportRailService" => Some(Self::CarTransportRailService),
			"touristRailway" => Some(Self::TouristRailway),
			"railShuttle" => Some(Self::RailShuttle),
			"replacementRailService" => Some(Self::ReplacementRailService),
			"specialTrain" => Some(Self::SpecialTrain),
			"crossCountryRail" => Some(Self::CrossCountryRail),
			"rackAndPinionRailway" => Some(Self::RackAndPinionRailway),
            _ => None
        }
    }
}

pub enum TelecabinTransportMode {
    Unknown,
    Undefined,
    Telecabin,
    CableCar,
    Lift,
    ChairLift,
    DragLift,
    TelecabinLink,
}

impl TelecabinTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"undefined" => Some(Self::Undefined),
			"telecabin" => Some(Self::Telecabin),
			"cableCar" => Some(Self::CableCar),
			"lift" => Some(Self::Lift),
			"chairLift" => Some(Self::ChairLift),
			"dragLift" => Some(Self::DragLift),
			"telecabinLink" => Some(Self::TelecabinLink),
            _ => None
        }
    }
}

pub enum TramTransportMode {
    Unknown,
    Undefined,
    CityTram,
    LocalTram,
    RegionalTram,
    SightseeingTram,
    ShuttleTram,
}

impl TramTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"undefined" => Some(Self::Undefined),
			"cityTram" => Some(Self::CityTram),
			"localTram" => Some(Self::LocalTram),
			"regionalTram" => Some(Self::RegionalTram),
			"sightseeingTram" => Some(Self::SightseeingTram),
			"shuttleTram" => Some(Self::ShuttleTram),
            _ => None
        }
    }
}

pub enum WaterTransportMode {
    Unknown,
    Undefined,
    InternationalCarFerry,
    NationalCarFerry,
    RegionalCarFerry,
    LocalCarFerry,
    InternationalPassengerFerry,
    NationalPassengerFerry,
    RegionalPassengerFerry,
    LocalPassengerFerry,
    PostBoat,
    TrainFerry,
    RoadFerryLink,
    AirportBoatLink,
    HighSpeedVehicleService,
    HighSpeedPassengerService,
    SightseeingService,
    SchoolBoat,
    CableFerry,
    RiverBus,
    ScheduledFerry,
    ShuttleFerryService,
}

impl WaterTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"undefined" => Some(Self::Undefined),
			"internationalCarFerry" => Some(Self::InternationalCarFerry),
			"nationalCarFerry" => Some(Self::NationalCarFerry),
			"regionalCarFerry" => Some(Self::RegionalCarFerry),
			"localCarFerry" => Some(Self::LocalCarFerry),
			"internationalPassengerFerry" => Some(Self::InternationalPassengerFerry),
			"nationalPassengerFerry" => Some(Self::NationalPassengerFerry),
			"regionalPassengerFerry" => Some(Self::RegionalPassengerFerry),
			"localPassengerFerry" => Some(Self::LocalPassengerFerry),
			"postBoat" => Some(Self::PostBoat),
			"trainFerry" => Some(Self::TrainFerry),
			"roadFerryLink" => Some(Self::RoadFerryLink),
			"airportBoatLink" => Some(Self::AirportBoatLink),
			"highSpeedVehicleService" => Some(Self::HighSpeedVehicleService),
			"highSpeedPassengerService" => Some(Self::HighSpeedPassengerService),
			"sightseeingService" => Some(Self::SightseeingService),
			"schoolBoat" => Some(Self::SchoolBoat),
			"cableFerry" => Some(Self::CableFerry),
			"riverBus" => Some(Self::RiverBus),
			"scheduledFerry" => Some(Self::ScheduledFerry),
			"shuttleFerryService" => Some(Self::ShuttleFerryService),
            _ => None
        }
    }
}

pub enum TaxiTransportMode {
    Unknown,
    Undefined,
    CommunalTaxi,
    WaterTaxi,
    RailTaxi,
    BikeTaxi,
    BlackCab,
    MiniCab,
    AllTaxiServices,
}

impl TaxiTransportMode {
    pub fn from_string(input: &str) -> Option<Self> {
        match input {
			"unknown" => Some(Self::Unknown),
			"undefined" => Some(Self::Undefined),
			"communalTaxi" => Some(Self::CommunalTaxi),
			"waterTaxi" => Some(Self::WaterTaxi),
			"railTaxi" => Some(Self::RailTaxi),
			"bikeTaxi" => Some(Self::BikeTaxi),
			"blackCab" => Some(Self::BlackCab),
			"miniCab" => Some(Self::MiniCab),
			"allTaxiServices" => Some(Self::AllTaxiServices),
            _ => None
        }
    }
}

pub enum TransportMode {
    All,
    Unknown,
    Air(Option<AirTransportMode>),
    Bus(Option<BusTransportMode>),
    TrolleyBus,
    Tram(Option<TramTransportMode>),
    Coach(Option<CoachTransportMode>),
    Rail(Option<RailTransportMode>),
    IntercityRail,
    UrbanRail,
    Metro(Option<MetroTransportMode>),
    Water(Option<WaterTransportMode>),
    Cableway,
    Funicular(Option<FunicularTransportMode>),
    Taxi(Option<TaxiTransportMode>),
}

impl TransportMode {
    pub fn new(primary_name: &str, secondary_name: Option<&str>) -> TransportMode {
        match primary_name {
			"all" => Self::All,
			"unknown" => Self::Unknown,
			"air" => Self::Air(secondary_name.and_then(AirTransportMode::from_string)),
			"bus" => Self::Bus(secondary_name.and_then(BusTransportMode::from_string)),
			"trolleyBus" => Self::TrolleyBus,
			"tram" => Self::Tram(secondary_name.and_then(TramTransportMode::from_string)),
			"coach" => Self::Coach(secondary_name.and_then(CoachTransportMode::from_string)),
			"rail" => Self::Rail(secondary_name.and_then(RailTransportMode::from_string)),
			"intercityRail" => Self::IntercityRail,
			"urbanRail" => Self::UrbanRail,
			"metro" => Self::Metro(secondary_name.and_then(MetroTransportMode::from_string)),
			"water" => Self::Water(secondary_name.and_then(WaterTransportMode::from_string)),
			"cableway" => Self::Cableway,
			"funicular" => Self::Funicular(secondary_name.and_then(FunicularTransportMode::from_string)),
			"taxi" => Self::Taxi(secondary_name.and_then(TaxiTransportMode::from_string)),
            _ => unreachable!("invalid transport mode {}", primary_name)
        }
    }
}

impl FromNode for TransportMode {
    fn from_node(from: Node) -> Option<Self> {
        #[cfg(debug)]
        assert_eq!(from.attribute().map(|a| a.name().local_part()), Some("Mode"));
        let mut primary_name = None;
        let mut secondary_name = None;
        for node in from.children() {
            if let Some(name) = node.attribute().map(|a| a.name().local_part()) {
                if name == "PtMode" {
                    primary_name = Some(node.string_value());
                } else if [
                    "AirSubmode", "BusSubmode", "CoachSubmode", "FunicularSubmode", "MetroSubmode",
                    "RailSubmode", "TelecabinSubmode", "TramSubmode", "WaterSubmode", "TaxiSubmode"
                ].contains(&name) {
                    secondary_name = Some(node.string_value())
                }
            }
        }
        Some(TransportMode::new(primary_name?.as_str(), secondary_name.as_deref()))
    }
}
