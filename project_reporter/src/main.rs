use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::record_batch::RecordBatch;
use anyhow::{Result, anyhow};

#[tokio::main]
async fn main() -> Result<()> {
    let analysis_results_dir = Path::new("../analysis_results/");

    println!("| Crate Name     | Version | Description                               | LoC    | Direct Deps | Build Script | Downloads | Commits |");
    println!("|----------------|---------|-------------------------------------------|--------|-------------|--------------|-----------|---------|");

    for entry in std::fs::read_dir(analysis_results_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let crate_name = path.file_name().unwrap().to_string_lossy().to_string();
            let mut project_data: HashMap<String, RecordBatch> = HashMap::new();

            let phases = vec![
                "project_metadata",
                "dependency_analysis",
                "source_code_analysis",
                "build_analysis",
                "ecosystem_analysis",
                "version_history",
            ];

            for phase in phases {
                let parquet_file_path = path.join(format!("{}-phase/data.parquet", phase));
                if parquet_file_path.exists() {
                    let file = File::open(&parquet_file_path)?;
                    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
                    let mut reader = builder.build()?;
                    if let Some(record_batch) = reader.next().transpose()? {
                        project_data.insert(phase.to_string(), record_batch);
                    }
                }
            }

            if let Some(metadata_batch) = project_data.get("project_metadata") {
                for i in 0..metadata_batch.num_rows() {
                    let name = arrow::array::as_string_array(metadata_batch.column(metadata_batch.schema().index_of("project_name").unwrap())).value(i);
                    let version = arrow::array::as_string_array(metadata_batch.column(metadata_batch.schema().index_of("project_version").unwrap())).value(i);
                    let description = arrow::array::as_string_array(metadata_batch.column(metadata_batch.schema().index_of("description").unwrap())).value(i);

                    let loc = project_data.get("source_code_analysis")
                        .and_then(|b| b.column(b.schema().index_of("lines_of_code").unwrap()).as_any().downcast_ref::<arrow::array::UInt32Array>())
                        .map_or(0, |arr| arr.value(i));
                    
                    let direct_deps = project_data.get("dependency_analysis")
                        .and_then(|b| b.column(b.schema().index_of("direct_dependencies").unwrap()).as_any().downcast_ref::<arrow::array::UInt32Array>())
                        .map_or(0, |arr| arr.value(i));

                    let has_build_script = project_data.get("build_analysis")
                        .and_then(|b| b.column(b.schema().index_of("has_build_script").unwrap()).as_any().downcast_ref::<arrow::array::BooleanArray>())
                        .map_or(false, |arr| arr.value(i));

                    let downloads = project_data.get("ecosystem_analysis")
                        .and_then(|b| b.column(b.schema().index_of("download_count").unwrap()).as_any().downcast_ref::<arrow::array::UInt64Array>())
                        .map_or(0, |arr| arr.value(i));

                    let commits = project_data.get("version_history")
                        .and_then(|b| b.column(b.schema().index_of("commit_count").unwrap()).as_any().downcast_ref::<arrow::array::UInt32Array>())
                        .map_or(0, |arr| arr.value(i));

                    println!("| {:<14} | {:<7} | {:<41} | {:<6} | {:<11} | {:<12} | {:<9} | {:<7} |", 
                             name, version, description, loc, direct_deps, has_build_script, downloads, commits);
                }
            }
        }
    }

    Ok(())
}
