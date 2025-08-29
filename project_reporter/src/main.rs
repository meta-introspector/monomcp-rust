use std::fs::File;
use std::path::Path;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::record_batch::RecordBatch;
use anyhow::{Result, anyhow};

#[tokio::main]
async fn main() -> Result<()> {
    let analysis_results_dir = Path::new("../analysis_results/");

    println!("| Crate Name     | Version | Description                               |");
    println!("|----------------|---------|-------------------------------------------|");

    for entry in std::fs::read_dir(analysis_results_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let crate_name = path.file_name().unwrap().to_string_lossy().to_string();
            let parquet_file_path = path.join("project_metadata-phase/data.parquet");

            if parquet_file_path.exists() {
                let file = File::open(&parquet_file_path)?;
                let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
                let mut reader = builder.build()?;

                if let Some(record_batch) = reader.next().transpose()? {
                    let name_array = record_batch.column(record_batch.schema().index_of("project_name").unwrap());
                    let version_array = record_batch.column(record_batch.schema().index_of("project_version").unwrap());
                    let description_array = record_batch.column(record_batch.schema().index_of("description").unwrap());

                    for i in 0..record_batch.num_rows() {
                        let name = arrow::array::as_string_array(name_array).value(i);
                        let version = arrow::array::as_string_array(version_array).value(i);
                        let description = arrow::array::as_string_array(description_array).value(i);
                        println!("| {:<14} | {:<7} | {:<41} |", name, version, description);
                    }
                }
            }
        }
    }

    Ok(())
}
