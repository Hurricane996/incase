use std::error::Error;

use actix_cors::Cors;
use actix_web::{Responder, web::{self, Data}, dev::HttpServiceFactory, get};
use common::Database;

struct AppData {
    address: &'static str,
    area: &'static str,
    connection_string: String
}

pub fn video_service(address: &'static str, area: &'static str, connection_string: String) -> impl HttpServiceFactory {
    // we want any application with the correct token to be able to play our video, so allow all cors for here
    // on other routes we want to be much more restrictive
    let cors = Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allowed_methods(["GET"]);
    web::scope(area)
        .wrap(cors)
        .service(get_playlist)
        .service(get_video_segment)
        .app_data(Data::new(AppData {
            address,
            area,
            connection_string
        }))
}

#[get("{username}/playlist.m3u8")]
async fn get_playlist(username: web::Path<String>,data: web::Data<AppData>) -> impl Responder {
    let mut client = Database::new(&data.connection_string).await?;
    let sequences = client.get_sequence_numbers(username.as_str()).await?;

    const HEADER: &str =
"#EXTM3U
#EXT-X-PLAYLIST-TYPE:VOD
#EXT-X-TARGETDURATION:3
#EXT-X-VERSION:4
#EXT-X-MEDIA-SEQUENCE:0";

    let body = sequences.iter()
        .map(|x| format!("#EXTINF:\nhttp://{}/{}/{}/{x}.ts\n", data.address, data.area, username.as_str()))
        .collect::<String>();

    Ok::<_,Box<dyn Error>>(format!("{HEADER}\n{body}\n#EXT-X-ENDLIST"))
}

#[get("{username}/{sequence}.ts")]
async fn get_video_segment(data: web::Data<AppData>, path: web::Path<(String,i32)>) -> impl Responder {
    let mut client = Database::new(&data.connection_string).await?;

    let result = client.get_video_clip(path.0.as_str(), path.1).await.expect("Failed to get video clip");

    Ok::<_, Box<dyn Error>>(result)

}