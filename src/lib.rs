use std::{path::Path};
use base64::encode;
use qrcode::{QrCode, Version, EcLevel, types::QrError, Color};
use image::Luma;
use serde::{Serialize, Deserialize};

pub const DEFAULT_QR_VERSION: Version = Version::Normal(40);
pub const DEFAULT_EC_LEVEL: EcLevel = EcLevel::L;

#[derive(Serialize, Deserialize)]
pub enum QrData {
    Base64 {
        width: usize,
        data: String
    },
    String (String)
}

impl QrData {
    pub fn base64_from(code: &QrCode) -> Self {
        let width = code.width();
        let mut bytes = vec![0u8; ((width * width) as f32 / 8.0).ceil() as usize];
        for (i, color) in code.to_colors().iter().enumerate() {
            let byte_index = i / 8;
            let shift = 7 - i%8;
            if let Color::Dark = color {
                bytes[byte_index] |= 1u8 << shift;
            }
        }
        QrData::Base64 { width, data: encode(bytes) }
    }
}

pub struct MultiQrCode {
    pub codes: Vec<QrCode>
}

impl MultiQrCode {
    pub fn new<D: AsRef<[u8]>>(data: D, version: Version, ec: EcLevel) -> Result<Self, QrError> {
        Self::with_slack(data, version, ec, QR_VERSION_SLACK[version.to_index()])
    }

    pub fn with_slack<D: AsRef<[u8]>>(data: D, version: Version, ec: EcLevel, slack: usize) -> Result<Self, QrError> {
        let mut res: Vec<QrCode> = Vec::new();
        let data = data.as_ref();

        // fail if version is Micro (unsupported)
        if let Version::Micro(_) = version {
            return Err(QrError::InvalidVersion)
        }

        // calculate sizes
        let qr_size_total = QR_DATA_LENGTHS[version.to_index()][ec as usize];
        let qr_size_data = qr_size_total - (1 + slack);

        // create new qr codes for indexed data, add to res
        let mut qr_data: Vec<u8> = vec!(0; 1+qr_size_data);
        for (i, part) in data.chunks(qr_size_data).into_iter().enumerate() {
            qr_data[0] = i as u8;
            qr_data[1..1+part.len()].clone_from_slice(part);

            res.push(QrCode::with_version(&qr_data[0..1+part.len()], version, ec)?);
        }

        Ok(MultiQrCode {
            codes: res
        })
    }

    pub fn default<D: AsRef<[u8]>>(data: D) -> Result<Self, QrError> {
        Self::new(data, DEFAULT_QR_VERSION, DEFAULT_EC_LEVEL)
    }

    pub fn to_strings(&self) -> Vec<QrData> {
        self.codes.iter().map(|code| QrData::String(code.render().light_color(' ').dark_color('#').build().to_string())).collect()
    }

    pub fn to_base64(&self) -> Vec<QrData> {
        self.codes.iter().map(|code| QrData::base64_from(code)).collect()
    }

    pub fn save(&self, path: &str) {
        let path = Path::new(path);
        for (i, code) in self.codes.iter().enumerate() {
            code.render::<Luma<u8>>().build().save(path.with_extension(format!("{}.png", i))).unwrap();
        }
    }
} impl ToString for MultiQrCode {
    fn to_string(&self) -> String {
        let strings = self.to_strings();
        let mut string = String::new();
        for qr_data in strings {
            if let QrData::String(x) = qr_data { string.push_str(&x) }
        }
        string
    }
}

// simple addition to Version to support easy conversion to index number on tables
trait ToIndex { 
    fn to_index(&self) -> usize; 
} impl ToIndex for Version {
    fn to_index(&self) -> usize {
        (match self {
            Version::Normal(x) => x-1,
            Version::Micro(x) => x+39
        }) as usize
    }
} 

#[cfg(test)]
mod tests {
    use more_asserts::assert_le;

    use super::*;

