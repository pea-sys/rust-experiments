use csv::{Reader, Writer, Error};
use std::io;
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}
#[derive(Serialize)]
struct Record2<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}
#[derive(Debug)]
struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}
#[derive(Debug, Deserialize)]
struct Row {
    color_name: String,
    color: HexColor,
}
impl FromStr for HexColor {
    type Err = Error;

    fn from_str(hex_color: &str) -> std::result::Result<Self, Self::Err> {
        let trimmed = hex_color.trim_matches('#');
        if trimmed.len() != 6 {
            Err("Error").unwrap()
        } else {
            Ok(HexColor {
                red: u8::from_str_radix(&trimmed[..2], 16).unwrap(),
                green: u8::from_str_radix(&trimmed[2..4], 16).unwrap(),
                blue: u8::from_str_radix(&trimmed[4..6], 16).unwrap(),
            })
        }
    }
}
impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).unwrap();
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}
use csv::ReaderBuilder;

fn main() -> Result<(), Error> {
    //CSVレコードを読み込む
    let csv = "year,make,model,description
        1948,Porsche,356,Luxury sports car
        1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]
        );
    }
    //違う区切りでCSVレコードを読み込む
    let data = "name\tplace\tid
        Mark\tMelbourne\t46
        Ashley\tZurich\t92";

    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    for result in reader.deserialize::<Record>() {
        println!("{:?}", result?);
    }
    //述語にマッチするCSVレコードをフィルタリング
    let query = "CA";
    let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";

    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == query) {
            wtr.write_record(&record)?;
        }
    }

    wtr.flush()?;

    //Serdeで無効なCSVデータをハンドル
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
//レコードをCSVにシリアライズする
let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    //Serdeを使ってレコードをCSVにシリアライズする
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let rec1 = Record2 { name: "Mark", place: "Melbourne", id: 56};
    let rec2 = Record2 { name: "Ashley", place: "Sydney", id: 64};
    let rec3 = Record2 { name: "Akshat", place: "Delhi", id: 98};

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;
    //Transform CSV column
    let data = "color_name,color
    red,#ff0000
    green,#00ff00
    blue,#0000FF
    periwinkle,#ccccff
    magenta,#ff00ff"
            .to_owned();
        let mut out = Writer::from_writer(vec![]);
        let mut reader = Reader::from_reader(data.as_bytes());
        for result in reader.deserialize::<Row>() {
            let res = result?;
            out.serialize((
                res.color_name,
                res.color.red,
                res.color.green,
                res.color.blue,
            ))?;
        }
        let written = String::from_utf8(out.into_inner().unwrap()).unwrap();
        assert_eq!(Some("magenta,255,0,255"), written.lines().last());
        println!("{}", written);
    Ok(())
}
