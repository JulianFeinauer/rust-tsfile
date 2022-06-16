use std::time::SystemTime;
use tsfile_rust::{IoTDBValue, Schema, TSDataType};
use tsfile_rust::compression::CompressionType;
use tsfile_rust::encoding::TSEncoding;
use tsfile_rust::schema::{DeviceBuilder, TsFileSchemaBuilder};
use tsfile_rust::test_utils::write_ts_file;
use tsfile_rust::tsfile_writer::{DataPoint, TsFileWriter};

fn main() {
    let schema = TsFileSchemaBuilder::new()
        .add(
            "d1",
            DeviceBuilder::new()
                .add(
                    "s1",
                    TSDataType::INT64,
                    TSEncoding::PLAIN,
                    CompressionType::UNCOMPRESSED,
                )
                .add(
                    "s2",
                    TSDataType::FLOAT,
                    TSEncoding::PLAIN,
                    CompressionType::UNCOMPRESSED,
                )
                .build(),
        )
        .add(
            "d2",
            DeviceBuilder::new()
                .add(
                    "s1",
                    TSDataType::INT64,
                    TSEncoding::PLAIN,
                    CompressionType::UNCOMPRESSED,
                )
                .add(
                    "s2",
                    TSDataType::FLOAT,
                    TSEncoding::PLAIN,
                    CompressionType::UNCOMPRESSED,
                )
                .build(),
        )
        .build();

    let mut durations: Vec<f64> = vec![];
    for _ in 0..10 {
        let start = SystemTime::now();
        let mut writer = TsFileWriter::new("benchmark2.tsfile", schema.clone(), Default::default());
        for i in 0..10000001 {
            // writer.write("d1", "s1", i, IoTDBValue::LONG(i));
            // writer.write("d1", "s2", i, IoTDBValue::FLOAT(i as f32));
            // writer.write("d2", "s1", i, IoTDBValue::LONG(i));
            // writer.write("d2", "s2", i, IoTDBValue::FLOAT(i as f32));

            writer.write_many("d1", i, vec![
                DataPoint::new("s1",IoTDBValue::LONG(i)),
                DataPoint::new("s2",IoTDBValue::FLOAT(i as f32)),
            ]);
            writer.write_many("d2", i, vec![
                DataPoint::new("s1",IoTDBValue::LONG(i)),
                DataPoint::new("s2",IoTDBValue::FLOAT(i as f32)),
            ]);
        }
        writer.close();

        let end = SystemTime::now();

        let duration = end.duration_since(start);

        let duration_s = duration.unwrap_or_default().as_millis() as f64 / 1000_f64;
        durations.push(duration_s);
        println!("Execution took {:.2}s", duration_s)
    }


    println!("Results:");
    for d in durations.iter() {
        println!(" - {:.3}s", *d);
    }

    let count = *&durations.len() as f64;
    println!("Mean: {:.3}", durations.into_iter().reduce(|a, b| a + b).unwrap() / count);
}