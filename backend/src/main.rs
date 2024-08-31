use warp::Filter;
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3, ListObjectsV2Request};
use serde::Serialize;
use std::convert::Infallible;
use log::{info, error};
use env_logger;

#[derive(Serialize)]
struct FileList {
    files: Vec<String>,
}

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    // Define the API route
    let list_files_route = warp::path("list-csv-files")
        .and(warp::get())
        .and_then(list_csv_files_handler);

    // Start the warp server on localhost:3030
    info!("Starting server on http://127.0.0.1:3030");
    warp::serve(list_files_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Handler for listing CSV files
async fn list_csv_files_handler() -> Result<Box<dyn warp::Reply>, Infallible> {
    match list_csv_files().await {
        Ok(files) => Ok(Box::new(warp::reply::json(&files))),
        Err(e) => {
            error!("Failed to list files: {}", e);
            let json_response = warp::reply::json(&serde_json::json!({
                "error": "Failed to list files"
            }));
            Ok(Box::new(warp::reply::with_status(json_response, warp::http::StatusCode::INTERNAL_SERVER_ERROR)))
        }
    }
}

// Function to list CSV files from S3
async fn list_csv_files() -> Result<FileList, Box<dyn std::error::Error>> {
    let client = S3Client::new(Region::UsEast1);  // Change the region as necessary
    let bucket_name = "your-bucket-name";
    let prefix = "csv_files/";  // Specify the folder in your bucket where CSV files are stored

    let request = ListObjectsV2Request {
        bucket: bucket_name.to_string(),
        prefix: Some(prefix.to_string()),
        ..Default::default()
    };

    let result = client.list_objects_v2(request).await?;
    let files = result.contents
        .unwrap_or_default()
        .into_iter()
        .filter_map(|obj| obj.key)
        .filter(|key| key.ends_with(".csv"))
        .collect::<Vec<String>>();

    info!("Found {} CSV files", files.len());
    Ok(FileList { files })
}
