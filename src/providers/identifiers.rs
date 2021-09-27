#[derive(PartialEq, Clone)]
/// The source provider
pub enum Provider {
    QQ,
    Kugou,
    Kuwo,
    Migu,
    Joox,
    Youtube,
    YoutubeDL,
    Bilibili,
    Pyncmd,
}

impl std::str::FromStr for Provider {
    type Err = String;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "qq" => Ok(Provider::QQ),
            "kugou" => Ok(Provider::Kugou),
            "kuwo" => Ok(Provider::Kuwo),
            "migu" => Ok(Provider::Migu),
            "joox" => Ok(Provider::Joox),
            "youtube" => Ok(Provider::Youtube),
            "youtubedl" => Ok(Provider::YoutubeDL),
            "bilibili" => Ok(Provider::Bilibili),
            "pyncmd" => Ok(Provider::Pyncmd),
            _ => Err(src.to_string()),
        }
    }
}

impl std::fmt::Debug for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Provider::QQ => "qq",
                Provider::Kugou => "kugou",
                Provider::Kuwo => "kuwo",
                Provider::Migu => "migu",
                Provider::Joox => "joox",
                Provider::Youtube => "youtube",
                Provider::YoutubeDL => "youtubedl",
                Provider::Bilibili => "bilibili",
                Provider::Pyncmd => "pyncmd",
            }
        )
    }
}
