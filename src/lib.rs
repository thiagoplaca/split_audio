use std::process::{Command, Stdio};
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_url() {
        let cases = [
            ("https://youtu.be", Ok(true)),
            ("https://www.youtu.be", Ok(true)),
            ("https://youtube.com", Ok(true)),
            ("https://www.youtube.com", Ok(true)),
        ];
            for (url, expected) in cases {
                assert_eq!(is_youtube(url), expected, "Falhou para a url: {}", url );
            }
    }
}

pub fn download_audio_youtube(url: &str) -> Result<String, Box<dyn Error>> {
    println!("Baixando o áudio...");

    let output_template = "%(title)s.%(ext)s";

    let output = Command::new("yt-dlp")
        .args([
            "-x", "--audio-format","mp3",
            "-o", output_template,
            "--no-overwrites",
            "--extractor-args", "youtube:player_client=ios,android",
            "--print", "after_move:filepath", url])
        .output()?;

     if output.status.success() {
         let file = String::from_utf8_lossy(&output.stdout).trim().to_string();
         Ok(file)
     } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Erro no download com yt-dlp: {}", stderr).into())
     } 
}

pub fn audio_separator(audio_file: &str) -> Result<(), Box<dyn Error>>  {
    println!("Iniciando separação do arquivo: {}", audio_file);

    let mut child = Command::new("audio-separator")
        .args([audio_file, "--model_filename", "UVR-MDX-NET-Inst_HQ_3.onnx"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;

    if status.success() {
        println!("O Áudio foi separado com successo!");
        Ok(())
    } else {
        Err(format!("O audio-separator finalizou com erro (código: {:?})", status.code()).into())
    }
}

pub fn is_youtube(url: &str) -> Result<bool, &'static str> {

    let valid_youtube_urls = [
        "https://youtu.be",
        "https://www.youtu.be",
        "https://youtube.com",
        "https://www.youtube.com",
    ];

    let verify_youtube_url = valid_youtube_urls.iter().any(|&domain| url.contains(domain));

    if !verify_youtube_url {
        return Err("invalid url");
    } else {
        Ok(verify_youtube_url)
    }
}
