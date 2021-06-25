mod base;
mod error;
mod downloader;
mod addons_manager;

use error::*;
use downloader::*;
use std::env::current_dir;
use std::path::Path;

// metal check - поиск по аддонам из json файла + проверка что из этого уже установлено из lock
// файла и показ что можно обновить
// metal update - то же самое что и check, но потом 
//   - 1 сбор аддонов который можно обновить
//   - 2 поиск по гитхабу ссылок
//   - 3 загрузка на файловую систему
//   - 4 unzip
//   - 5 обновление lock файла
//   - 6 проверить на неактивные аддоны и удалить в папке с игрой
//   - 7 удалить все unzip файлы и файловую сткруктуру
//metal clear - сопоставление lock и json файлов и поиск тех аддонов что больше не находятся в json
//файле. Все такие аддоны считаются удаленным и должны быть удалены из lock файла и из папки с
//игрой
//metal add <url> - добавить новый аддон
//metal remove url? repo? - удалить аддон

#[tokio::main]
async fn main() -> MetalResult<()> {
    let http_client = reqwest::Client::new();
    let home_dir = Path::new(&dirs::home_dir().unwrap_or(current_dir()?)).join(".metal");
    download_assets(&http_client, "WeakAuras", "WeakAuras2", &home_dir).await?;
    Ok(())
}
