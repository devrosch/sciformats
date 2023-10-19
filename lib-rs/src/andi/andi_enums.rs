use strum::{Display, EnumString, EnumIter, IntoEnumIterator};

use super::AndiError;

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsExperimentType {
    #[default]
    #[strum(serialize = "Centroided Mass Spectrum")]
    CentroidedMassSpectrum,
    #[strum(serialize = "Continuum Mass Spectrum")]
    ContinuumMassSpectrum,
    #[strum(serialize = "Library Mass Spectrum")]
    LibraryMassSpectrum,
}

// #[derive(Debug, PartialEq)]
// pub enum AndiMsExperimentType {
//     /// default
//     CentroidedMassSpectrum,
//     ContinuumMassSpectrum,
//     LibraryMassSpectrum,
// }

// impl AndiMsExperimentType {
//     const CENTROIDED_MASS_SPECTRUM_STR: &str = "Centroided Mass Spectrum";
//     const CONTINUUM_MASS_SPECTRUM_STR: &str = "Continuum Mass Spectrum";
//     const LIBRARY_MASS_SPECTRUM_STR: &str = "Library Mass Spectrum";
// }

// impl FromStr for AndiMsExperimentType {
//     type Err = AndiError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             Self::CENTROIDED_MASS_SPECTRUM_STR => Ok(Self::CentroidedMassSpectrum),
//             Self::CONTINUUM_MASS_SPECTRUM_STR => Ok(Self::ContinuumMassSpectrum),
//             Self::LIBRARY_MASS_SPECTRUM_STR => Ok(Self::LibraryMassSpectrum),
//             _ => Err(AndiError::new(&format!(
//                 "Illegal MS experiment type: {}",
//                 s
//             ))),
//         }
//     }
// }

// impl Display for AndiMsExperimentType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::CentroidedMassSpectrum => write!(f, "{}", Self::CENTROIDED_MASS_SPECTRUM_STR),
//             Self::ContinuumMassSpectrum => write!(f, "{}", Self::CONTINUUM_MASS_SPECTRUM_STR),
//             Self::LibraryMassSpectrum => write!(f, "{}", Self::LIBRARY_MASS_SPECTRUM_STR),
//         }
//     }
// }

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsSampleState {
    #[strum(serialize = "Solid")]
    Solid,
    #[strum(serialize = "Liquid")]
    Liquid,
    #[strum(serialize = "Gas")]
    Gas,
    #[strum(serialize = "Supercritical Fluid")]
    SupercriticalFluid,
    #[strum(serialize = "Plasma")]
    Plasma,
    #[default]
    #[strum(serialize = "Other State")]
    OtherState,
}

// impl AndiMsSampleState {
//     const SOLID_STR: &str = "Solid";
//     const LIQUID_STR: &str = "Liquid";
//     const GAS_STR: &str = "Gas";
//     const SUPERCRITICAL_FLUID_STR: &str = "Supercritical Fluid";
//     const PLASMA_STR: &str = "Plasma";
//     const OTHER_STATE_STR: &str = "Other State";
// }

// impl FromStr for AndiMsSampleState {
//     type Err = AndiError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             Self::SOLID_STR => Ok(Self::Solid),
//             Self::LIQUID_STR => Ok(Self::Liquid),
//             Self::GAS_STR => Ok(Self::Gas),
//             Self::SUPERCRITICAL_FLUID_STR => Ok(Self::SupercriticalFluid),
//             Self::PLASMA_STR => Ok(Self::Plasma),
//             Self::OTHER_STATE_STR => Ok(Self::OtherState),
//             _ => Err(AndiError::new(&format!("Illegal MS sample state: {}", s))),
//         }
//     }
// }