    const LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer tristique turpis eros, vel hendrerit purus laoreet vitae. Praesent consectetur non mauris quis fermentum. Vivamus vulputate mi sed felis mollis dapibus. Interdum et malesuada fames ac ante ipsum primis in faucibus. Quisque vel tortor vitae ex blandit sodales in id diam. Morbi urna quam, lobortis id tristique a, imperdiet ac neque. Vivamus metus enim, posuere sed imperdiet in, tempor in purus. Nunc molestie, eros at dignissim commodo, ex eros ultrices augue, nec tristique turpis neque at ipsum. Phasellus sed malesuada justo. Sed lacus tellus, sodales quis pellentesque vitae, sagittis id augue. Quisque sit amet suscipit enim, in luctus lorem. Curabitur condimentum augue ac ornare mattis. Donec tempor pretium maximus.

    Cras tempus lacinia sagittis. Vestibulum suscipit nisi sit amet ipsum hendrerit, sit amet iaculis mauris auctor. Vestibulum dictum felis nec risus egestas vulputate. Donec sit amet mauris a mauris venenatis suscipit. Vivamus elementum faucibus ornare. Interdum et malesuada fames ac ante ipsum primis in faucibus. Suspendisse ornare tellus id nisl condimentum condimentum. Suspendisse placerat non nisi sit amet tempor. Curabitur ante lacus, tempor in vulputate eget, vestibulum ut arcu. Sed quam massa, mattis a efficitur cursus, vehicula a purus. Aenean leo justo, malesuada et massa condimentum, pellentesque fermentum lectus.
    
    Aliquam rhoncus augue vitae metus congue hendrerit. Nulla tempus turpis urna, sed feugiat nisi vestibulum vitae. Suspendisse at bibendum urna. Phasellus malesuada urna eu imperdiet semper. Morbi rhoncus felis ac ex vestibulum, quis semper eros suscipit. Donec in dignissim eros. In ultrices fermentum purus vitae tempus. Morbi vel ipsum at est laoreet commodo. Morbi iaculis, enim sed iaculis vestibulum, diam arcu aliquet diam, nec eleifend nisl dolor in nunc. Sed dictum elit eget hendrerit feugiat. Donec feugiat lectus at est dignissim, ut ullamcorper dui euismod. Nunc commodo, quam ac efficitur rhoncus, dolor arcu volutpat felis, sed auctor leo neque eget enim. Donec lobortis at orci eu consectetur.
    
    Cras convallis neque vitae nisl ullamcorper, a dictum eros ultricies. Etiam purus arcu, faucibus egestas mollis ac, pulvinar ut dolor. Suspendisse potenti. Vivamus id elementum est. Donec vel ornare nulla. Sed laoreet maximus sodales. Nam ullamcorper molestie ipsum quis ultrices. Phasellus aliquam nisi vel iaculis convallis. Quisque volutpat libero nec ipsum rutrum, vel fermentum augue eleifend. Proin vitae libero eu lectus convallis sodales. Etiam posuere nulla sit amet elit viverra placerat. Praesent quis risus in eros fermentum tincidunt. Sed ipsum elit, tempor et nisi sed, bibendum efficitur est. Nullam ultricies arcu mi, at maximus dui mollis nec. Fusce mauris nisl, porttitor ut lacus at, malesuada varius arcu. Nulla facilisi.
    
    Phasellus consequat dictum eros, sit amet lobortis orci volutpat in. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nullam auctor bibendum venenatis. Cras est libero, feugiat sit amet massa id, semper lacinia velit. Phasellus justo augue, consequat ut dictum ut, gravida nec neque. Duis iaculis nisi vitae augue pellentesque ornare. Curabitur ullamcorper a urna ut lobortis. Praesent fermentum mattis facilisis. In hac habitasse platea dictumst. Vivamus nec lorem ullamcorper, fringilla tellus sit amet, facilisis nibh. Suspendisse id metus elementum, dignissim risus ac, blandit risus. Cras non ex at augue varius rhoncus. Pellentesque at ex nibh. Cras porttitor lobortis imperdiet. Vestibulum quis euismod turpis.";

