use scale_generator::Scale;

#[test]
fn chromatic_scale_with_sharps() {
    let tonic = "C";
    let scale = Scale::chromatic(tonic).unwrap();
    let output = scale.enumerate();
    let expected = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn chromatic_scale_with_flats() {
    let tonic = "F";
    let scale = Scale::chromatic(tonic).unwrap();
    let output = scale.enumerate();
    let expected = [
        "F", "Gb", "G", "Ab", "A", "Bb", "B", "C", "Db", "D", "Eb", "E",
    ];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn simple_major_scale() {
    let tonic = "C";
    let intervals = "MMmMMMm";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["C", "D", "E", "F", "G", "A", "B", "C"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn major_scale_with_sharps() {
    let tonic = "G";
    let intervals = "MMmMMMm";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["G", "A", "B", "C", "D", "E", "F#", "G"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn major_scale_with_flats() {
    let tonic = "F";
    let intervals = "MMmMMMm";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["F", "G", "A", "Bb", "C", "D", "E", "F"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn minor_scale_with_sharps() {
    let tonic = "f#";
    let intervals = "MmMMmMM";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["F#", "G#", "A", "B", "C#", "D", "E", "F#"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn minor_scale_with_flats() {
    let tonic = "bb";
    let intervals = "MmMMmMM";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["Bb", "C", "Db", "Eb", "F", "Gb", "Ab", "Bb"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn dorian_mode() {
    let tonic = "d";
    let intervals = "MmMMMmM";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["D", "E", "F", "G", "A", "B", "C", "D"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn mixolydian_mode() {
    let tonic = "Eb";
    let intervals = "MMmMMmM";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["Eb", "F", "G", "Ab", "Bb", "C", "Db", "Eb"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn lydian_mode() {
    let tonic = "a";
    let intervals = "MMMmMMm";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["A", "B", "C#", "D#", "E", "F#", "G#", "A"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn phrygian_mode() {
    let tonic = "e";
    let intervals = "mMMMmMM";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["E", "F", "G", "A", "B", "C", "D", "E"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn locrian_mode() {
    let tonic = "g";
    let intervals = "mMMmMMM";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["G", "Ab", "Bb", "C", "Db", "Eb", "F", "G"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn harmonic_minor() {
    let tonic = "d";
    let intervals = "MmMMmAm";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["D", "E", "F", "G", "A", "Bb", "Db", "D"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn octatonic() {
    let tonic = "C";
    let intervals = "MmMmMmMm";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["C", "D", "D#", "F", "F#", "G#", "A", "B", "C"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn hexatonic() {
    let tonic = "Db";
    let intervals = "MMMMMM";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["Db", "Eb", "F", "G", "A", "B", "Db"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn pentatonic() {
    let tonic = "A";
    let intervals = "MMAMA";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["A", "B", "C#", "E", "F#", "A"];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn enigmatic() {
    let tonic = "G";
    let intervals = "mAMMMmm";
    let scale = Scale::new(tonic, intervals).unwrap();
    let output = scale.enumerate();
    let expected = ["G", "G#", "B", "C#", "D#", "F", "F#", "G"];
    assert_eq!(output, expected);
}