// impl Display for AndiMsSampleState {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Solid => write!(f, "{}", Self::SOLID_STR),
//             Self::Liquid => write!(f, "{}", Self::LIQUID_STR),
//             Self::Gas => write!(f, "{}", Self::GAS_STR),
//             Self::SupercriticalFluid => write!(f, "{}", Self::SUPERCRITICAL_FLUID_STR),
//             Self::Plasma => write!(f, "{}", Self::PLASMA_STR),
//             Self::OtherState => write!(f, "{}", Self::OTHER_STATE_STR),
//         }
//     }
// }

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsSeparationMethod {
    #[strum(serialize = "Gas-Liquid Chromatography")]
    Glc,
    #[strum(serialize = "Gas-Solid Chromatography")]
    Gsc,
    #[strum(serialize = "Normal Phase Liquid Chromatography")]
    Nplc,
    #[strum(serialize = "Reverse Phase Liquid Chromatography")]
    Rplc,
    #[strum(serialize = "Ion Exchange Liquid Chromatography")]
    Ielc,
    #[strum(serialize = "Size Exclusion Liquid Chromatography")]
    Selc,
    #[strum(serialize = "Ion Pair Liquid Chromatography")]
    Iplc,
    #[strum(serialize = "Other Liquid Chromatography")]
    Olc,
    #[strum(serialize = "Supercritical Fluid Chromatography")]
    Sfc,
    #[strum(serialize = "Thin Layer Chromatography")]
    Tlc,
    #[strum(serialize = "Field Flow Fractionation")]
    Fff,
    #[strum(serialize = "Capillary Zone Electrophoresis")]
    Cze,
    #[strum(serialize = "Other Chromatography")]
    Other,
    #[default]
    #[strum(serialize = "No Chromatography")]
    None,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsMassSpectrometerInlet {
    #[strum(serialize = "Membrane Separator")]
    Membrane,
    #[strum(serialize = "Capillary Direct")]
    Capillary,
    #[strum(serialize = "Open Split")]
    OpenSplit,
    #[strum(serialize = "Jet Separator")]
    Jet,
    #[default]
    #[strum(serialize = "Direct Inlet Probe")]
    Direct,
    #[strum(serialize = "Septum")]
    Septum,
    #[strum(serialize = "Particle Beam")]
    Pb,
    #[strum(serialize = "Reservoir")]
    Reservoir,
    #[strum(serialize = "Moving Belt")]
    Belt,
    #[strum(serialize = "Atmospheric Pressure Chemical Ionization Inlet")]
    Apci,
    #[strum(serialize = "Flow Injection Analysis")]
    Fia,
    #[strum(serialize = "Electrospray Inlet")]
    Es,
    #[strum(serialize = "Infusion")]
    Infusion,
    #[strum(serialize = "Thermospray Inlet")]
    Ts,
    #[strum(serialize = "Other Probe")]
    Probe,
    #[strum(serialize = "Other Inlet")]
    Other,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsIonizationMethod {
    #[default]
    #[strum(serialize = "Electron Impact")]
    Ei,
    #[strum(serialize = "Chemical Ionization")]
    Ci,
    #[strum(serialize = "Fast Atom Bombardment")]
    Fab,
    #[strum(serialize = "Field Desorption")]
    Fd,
    #[strum(serialize = "Field Ionization")]
    Fi,
    #[strum(serialize = "Electrospray Ionization")]
    Es,
    #[strum(serialize = "Thermospray Ionization")]
    Ts,
    #[strum(serialize = "Atmospheric Pressure Chemical Ionization")]
    Apci,
    #[strum(serialize = "Plasma Desorption")]
    Pd,
    #[strum(serialize = "Laser Desorption")]
    Ld,
    #[strum(serialize = "Spark Ionization")]
    Spark,
    #[strum(serialize = "Thermal Ionization")]
    Thermal,
    #[strum(serialize = "Other Ionization")]
    Other,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsIonizationPolarity {
    #[default]
    #[strum(serialize = "Positive Polarity")]
    Plus,
    #[strum(serialize = "Negative Polarity")]
    Minus,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsDetectorType {
    #[default]
    #[strum(serialize = "Electron Multiplier")]
    Em,
    #[strum(serialize = "Photomultplier")]
    Pm,
    #[strum(serialize = "Focal Plane Array")]
    Focal,
    #[strum(serialize = "Faraday Cup")]
    Cup,
    #[strum(serialize = "Conversion Dynode Electron Multiplier")]
    DynodeEm,
    #[strum(serialize = "Conversion Dynode Photomultiplier")]
    DynodePm,
    #[strum(serialize = "Multicollector")]
    Multicoll,
    #[strum(serialize = "Other Detector")]
    Other,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsResolutionType {
    #[default]
    #[strum(serialize = "Constant Resolution")]
    Constant,
    #[strum(serialize = "Proportional Resolution")]
    Proportional,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsScanFunction {
    #[default]
    #[strum(serialize = "Mass Scan")]
    Scan,
    #[strum(serialize = "Selected Ion Detection")]
    Sid,
    #[strum(serialize = "Other Function")]
    Other,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsScanDirection {
    #[default]
    #[strum(serialize = "Up")]
    Up,
    #[strum(serialize = "Down")]
    Down,
    #[strum(serialize = "Other Direction")]
    Other,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsScanLaw {
    #[default]
    #[strum(serialize = "Linear")]
    Linear,
    #[strum(serialize = "Exponential")]
    Exponential,
    #[strum(serialize = "Quadratic")]
    Quadratic,
    #[strum(serialize = "Other Law")]
    Other,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsMassAxisUnit {
    #[default]
    #[strum(serialize = "M/Z")]
    Mz,
    #[strum(serialize = "Arbitrary Mass Units")]
    Arbitrary,
    #[strum(serialize = "Other Mass")]
    Other,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsTimeAxisUnit {
    #[default]
    #[strum(serialize = "Seconds")]
    Seconds,
    #[strum(serialize = "Arbitrary Time Units")]
    Arbitrary,
    #[strum(serialize = "Other Time")]
    Other,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsIntensityAxisUnit {
    #[strum(serialize = "Total counts")]
    Counts,
    #[strum(serialize = "Counts Per Second")]
    Cps,
    #[strum(serialize = "Volts")]
    Volts,
    #[strum(serialize = "Current")]
    Current,
    #[default]
    #[strum(serialize = "Arbitrary Intensity Units")]
    Arbitrary,
    #[strum(serialize = "Other Intensity")]
    Other,
}

#[derive(Debug, PartialEq, Default, EnumString, Display)]
pub enum AndiMsDataFormat {
    #[default]
    #[strum(serialize = "Short")]
    Short,
    #[strum(serialize = "Long")]
    Long,
    #[strum(serialize = "Float")]
    Float,
    #[strum(serialize = "Double")]
    Double,
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum AndiMsFlagValue {
    #[strum(serialize = "Not High Resolution")]
    NotHighResolution,
    #[strum(serialize = "Missed Reference")]
    MissedReference,
    #[strum(serialize = "Unresolved")]
    Unresolved,
    #[strum(serialize = "Doubly Charged")]
    DoublyCharged,
    #[strum(serialize = "Reference")]
    Reference,
    #[strum(serialize = "Exception")]
    Exception,
    #[strum(serialize = "Saturated")]
    Saturated,
    #[strum(serialize = "Significant")]
    Significant,
    #[strum(serialize = "Merged")]
    Merged,
    #[strum(serialize = "Fragemented")]
    Fragemented,
    #[strum(serialize = "Area Height")]
    AreaHeight,
    #[strum(serialize = "Math Modified")]
    MathModified,
    #[strum(serialize = "Negative Intensity")]
    NegativeIntensity,
    #[strum(serialize = "Extended Accuracy")]
    ExtendedAccuracy,
    #[strum(serialize = "Calculated")]
    Calculated,
    #[strum(serialize = "Lock Mass")]
    LockMass,
}

impl AndiMsFlagValue {
    pub fn as_i32(&self) -> i32 {
        match self {
            Self::NotHighResolution => 0x01,
            Self::MissedReference => 0x02,
            Self::Unresolved => 0x04,
            Self::DoublyCharged => 0x08,
            Self::Reference => 0x10,
            Self::Exception => 0x020,
            Self::Saturated => 0x40,
            Self::Significant => 0x80,
            Self::Merged => 0x100,
            Self::Fragemented => 0x200,
            Self::AreaHeight => 0x400,
            Self::MathModified => 0x800,
            Self::NegativeIntensity => 0x1000,
            Self::ExtendedAccuracy => 0x2000,
            Self::Calculated => 0x4000,
            Self::LockMass => 0x8000,
        }
    }

    pub fn set_from_i32(v: &i32) -> Result<Vec<Self>, AndiError> {
        if *v as u32 & 0xFFFF0000 != 0 {
            return Err(AndiError::new(&format!("Illegal flags: {:#0x}", v)));
        }
        let mut res = Vec::<Self>::new();
        for flag in Self::iter() {
            if flag.as_i32() & v != 0 {
                res.push(flag);
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_experiment_type_from_string_conversion() {
        assert_eq!(
            AndiMsExperimentType::CentroidedMassSpectrum,
            AndiMsExperimentType::from_str("Centroided Mass Spectrum").unwrap()
        );
        assert_eq!(
            AndiMsExperimentType::ContinuumMassSpectrum,
            AndiMsExperimentType::from_str("Continuum Mass Spectrum").unwrap()
        );
        assert_eq!(
            AndiMsExperimentType::LibraryMassSpectrum,
            AndiMsExperimentType::from_str("Library Mass Spectrum").unwrap()
        );
        assert!(AndiMsExperimentType::from_str("Illegal string").is_err());
        assert_eq!(
            AndiMsExperimentType::CentroidedMassSpectrum,
            AndiMsExperimentType::default()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_experiment_type_to_string_conversion() {
        assert_eq!(
            "Centroided Mass Spectrum",
            AndiMsExperimentType::CentroidedMassSpectrum.to_string(),
        );
        assert_eq!(
            "Continuum Mass Spectrum",
            AndiMsExperimentType::ContinuumMassSpectrum.to_string(),
        );
        assert_eq!(
            "Library Mass Spectrum",
            AndiMsExperimentType::LibraryMassSpectrum.to_string(),
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_sample_state_from_string_conversion() {
        assert_eq!(
            AndiMsSampleState::Solid,
            AndiMsSampleState::from_str("Solid").unwrap()
        );
        assert_eq!(
            AndiMsSampleState::Liquid,
            AndiMsSampleState::from_str("Liquid").unwrap()
        );
        assert_eq!(
            AndiMsSampleState::Gas,
            AndiMsSampleState::from_str("Gas").unwrap()
        );
        assert_eq!(
            AndiMsSampleState::SupercriticalFluid,
            AndiMsSampleState::from_str("Supercritical Fluid").unwrap()
        );
        assert_eq!(
            AndiMsSampleState::Plasma,
            AndiMsSampleState::from_str("Plasma").unwrap()
        );
        assert_eq!(
            AndiMsSampleState::OtherState,
            AndiMsSampleState::from_str("Other State").unwrap()
        );
        assert!(AndiMsSampleState::from_str("Illegal string").is_err());
        assert_eq!(AndiMsSampleState::OtherState, AndiMsSampleState::default());
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_sample_state_to_string_conversion() {
        assert_eq!("Solid", AndiMsSampleState::Solid.to_string(),);
        assert_eq!("Liquid", AndiMsSampleState::Liquid.to_string(),);
        assert_eq!("Gas", AndiMsSampleState::Gas.to_string(),);
        assert_eq!(
            "Supercritical Fluid",
            AndiMsSampleState::SupercriticalFluid.to_string(),
        );
        assert_eq!("Plasma", AndiMsSampleState::Plasma.to_string(),);
        assert_eq!("Other State", AndiMsSampleState::OtherState.to_string(),);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_separation_method_from_string_conversion() {
        assert_eq!(
            AndiMsSeparationMethod::Glc,
            AndiMsSeparationMethod::from_str("Gas-Liquid Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Gsc,
            AndiMsSeparationMethod::from_str("Gas-Solid Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Nplc,
            AndiMsSeparationMethod::from_str("Normal Phase Liquid Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Rplc,
            AndiMsSeparationMethod::from_str("Reverse Phase Liquid Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Ielc,
            AndiMsSeparationMethod::from_str("Ion Exchange Liquid Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Selc,
            AndiMsSeparationMethod::from_str("Size Exclusion Liquid Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Iplc,
            AndiMsSeparationMethod::from_str("Ion Pair Liquid Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Olc,
            AndiMsSeparationMethod::from_str("Other Liquid Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Sfc,
            AndiMsSeparationMethod::from_str("Supercritical Fluid Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Tlc,
            AndiMsSeparationMethod::from_str("Thin Layer Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Fff,
            AndiMsSeparationMethod::from_str("Field Flow Fractionation").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Cze,
            AndiMsSeparationMethod::from_str("Capillary Zone Electrophoresis").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::Other,
            AndiMsSeparationMethod::from_str("Other Chromatography").unwrap()
        );
        assert_eq!(
            AndiMsSeparationMethod::None,
            AndiMsSeparationMethod::from_str("No Chromatography").unwrap()
        );
        assert!(AndiMsSeparationMethod::from_str("Illegal string").is_err());
        assert_eq!(
            AndiMsSeparationMethod::None,
            AndiMsSeparationMethod::default()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_separation_method_to_string_conversion() {
        assert_eq!(
            "Gas-Liquid Chromatography",
            AndiMsSeparationMethod::Glc.to_string(),
        );
        assert_eq!(
            "Gas-Solid Chromatography",
            AndiMsSeparationMethod::Gsc.to_string(),
        );
        assert_eq!(
            "Normal Phase Liquid Chromatography",
            AndiMsSeparationMethod::Nplc.to_string(),
        );
        assert_eq!(
            "Reverse Phase Liquid Chromatography",
            AndiMsSeparationMethod::Rplc.to_string(),
        );
        assert_eq!(
            "Ion Exchange Liquid Chromatography",
            AndiMsSeparationMethod::Ielc.to_string(),
        );
        assert_eq!(
            "Size Exclusion Liquid Chromatography",
            AndiMsSeparationMethod::Selc.to_string(),
        );
        assert_eq!(
            "Ion Pair Liquid Chromatography",
            AndiMsSeparationMethod::Iplc.to_string(),
        );
        assert_eq!(
            "Other Liquid Chromatography",
            AndiMsSeparationMethod::Olc.to_string(),
        );
        assert_eq!(
            "Supercritical Fluid Chromatography",
            AndiMsSeparationMethod::Sfc.to_string(),
        );
        assert_eq!(
            "Thin Layer Chromatography",
            AndiMsSeparationMethod::Tlc.to_string(),
        );
        assert_eq!(
            "Field Flow Fractionation",
            AndiMsSeparationMethod::Fff.to_string(),
        );
        assert_eq!(
            "Capillary Zone Electrophoresis",
            AndiMsSeparationMethod::Cze.to_string(),
        );
        assert_eq!(
            "Other Chromatography",
            AndiMsSeparationMethod::Other.to_string(),
        );
        assert_eq!(
            "No Chromatography",
            AndiMsSeparationMethod::None.to_string(),
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_mass_spectrometer_inlet_from_string_conversion() {
        assert_eq!(
            AndiMsMassSpectrometerInlet::Membrane,
            AndiMsMassSpectrometerInlet::from_str("Membrane Separator").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Capillary,
            AndiMsMassSpectrometerInlet::from_str("Capillary Direct").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::OpenSplit,
            AndiMsMassSpectrometerInlet::from_str("Open Split").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Jet,
            AndiMsMassSpectrometerInlet::from_str("Jet Separator").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Direct,
            AndiMsMassSpectrometerInlet::from_str("Direct Inlet Probe").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Septum,
            AndiMsMassSpectrometerInlet::from_str("Septum").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Pb,
            AndiMsMassSpectrometerInlet::from_str("Particle Beam").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Reservoir,
            AndiMsMassSpectrometerInlet::from_str("Reservoir").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Belt,
            AndiMsMassSpectrometerInlet::from_str("Moving Belt").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Apci,
            AndiMsMassSpectrometerInlet::from_str("Atmospheric Pressure Chemical Ionization Inlet")
                .unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Fia,
            AndiMsMassSpectrometerInlet::from_str("Flow Injection Analysis").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Es,
            AndiMsMassSpectrometerInlet::from_str("Electrospray Inlet").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Infusion,
            AndiMsMassSpectrometerInlet::from_str("Infusion").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Ts,
            AndiMsMassSpectrometerInlet::from_str("Thermospray Inlet").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Probe,
            AndiMsMassSpectrometerInlet::from_str("Other Probe").unwrap()
        );
        assert_eq!(
            AndiMsMassSpectrometerInlet::Other,
            AndiMsMassSpectrometerInlet::from_str("Other Inlet").unwrap()
        );
        assert!(AndiMsMassSpectrometerInlet::from_str("Illegal string").is_err());
        assert_eq!(
            AndiMsMassSpectrometerInlet::Direct,
            AndiMsMassSpectrometerInlet::default()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_mass_spectrometer_inlet_to_string_conversion() {
        assert_eq!(
            "Membrane Separator",
            AndiMsMassSpectrometerInlet::Membrane.to_string(),
        );
        assert_eq!(
            "Capillary Direct",
            AndiMsMassSpectrometerInlet::Capillary.to_string(),
        );
        assert_eq!(
            "Open Split",
            AndiMsMassSpectrometerInlet::OpenSplit.to_string(),
        );
        assert_eq!(
            "Jet Separator",
            AndiMsMassSpectrometerInlet::Jet.to_string(),
        );
        assert_eq!(
            "Direct Inlet Probe",
            AndiMsMassSpectrometerInlet::Direct.to_string(),
        );
        assert_eq!("Septum", AndiMsMassSpectrometerInlet::Septum.to_string(),);
        assert_eq!("Particle Beam", AndiMsMassSpectrometerInlet::Pb.to_string(),);
        assert_eq!(
            "Reservoir",
            AndiMsMassSpectrometerInlet::Reservoir.to_string(),
        );
        assert_eq!("Moving Belt", AndiMsMassSpectrometerInlet::Belt.to_string(),);
        assert_eq!(
            "Atmospheric Pressure Chemical Ionization Inlet",
            AndiMsMassSpectrometerInlet::Apci.to_string(),
        );
        assert_eq!(
            "Flow Injection Analysis",
            AndiMsMassSpectrometerInlet::Fia.to_string(),
        );
        assert_eq!(
            "Electrospray Inlet",
            AndiMsMassSpectrometerInlet::Es.to_string(),
        );
        assert_eq!(
            "Infusion",
            AndiMsMassSpectrometerInlet::Infusion.to_string(),
        );
        assert_eq!(
            "Thermospray Inlet",
            AndiMsMassSpectrometerInlet::Ts.to_string(),
        );
        assert_eq!(
            "Other Probe",
            AndiMsMassSpectrometerInlet::Probe.to_string(),
        );
        assert_eq!(
            "Other Inlet",
            AndiMsMassSpectrometerInlet::Other.to_string(),
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_ionization_method_inlet_from_string_conversion() {
        assert_eq!(
            AndiMsIonizationMethod::Ei,
            AndiMsIonizationMethod::from_str("Electron Impact").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Ci,
            AndiMsIonizationMethod::from_str("Chemical Ionization").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Fab,
            AndiMsIonizationMethod::from_str("Fast Atom Bombardment").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Fd,
            AndiMsIonizationMethod::from_str("Field Desorption").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Fi,
            AndiMsIonizationMethod::from_str("Field Ionization").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Es,
            AndiMsIonizationMethod::from_str("Electrospray Ionization").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Ts,
            AndiMsIonizationMethod::from_str("Thermospray Ionization").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Apci,
            AndiMsIonizationMethod::from_str("Atmospheric Pressure Chemical Ionization").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Pd,
            AndiMsIonizationMethod::from_str("Plasma Desorption").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Ld,
            AndiMsIonizationMethod::from_str("Laser Desorption").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Spark,
            AndiMsIonizationMethod::from_str("Spark Ionization").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Thermal,
            AndiMsIonizationMethod::from_str("Thermal Ionization").unwrap()
        );
        assert_eq!(
            AndiMsIonizationMethod::Other,
            AndiMsIonizationMethod::from_str("Other Ionization").unwrap()
        );
        assert!(AndiMsIonizationMethod::from_str("Illegal string").is_err());
        assert_eq!(
            AndiMsIonizationMethod::Ei,
            AndiMsIonizationMethod::default()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_ionization_method_inlet_to_string_conversion() {
        assert_eq!("Electron Impact", AndiMsIonizationMethod::Ei.to_string(),);
        assert_eq!(
            "Chemical Ionization",
            AndiMsIonizationMethod::Ci.to_string(),
        );
        assert_eq!(
            "Fast Atom Bombardment",
            AndiMsIonizationMethod::Fab.to_string(),
        );
        assert_eq!("Field Desorption", AndiMsIonizationMethod::Fd.to_string(),);
        assert_eq!("Field Ionization", AndiMsIonizationMethod::Fi.to_string(),);
        assert_eq!(
            "Electrospray Ionization",
            AndiMsIonizationMethod::Es.to_string(),
        );
        assert_eq!(
            "Thermospray Ionization",
            AndiMsIonizationMethod::Ts.to_string(),
        );
        assert_eq!(
            "Atmospheric Pressure Chemical Ionization",
            AndiMsIonizationMethod::Apci.to_string(),
        );
        assert_eq!("Plasma Desorption", AndiMsIonizationMethod::Pd.to_string(),);
        assert_eq!("Laser Desorption", AndiMsIonizationMethod::Ld.to_string(),);
        assert_eq!(
            "Spark Ionization",
            AndiMsIonizationMethod::Spark.to_string(),
        );
        assert_eq!(
            "Thermal Ionization",
            AndiMsIonizationMethod::Thermal.to_string(),
        );
        assert_eq!(
            "Other Ionization",
            AndiMsIonizationMethod::Other.to_string(),
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_ionization_polarity_from_string_conversion() {
        assert_eq!(
            AndiMsIonizationPolarity::Plus,
            AndiMsIonizationPolarity::from_str("Positive Polarity").unwrap()
        );
        assert_eq!(
            AndiMsIonizationPolarity::Minus,
            AndiMsIonizationPolarity::from_str("Negative Polarity").unwrap()
        );
        assert!(AndiMsIonizationPolarity::from_str("Illegal string").is_err());
        assert_eq!(
            AndiMsIonizationPolarity::Plus,
            AndiMsIonizationPolarity::default()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_ionization_polarity_to_string_conversion() {
        assert_eq!(
            "Positive Polarity",
            AndiMsIonizationPolarity::Plus.to_string(),
        );
        assert_eq!(
            "Negative Polarity",
            AndiMsIonizationPolarity::Minus.to_string(),
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_detector_type_from_string_conversion() {
        assert_eq!(
            AndiMsDetectorType::Em,
            AndiMsDetectorType::from_str("Electron Multiplier").unwrap()
        );
        assert_eq!(
            AndiMsDetectorType::Pm,
            AndiMsDetectorType::from_str("Photomultplier").unwrap()
        );
        assert_eq!(
            AndiMsDetectorType::Focal,
            AndiMsDetectorType::from_str("Focal Plane Array").unwrap()
        );
        assert_eq!(
            AndiMsDetectorType::Cup,
            AndiMsDetectorType::from_str("Faraday Cup").unwrap()
        );
        assert_eq!(
            AndiMsDetectorType::DynodeEm,
            AndiMsDetectorType::from_str("Conversion Dynode Electron Multiplier").unwrap()
        );
        assert_eq!(
            AndiMsDetectorType::DynodePm,
            AndiMsDetectorType::from_str("Conversion Dynode Photomultiplier").unwrap()
        );
        assert_eq!(
            AndiMsDetectorType::Multicoll,
            AndiMsDetectorType::from_str("Multicollector").unwrap()
        );
        assert_eq!(
            AndiMsDetectorType::Other,
            AndiMsDetectorType::from_str("Other Detector").unwrap()
        );
        assert!(AndiMsDetectorType::from_str("Illegal string").is_err());
        assert_eq!(AndiMsDetectorType::Em, AndiMsDetectorType::default());
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_detector_type_to_string_conversion() {
        assert_eq!("Electron Multiplier", AndiMsDetectorType::Em.to_string(),);
        assert_eq!("Photomultplier", AndiMsDetectorType::Pm.to_string(),);
        assert_eq!("Focal Plane Array", AndiMsDetectorType::Focal.to_string(),);
        assert_eq!("Faraday Cup", AndiMsDetectorType::Cup.to_string(),);
        assert_eq!(
            "Conversion Dynode Electron Multiplier",
            AndiMsDetectorType::DynodeEm.to_string(),
        );
        assert_eq!(
            "Conversion Dynode Photomultiplier",
            AndiMsDetectorType::DynodePm.to_string(),
        );
        assert_eq!("Multicollector", AndiMsDetectorType::Multicoll.to_string(),);
        assert_eq!("Other Detector", AndiMsDetectorType::Other.to_string(),);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_resolution_type_from_string_conversion() {
        assert_eq!(
            AndiMsResolutionType::Constant,
            AndiMsResolutionType::from_str("Constant Resolution").unwrap()
        );
        assert_eq!(
            AndiMsResolutionType::Proportional,
            AndiMsResolutionType::from_str("Proportional Resolution").unwrap()
        );
        assert!(AndiMsResolutionType::from_str("Illegal string").is_err());
        assert_eq!(
            AndiMsResolutionType::Constant,
            AndiMsResolutionType::default()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_resolution_type_to_string_conversion() {
        assert_eq!(
            "Constant Resolution",
            AndiMsResolutionType::Constant.to_string(),
        );
        assert_eq!(
            "Proportional Resolution",
            AndiMsResolutionType::Proportional.to_string(),
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_scan_function_from_string_conversion() {
        assert_eq!(
            AndiMsScanFunction::Scan,
            AndiMsScanFunction::from_str("Mass Scan").unwrap()
        );
        assert_eq!(
            AndiMsScanFunction::Sid,
            AndiMsScanFunction::from_str("Selected Ion Detection").unwrap()
        );
        assert_eq!(
            AndiMsScanFunction::Other,
            AndiMsScanFunction::from_str("Other Function").unwrap()
        );
        assert!(AndiMsScanFunction::from_str("Illegal string").is_err());
        assert_eq!(AndiMsScanFunction::Scan, AndiMsScanFunction::default());
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_scan_function_to_string_conversion() {
        assert_eq!("Mass Scan", AndiMsScanFunction::Scan.to_string(),);
        assert_eq!(
            "Selected Ion Detection",
            AndiMsScanFunction::Sid.to_string(),
        );
        assert_eq!("Other Function", AndiMsScanFunction::Other.to_string(),);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_scan_direction_from_string_conversion() {
        assert_eq!(
            AndiMsScanDirection::Up,
            AndiMsScanDirection::from_str("Up").unwrap()
        );
        assert_eq!(
            AndiMsScanDirection::Down,
            AndiMsScanDirection::from_str("Down").unwrap()
        );
        assert_eq!(
            AndiMsScanDirection::Other,
            AndiMsScanDirection::from_str("Other Direction").unwrap()
        );
        assert!(AndiMsScanDirection::from_str("Illegal string").is_err());
        assert_eq!(AndiMsScanDirection::Up, AndiMsScanDirection::default());
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_scan_direction_to_string_conversion() {
        assert_eq!("Up", AndiMsScanDirection::Up.to_string(),);
        assert_eq!("Down", AndiMsScanDirection::Down.to_string(),);
        assert_eq!("Other Direction", AndiMsScanDirection::Other.to_string(),);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_scan_law_from_string_conversion() {
        assert_eq!(
            AndiMsScanLaw::Linear,
            AndiMsScanLaw::from_str("Linear").unwrap()
        );
        assert_eq!(
            AndiMsScanLaw::Exponential,
            AndiMsScanLaw::from_str("Exponential").unwrap()
        );
        assert_eq!(
            AndiMsScanLaw::Quadratic,
            AndiMsScanLaw::from_str("Quadratic").unwrap()
        );
        assert_eq!(
            AndiMsScanLaw::Other,
            AndiMsScanLaw::from_str("Other Law").unwrap()
        );
        assert!(AndiMsScanLaw::from_str("Illegal string").is_err());
        assert_eq!(AndiMsScanLaw::Linear, AndiMsScanLaw::default());
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_scan_law_to_string_conversion() {
        assert_eq!("Linear", AndiMsScanLaw::Linear.to_string(),);
        assert_eq!("Exponential", AndiMsScanLaw::Exponential.to_string(),);
        assert_eq!("Quadratic", AndiMsScanLaw::Quadratic.to_string(),);
        assert_eq!("Other Law", AndiMsScanLaw::Other.to_string(),);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_mass_axis_unit_from_string_conversion() {
        assert_eq!(
            AndiMsMassAxisUnit::Mz,
            AndiMsMassAxisUnit::from_str("M/Z").unwrap()
        );
        assert_eq!(
            AndiMsMassAxisUnit::Arbitrary,
            AndiMsMassAxisUnit::from_str("Arbitrary Mass Units").unwrap()
        );
        assert_eq!(
            AndiMsMassAxisUnit::Other,
            AndiMsMassAxisUnit::from_str("Other Mass").unwrap()
        );
        assert!(AndiMsMassAxisUnit::from_str("Illegal string").is_err());
        assert_eq!(AndiMsMassAxisUnit::Mz, AndiMsMassAxisUnit::default());
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_mass_axis_unit_to_string_conversion() {
        assert_eq!("M/Z", AndiMsMassAxisUnit::Mz.to_string(),);
        assert_eq!(
            "Arbitrary Mass Units",
            AndiMsMassAxisUnit::Arbitrary.to_string(),
        );
        assert_eq!("Other Mass", AndiMsMassAxisUnit::Other.to_string(),);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_time_axis_unit_from_string_conversion() {
        assert_eq!(
            AndiMsTimeAxisUnit::Seconds,
            AndiMsTimeAxisUnit::from_str("Seconds").unwrap()
        );
        assert_eq!(
            AndiMsTimeAxisUnit::Arbitrary,
            AndiMsTimeAxisUnit::from_str("Arbitrary Time Units").unwrap()
        );
        assert_eq!(
            AndiMsTimeAxisUnit::Other,
            AndiMsTimeAxisUnit::from_str("Other Time").unwrap()
        );
        assert!(AndiMsTimeAxisUnit::from_str("Illegal string").is_err());
        assert_eq!(AndiMsTimeAxisUnit::Seconds, AndiMsTimeAxisUnit::default());
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_time_axis_unit_to_string_conversion() {
        assert_eq!("Seconds", AndiMsTimeAxisUnit::Seconds.to_string(),);
        assert_eq!(
            "Arbitrary Time Units",
            AndiMsTimeAxisUnit::Arbitrary.to_string(),
        );
        assert_eq!("Other Time", AndiMsTimeAxisUnit::Other.to_string(),);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_intensity_axis_unit_from_string_conversion() {
        assert_eq!(
            AndiMsIntensityAxisUnit::Counts,
            AndiMsIntensityAxisUnit::from_str("Total counts").unwrap()
        );
        assert_eq!(
            AndiMsIntensityAxisUnit::Cps,
            AndiMsIntensityAxisUnit::from_str("Counts Per Second").unwrap()
        );
        assert_eq!(
            AndiMsIntensityAxisUnit::Volts,
            AndiMsIntensityAxisUnit::from_str("Volts").unwrap()
        );
        assert_eq!(
            AndiMsIntensityAxisUnit::Current,
            AndiMsIntensityAxisUnit::from_str("Current").unwrap()
        );
        assert_eq!(
            AndiMsIntensityAxisUnit::Arbitrary,
            AndiMsIntensityAxisUnit::from_str("Arbitrary Intensity Units").unwrap()
        );
        assert_eq!(
            AndiMsIntensityAxisUnit::Other,
            AndiMsIntensityAxisUnit::from_str("Other Intensity").unwrap()
        );
        assert!(AndiMsIntensityAxisUnit::from_str("Illegal string").is_err());
        assert_eq!(
            AndiMsIntensityAxisUnit::Arbitrary,
            AndiMsIntensityAxisUnit::default()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_intensity_axis_unit_to_string_conversion() {
        assert_eq!("Total counts", AndiMsIntensityAxisUnit::Counts.to_string(),);
        assert_eq!(
            "Counts Per Second",
            AndiMsIntensityAxisUnit::Cps.to_string(),
        );
        assert_eq!("Volts", AndiMsIntensityAxisUnit::Volts.to_string(),);
        assert_eq!("Current", AndiMsIntensityAxisUnit::Current.to_string(),);
        assert_eq!(
            "Arbitrary Intensity Units",
            AndiMsIntensityAxisUnit::Arbitrary.to_string(),
        );
        assert_eq!(
            "Other Intensity",
            AndiMsIntensityAxisUnit::Other.to_string(),
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_data_fomat_from_string_conversion() {
        assert_eq!(
            AndiMsDataFormat::Short,
            AndiMsDataFormat::from_str("Short").unwrap()
        );
        assert_eq!(
            AndiMsDataFormat::Long,
            AndiMsDataFormat::from_str("Long").unwrap()
        );
        assert_eq!(
            AndiMsDataFormat::Float,
            AndiMsDataFormat::from_str("Float").unwrap()
        );
        assert_eq!(
            AndiMsDataFormat::Double,
            AndiMsDataFormat::from_str("Double").unwrap()
        );
        assert!(AndiMsDataFormat::from_str("Illegal string").is_err());
        assert_eq!(AndiMsDataFormat::Short, AndiMsDataFormat::default());
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ms_data_fomat_to_string_conversion() {
        assert_eq!("Short", AndiMsDataFormat::Short.to_string(),);
        assert_eq!("Long", AndiMsDataFormat::Long.to_string(),);
        assert_eq!("Float", AndiMsDataFormat::Float.to_string(),);
        assert_eq!("Double", AndiMsDataFormat::Double.to_string(),);
    }
}
