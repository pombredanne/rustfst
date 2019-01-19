extern crate rustfst;
#[macro_use]
extern crate serde_derive;

use std::fs::read_to_string;
use std::string::String;

use failure::format_err;
use failure::Fail;
use failure::Fallible;
use failure::ResultExt;

use rustfst::algorithms::arc_mappers::{
    IdentityArcMapper, InputEpsilonMapper, InvertWeightMapper, OutputEpsilonMapper, PlusMapper,
    QuantizeMapper, RmWeightMapper, TimesMapper,
};
use rustfst::algorithms::{
    connect, decode, encode, invert, isomorphic, project, push_weights, reverse, rm_epsilon,
    ProjectType, ReweightType,
};
use rustfst::fst_impls::VectorFst;
use rustfst::fst_traits::{MutableFst, TextParser};
use rustfst::semirings::{Semiring, TropicalWeight};

#[derive(Serialize, Deserialize, Debug)]
struct OperationResult {
    result: String,
}

impl OperationResult {
    fn parse<F>(&self) -> F
    where
        F: TextParser,
        F::W: Semiring<Type = f32>,
    {
        F::from_text_string(self.result.as_str()).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EncodeOperationResult {
    encode_labels: bool,
    encode_weights: bool,
    result: String,
}

impl EncodeOperationResult {
    fn parse<F>(&self) -> EncodeTestData<F>
    where
        F: TextParser,
        F::W: Semiring<Type = f32>,
    {
        EncodeTestData {
            encode_weights: self.encode_weights,
            encode_labels: self.encode_labels,
            result: F::from_text_string(self.result.as_str()).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ParsedTestData {
    rmepsilon: OperationResult,
    name: String,
    invert: OperationResult,
    weight_type: String,
    raw: OperationResult,
    project_output: OperationResult,
    connect: OperationResult,
    weight_pushing_initial: OperationResult,
    weight_pushing_final: OperationResult,
    project_input: OperationResult,
    reverse: OperationResult,
    arc_map_identity: OperationResult,
    arc_map_rmweight: OperationResult,
    arc_map_invert: OperationResult,
    arc_map_input_epsilon: OperationResult,
    arc_map_output_epsilon: OperationResult,
    arc_map_plus: OperationResult,
    arc_map_times: OperationResult,
    arc_map_quantize: OperationResult,
    encode: Vec<EncodeOperationResult>,
    encode_decode: Vec<EncodeOperationResult>,
}

struct EncodeTestData<F>
where
    F: TextParser,
    F::W: Semiring<Type = f32>,
{
    encode_labels: bool,
    encode_weights: bool,
    result: F,
}

struct TestData<F>
where
    F: TextParser,
    F::W: Semiring<Type = f32>,
{
    rmepsilon: F,
    #[allow(unused)]
    name: String,
    invert: F,
    raw: F,
    project_output: F,
    connect: F,
    weight_pushing_initial: F,
    weight_pushing_final: F,
    project_input: F,
    reverse: F,
    arc_map_identity: F,
    arc_map_rmweight: F,
    arc_map_invert: F,
    arc_map_input_epsilon: F,
    arc_map_output_epsilon: F,
    arc_map_plus: F,
    arc_map_times: F,
    arc_map_quantize: F,
    encode: Vec<EncodeTestData<F>>,
    encode_decode: Vec<EncodeTestData<F>>,
}

impl<F> TestData<F>
where
    F: TextParser,
    F::W: Semiring<Type = f32>,
{
    pub fn new(data: &ParsedTestData) -> Self {
        Self {
            rmepsilon: data.rmepsilon.parse(),
            name: data.name.clone(),
            invert: data.invert.parse(),
            raw: data.raw.parse(),
            project_output: data.project_output.parse(),
            connect: data.connect.parse(),
            weight_pushing_initial: data.weight_pushing_initial.parse(),
            weight_pushing_final: data.weight_pushing_final.parse(),
            project_input: data.project_input.parse(),
            reverse: data.reverse.parse(),
            arc_map_identity: data.arc_map_identity.parse(),
            arc_map_rmweight: data.arc_map_rmweight.parse(),
            arc_map_invert: data.arc_map_invert.parse(),
            arc_map_input_epsilon: data.arc_map_input_epsilon.parse(),
            arc_map_output_epsilon: data.arc_map_output_epsilon.parse(),
            arc_map_plus: data.arc_map_plus.parse(),
            arc_map_times: data.arc_map_times.parse(),
            arc_map_quantize: data.arc_map_quantize.parse(),
            encode: data.encode.iter().map(|v| v.parse()).collect(),
            encode_decode: data.encode_decode.iter().map(|v| v.parse()).collect(),
        }
    }
}

macro_rules! error_message_fst {
    ($fst_ref:expr, $fst:expr, $operation_name:expr) => {
        format!(
            "\nTest {} with pynini failing : \nREF = \n{}\nPRED = \n{}\n",
            $operation_name, $fst_ref, $fst
        )
    };
}

fn run_test_pynini(test_name: &str) -> Fallible<()> {
    let mut absolute_path = std::env::current_dir()?;
    absolute_path.push("fst-test-data");
    absolute_path.push("fst_test_data");
    absolute_path.push(test_name);
    absolute_path.push("metadata.json");

    let string = read_to_string(absolute_path).unwrap();
    let parsed_test_data: ParsedTestData = serde_json::from_str(&string).unwrap();
    let test_data: TestData<VectorFst<TropicalWeight>> = TestData::new(&parsed_test_data);

    test_rmepsilon(&test_data)?;

    test_invert(&test_data)?;

    test_project_output(&test_data)?;

    test_project_input(&test_data)?;

    test_reverse(&test_data)?;

    test_connect(&test_data)?;

    test_weight_pushing_initial(&test_data)?;

    test_weight_pushing_final(&test_data)?;

    test_arc_map_identity(&test_data)?;

    test_arc_map_rmweight(&test_data)?;

    test_arc_map_invert(&test_data)?;

    test_arc_map_input_epsilon(&test_data)?;

    test_arc_map_output_epsilon(&test_data)?;

    test_arc_map_plus(&test_data)?;

    test_arc_map_times(&test_data)?;

    test_arc_map_quantize(&test_data)?;

    test_encode(&test_data)?;

    test_encode_decode(&test_data)?;

    Ok(())
}

fn test_rmepsilon(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // Remove epsilon
    let fst_rmepsilon: VectorFst<TropicalWeight> = rm_epsilon(&test_data.raw).unwrap();
    assert!(
        isomorphic(&fst_rmepsilon, &test_data.rmepsilon)?,
        "{}",
        error_message_fst!(test_data.rmepsilon, fst_rmepsilon, "RmEpsilon")
    );
    Ok(())
}

fn test_invert(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // Invert
    let mut fst_invert = test_data.raw.clone();
    invert(&mut fst_invert);
    assert_eq!(
        test_data.invert,
        fst_invert,
        "{}",
        error_message_fst!(test_data.invert, fst_invert, "Invert")
    );
    Ok(())
}

fn test_project_output(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // Project output
    let mut fst_project_output = test_data.raw.clone();
    project(&mut fst_project_output, ProjectType::ProjectOutput);
    assert_eq!(
        test_data.project_output,
        fst_project_output,
        "{}",
        error_message_fst!(
            test_data.project_output,
            fst_project_output,
            "Project Output"
        )
    );
    Ok(())
}

fn test_project_input(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // Project input
    let mut fst_project_input = test_data.raw.clone();
    project(&mut fst_project_input, ProjectType::ProjectInput);
    assert_eq!(
        test_data.project_input,
        fst_project_input,
        "{}",
        error_message_fst!(test_data.project_input, fst_project_input, "Project Input")
    );
    Ok(())
}

fn test_reverse(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // Reverse
    let fst_reverse: VectorFst<TropicalWeight> = reverse(&test_data.raw).unwrap();
    assert!(
        isomorphic(&test_data.reverse, &fst_reverse)?,
        "{}",
        error_message_fst!(test_data.reverse, fst_reverse, "Reverse")
    );
    Ok(())
}

fn test_connect(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // Connect
    let mut fst_connect = test_data.raw.clone();
    connect(&mut fst_connect)?;
    assert_eq!(
        test_data.connect,
        fst_connect,
        "{}",
        error_message_fst!(test_data.connect, fst_connect, "Connect")
    );
    Ok(())
}

fn test_weight_pushing_initial(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // Weight pushing initial
    let mut fst_weight_push_initial = test_data.raw.clone();
    push_weights(
        &mut fst_weight_push_initial,
        ReweightType::ReweightToInitial,
    )?;
    assert_eq!(
        test_data.weight_pushing_initial,
        fst_weight_push_initial,
        "{}",
        error_message_fst!(
            test_data.weight_pushing_initial,
            fst_weight_push_initial,
            "Weight Pushing initial"
        )
    );
    Ok(())
}

fn test_weight_pushing_final(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // Weight pushing final
    let mut fst_weight_push_final = test_data.raw.clone();
    push_weights(&mut fst_weight_push_final, ReweightType::ReweightToFinal)?;
    assert_eq!(
        test_data.weight_pushing_final,
        fst_weight_push_final,
        "{}",
        error_message_fst!(
            test_data.weight_pushing_final,
            fst_weight_push_final,
            "Weight Pushing final"
        )
    );
    Ok(())
}

fn test_arc_map_identity(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // ArcMap IdentityMapper
    let mut fst_arc_map_identity = test_data.raw.clone();
    let mut identity_mapper = IdentityArcMapper {};
    fst_arc_map_identity.arc_map(&mut identity_mapper)?;
    assert_eq!(
        test_data.arc_map_identity,
        fst_arc_map_identity,
        "{}",
        error_message_fst!(
            test_data.arc_map_identity,
            fst_arc_map_identity,
            "ArcMap identity"
        )
    );
    Ok(())
}

fn test_arc_map_rmweight(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // ArcMap RmWeightMapper
    let mut fst_arc_map_rmweight = test_data.raw.clone();
    let mut rmweight_mapper = RmWeightMapper {};
    fst_arc_map_rmweight.arc_map(&mut rmweight_mapper)?;
    assert_eq!(
        test_data.arc_map_rmweight,
        fst_arc_map_rmweight,
        "{}",
        error_message_fst!(
            test_data.arc_map_rmweight,
            fst_arc_map_rmweight,
            "ArcMap RmWeight"
        )
    );
    Ok(())
}

fn test_arc_map_invert(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    // ArcMap InvertWeightMapper
    let mut fst_arc_map_invert = test_data.raw.clone();
    let mut invertweight_mapper = InvertWeightMapper {};
    fst_arc_map_invert.arc_map(&mut invertweight_mapper)?;
    assert_eq!(
        test_data.arc_map_invert,
        fst_arc_map_invert,
        "{}",
        error_message_fst!(
            test_data.arc_map_invert,
            fst_arc_map_invert,
            "ArcMap InvertWeight"
        )
    );
    Ok(())
}

fn test_arc_map_input_epsilon(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    let mut fst_arc_map = test_data.raw.clone();
    let mut mapper = InputEpsilonMapper {};
    fst_arc_map.arc_map(&mut mapper)?;
    assert_eq!(
        test_data.arc_map_input_epsilon,
        fst_arc_map,
        "{}",
        error_message_fst!(
            test_data.arc_map_input_epsilon,
            fst_arc_map,
            "ArcMap InputEpsilonMapper"
        )
    );
    Ok(())
}

fn test_arc_map_output_epsilon(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    let mut fst_arc_map = test_data.raw.clone();
    let mut mapper = OutputEpsilonMapper {};
    fst_arc_map.arc_map(&mut mapper)?;
    assert_eq!(
        test_data.arc_map_output_epsilon,
        fst_arc_map,
        "{}",
        error_message_fst!(
            test_data.arc_map_output_epsilon,
            fst_arc_map,
            "ArcMap OutputEpsilonMapper"
        )
    );
    Ok(())
}

fn test_arc_map_plus(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    let mut fst_arc_map = test_data.raw.clone();
    let mut mapper = PlusMapper::new(1.5);
    fst_arc_map.arc_map(&mut mapper)?;
    assert_eq!(
        test_data.arc_map_plus,
        fst_arc_map,
        "{}",
        error_message_fst!(
            test_data.arc_map_plus,
            fst_arc_map,
            "ArcMap PlusMapper (1.5)"
        )
    );
    Ok(())
}

fn test_arc_map_times(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    let mut fst_arc_map = test_data.raw.clone();
    let mut mapper = TimesMapper::new(1.5);
    fst_arc_map.arc_map(&mut mapper)?;
    assert_eq!(
        test_data.arc_map_times,
        fst_arc_map,
        "{}",
        error_message_fst!(
            test_data.arc_map_times,
            fst_arc_map,
            "ArcMap TimesMapper (1.5)"
        )
    );
    Ok(())
}

fn test_arc_map_quantize(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    let mut fst_arc_map = test_data.raw.clone();
    let mut mapper = QuantizeMapper {};
    fst_arc_map.arc_map(&mut mapper)?;
    assert_eq!(
        test_data.arc_map_quantize,
        fst_arc_map,
        "{}",
        error_message_fst!(
            test_data.arc_map_quantize,
            fst_arc_map,
            "ArcMap QuantizeMapper"
        )
    );
    Ok(())
}

fn test_encode(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    for encode_test_data in &test_data.encode {
        let mut fst_encoded = test_data.raw.clone();
        encode(&mut fst_encoded, encode_test_data.encode_labels, encode_test_data.encode_weights)
            .with_context(|_| format_err!(
            "Error when running test_encode with parameters encode_labels={:?} and encode_weights={:?}.",
            encode_test_data.encode_labels, encode_test_data.encode_weights))?;
        assert_eq!(
            encode_test_data.result,
            fst_encoded,
            "{}",
            error_message_fst!(encode_test_data.result, fst_encoded, "Encode")
        );
    }
    Ok(())
}

fn test_encode_decode(test_data: &TestData<VectorFst<TropicalWeight>>) -> Fallible<()> {
    for encode_test_data in &test_data.encode_decode {
        let mut fst_encoded = test_data.raw.clone();
        let encode_table = encode(&mut fst_encoded, encode_test_data.encode_labels, encode_test_data.encode_weights)
            .with_context(|_| format_err!(
            "Error when running test_encode_decode with parameters encode_labels={:?} and encode_weights={:?}.",
            encode_test_data.encode_labels, encode_test_data.encode_weights))?;
        decode(&mut fst_encoded, encode_table)?;
        assert_eq!(
            encode_test_data.result,
            fst_encoded,
            "{}",
            error_message_fst!(
                encode_test_data.result,
                fst_encoded,
                format!(
                    "Encode/Decode with encode_labels={:?} and encode_weights={:?}",
                    encode_test_data.encode_labels, encode_test_data.encode_weights
                )
            )
        );
    }
    Ok(())
}

pub struct ExitFailure(failure::Error);

/// Prints a list of causes for this Error, along with any backtrace
/// information collected by the Error (if RUST_BACKTRACE=1).
impl std::fmt::Debug for ExitFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fail = self.0.as_fail();

        writeln!(f, "{}", &fail)?;

        let mut x: &Fail = fail;
        while let Some(cause) = x.cause() {
            writeln!(f, " -> caused by: {}", &cause)?;
            x = cause;
        }
        if let Some(backtrace) = x.backtrace() {
            writeln!(f, "{:?}", backtrace)?;
        }

        Ok(())
    }
}

impl<T: Into<failure::Error>> From<T> for ExitFailure {
    fn from(t: T) -> Self {
        ExitFailure(t.into())
    }
}
use std::result::Result as ResultRust;

#[test]
fn test_pynini_fst_000() -> ResultRust<(), ExitFailure> {
    run_test_pynini("fst_000").map_err(|v| v.into())
}

#[test]
fn test_pynini_fst_001() -> ResultRust<(), ExitFailure> {
    run_test_pynini("fst_001").map_err(|v| v.into())
}

#[test]
fn test_pynini_fst_002() -> ResultRust<(), ExitFailure> {
    run_test_pynini("fst_002").map_err(|v| v.into())
}

#[test]
fn test_pynini_fst_003() -> ResultRust<(), ExitFailure> {
    run_test_pynini("fst_003").map_err(|v| v.into())
}

#[test]
fn test_pynini_fst_004() -> ResultRust<(), ExitFailure> {
    run_test_pynini("fst_004").map_err(|v| v.into())
}