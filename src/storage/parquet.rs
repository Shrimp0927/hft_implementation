use parquet;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;
use parquet::arrow::ArrowWriter;
use arrow::array::*;
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use tempfile::tempfile;
use std::sync::Arc;

pub fn write_to_parquet_file<T>(column_names: Vec<String>, data: Vec<Vec<T>>)
where T: Into<i128> + Clone
{
    assert_eq!(column_names.len(), data.len(), "number of columns and number of data columns must be the same");

    let converted_data: Vec<Arc<dyn Array>> = data
        .into_iter()
        .map(|arr| {
            let values: Vec<i128> = arr.into_iter().map(|v| v.into()).collect();
            let decimal_array = Decimal128Array::from_iter_values(values);
            Arc::new(decimal_array) as ArrayRef
        })
        .collect();

    let fields: Vec<Field> = column_names
        .into_iter()
        .map(|name| Field::new(name, DataType::Decimal256(76, 2), false))
        .collect();

    let props = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    let schema = Arc::new(Schema::new(fields));
    let batch = RecordBatch::try_new(schema.clone(), converted_data).unwrap();

    let file = tempfile().expect("File creation failed");
    let mut writer = ArrowWriter::try_new(file, schema, Some(props)).unwrap();
    writer.write(&batch).expect("Failed to write");
    writer.close().unwrap();
}