    #[test]
    #[ignore]
    fn confirm_minimum_slack() {
        for version in 1_usize..41_usize {
            let mut slack: usize = 0;
            loop {
                match MultiQrCode::with_slack(LIPSUM, Version::Normal(version as i16), DEFAULT_EC_LEVEL, slack) {
                    Ok(_) => {
                        println!("Minimum slack for version {} is {}", version, slack);
                        assert_le!(slack, QR_VERSION_SLACK[version-1], "Invalid slack for version {}: {} > {}", version, slack, QR_VERSION_SLACK[version-1]);
                        break;
                    }
                    _ => {
                        slack += 1;
                        continue;
                    }
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn compare_string_vs_base64() {
        let qr = MultiQrCode::default("Hello world!").unwrap();
        let string = serde_json::to_string(&qr.to_strings()).unwrap();
        let base64 = serde_json::to_string(&qr.to_base64()).unwrap();
        println!("String:\n{}\nBase64:\n{}",string,base64);
        assert_le!(base64.len(), string.len());
    }

    #[test]
    fn print_hello() {
        let qr = MultiQrCode::new("Hello world!", Version::Normal(10), EcLevel::L).unwrap();
        println!("{}", qr.to_string());
    }

    #[test]
    fn save_hello() {
        let qr = MultiQrCode::default("Hello world!").unwrap();
        qr.save("./test-hw.png");
    }

    #[test]
    fn print_lipsum() {
        let qr = MultiQrCode::new(LIPSUM, Version::Normal(10), EcLevel::L).unwrap();
        println!("{}", qr.to_string());
    }

    #[test]
    fn save_lipsum() {
        let qr = MultiQrCode::default(LIPSUM).unwrap();
        qr.save("./test-lipsum.png");
    }
}

// This table is from <ISO/IEC 18004:2006 ??6.4.10, Table 7> but converted into bytes and with no Micro version.
pub const QR_DATA_LENGTHS: [[usize; 4]; 40] = [
    [19, 16, 13, 9],
    [34, 28, 22, 16],
    [55, 44, 34, 26],
    [80, 64, 48, 36],
    [108, 86, 62, 46],
    [136, 108, 76, 60],
    [156, 124, 88, 66],
    [194, 154, 110, 86],
    [232, 182, 132, 100],
    [274, 216, 154, 122],
    [324, 254, 180, 140],
    [370, 290, 206, 158],
    [428, 334, 244, 180],
    [461, 365, 261, 197],
    [523, 415, 295, 223],
    [589, 453, 325, 253],
    [647, 507, 367, 283],
    [721, 563, 397, 313],
    [795, 627, 445, 341],
    [861, 669, 485, 385],
    [932, 714, 512, 406],
    [1006, 782, 568, 442],
    [1094, 860, 614, 464],
    [1174, 914, 664, 514],
    [1276, 1000, 718, 538],
    [1370, 1062, 754, 596],
    [1468, 1128, 808, 628],
    [1531, 1193, 871, 661],
    [1631, 1267, 911, 701],
    [1735, 1373, 985, 745],
    [1843, 1455, 1033, 793],
    [1955, 1541, 1115, 845],
    [2071, 1631, 1171, 901],
    [2191, 1725, 1231, 961],
    [2306, 1812, 1286, 986],
    [2434, 1914, 1354, 1054],
    [2566, 1992, 1426, 1096],
    [2702, 2102, 1502, 1142],
    [2812, 2216, 1582, 1222],
    [2956, 2334, 1666, 1276]
];

pub const QR_VERSION_SLACK: [usize; 40] = [
    // 2 slack for version 1-9
    2, 2, 2, 2, 2, 2, 2, 2, 2,
    // 3 slack for version 10-40 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3
];