mod data;

use std::path::Path;
use data::{QR_DATA_LENGTHS};
use qrcode::{QrCode, Version, EcLevel, types::QrError};
use image::Luma;

const DEFAULT_QR_VERSION: Version = Version::Normal(40);
const DEFAULT_EC_LEVEL: EcLevel = EcLevel::L;
const DEFAULT_SLACK: usize = 144;

pub struct MultiQrCode {
    pub codes: Vec<QrCode>,
}

impl MultiQrCode {
    pub fn new<D: AsRef<[u8]>>(data: D, version: Version, ec: EcLevel) -> Result<Self, QrError> {
        let mut res: Vec<QrCode> = Vec::new();

        // calculate sizes

        let data_size = data.as_ref().len() * 8;
        let qr_size_total = QR_DATA_LENGTHS[match version {
            Version::Normal(x) => x-1,
            Version::Micro(x) => x+39
        } as usize][ec as usize];
        
        let qr_size_data = qr_size_total - 8 - DEFAULT_SLACK;
        let qr_count: usize = (data_size as f32 / qr_size_data as f32).ceil() as usize;

        // create new qr codes for data along with index, add to res

        let step = qr_size_data / 8;
        let remaining = (data_size % qr_size_data) / 8;

        let mut qr_data: Vec<u8> = Vec::with_capacity(qr_size_total / 8);
        for i in 0..qr_count {
            qr_data.clear();
            qr_data.push(i as u8);
            let slice_start = i * step;
            let slice_end = if i != qr_count-1 { slice_start + step } else { slice_start + remaining };
            qr_data.extend_from_slice(&data.as_ref()[slice_start..slice_end]);

            match QrCode::with_version(&qr_data, version, ec) {
                Ok(x) => res.push(x),
                Err(x) => return Err(x)
            }
        }

        // return multiple qr codes

        Ok(MultiQrCode {
            codes: res
        })
    }

    pub fn default<D: AsRef<[u8]>>(data: D) -> Result<Self, QrError> {
        Self::new(data, DEFAULT_QR_VERSION, DEFAULT_EC_LEVEL)
    }

    pub fn save(&self, path: &str) {
        let path = Path::new(path);
        for (i, code) in self.codes.iter().enumerate() {
            code.render::<Luma<u8>>().build().save(path.with_extension(format!("{}.png", i))).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_lorem_ipsum() {
        const LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer tristique turpis eros, vel hendrerit purus laoreet vitae. Praesent consectetur non mauris quis fermentum. Vivamus vulputate mi sed felis mollis dapibus. Interdum et malesuada fames ac ante ipsum primis in faucibus. Quisque vel tortor vitae ex blandit sodales in id diam. Morbi urna quam, lobortis id tristique a, imperdiet ac neque. Vivamus metus enim, posuere sed imperdiet in, tempor in purus. Nunc molestie, eros at dignissim commodo, ex eros ultrices augue, nec tristique turpis neque at ipsum. Phasellus sed malesuada justo. Sed lacus tellus, sodales quis pellentesque vitae, sagittis id augue. Quisque sit amet suscipit enim, in luctus lorem. Curabitur condimentum augue ac ornare mattis. Donec tempor pretium maximus.

        Cras tempus lacinia sagittis. Vestibulum suscipit nisi sit amet ipsum hendrerit, sit amet iaculis mauris auctor. Vestibulum dictum felis nec risus egestas vulputate. Donec sit amet mauris a mauris venenatis suscipit. Vivamus elementum faucibus ornare. Interdum et malesuada fames ac ante ipsum primis in faucibus. Suspendisse ornare tellus id nisl condimentum condimentum. Suspendisse placerat non nisi sit amet tempor. Curabitur ante lacus, tempor in vulputate eget, vestibulum ut arcu. Sed quam massa, mattis a efficitur cursus, vehicula a purus. Aenean leo justo, malesuada et massa condimentum, pellentesque fermentum lectus.
        
        Aliquam rhoncus augue vitae metus congue hendrerit. Nulla tempus turpis urna, sed feugiat nisi vestibulum vitae. Suspendisse at bibendum urna. Phasellus malesuada urna eu imperdiet semper. Morbi rhoncus felis ac ex vestibulum, quis semper eros suscipit. Donec in dignissim eros. In ultrices fermentum purus vitae tempus. Morbi vel ipsum at est laoreet commodo. Morbi iaculis, enim sed iaculis vestibulum, diam arcu aliquet diam, nec eleifend nisl dolor in nunc. Sed dictum elit eget hendrerit feugiat. Donec feugiat lectus at est dignissim, ut ullamcorper dui euismod. Nunc commodo, quam ac efficitur rhoncus, dolor arcu volutpat felis, sed auctor leo neque eget enim. Donec lobortis at orci eu consectetur.
        
        Cras convallis neque vitae nisl ullamcorper, a dictum eros ultricies. Etiam purus arcu, faucibus egestas mollis ac, pulvinar ut dolor. Suspendisse potenti. Vivamus id elementum est. Donec vel ornare nulla. Sed laoreet maximus sodales. Nam ullamcorper molestie ipsum quis ultrices. Phasellus aliquam nisi vel iaculis convallis. Quisque volutpat libero nec ipsum rutrum, vel fermentum augue eleifend. Proin vitae libero eu lectus convallis sodales. Etiam posuere nulla sit amet elit viverra placerat. Praesent quis risus in eros fermentum tincidunt. Sed ipsum elit, tempor et nisi sed, bibendum efficitur est. Nullam ultricies arcu mi, at maximus dui mollis nec. Fusce mauris nisl, porttitor ut lacus at, malesuada varius arcu. Nulla facilisi.
        
        Phasellus consequat dictum eros, sit amet lobortis orci volutpat in. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nullam auctor bibendum venenatis. Cras est libero, feugiat sit amet massa id, semper lacinia velit. Phasellus justo augue, consequat ut dictum ut, gravida nec neque. Duis iaculis nisi vitae augue pellentesque ornare. Curabitur ullamcorper a urna ut lobortis. Praesent fermentum mattis facilisis. In hac habitasse platea dictumst. Vivamus nec lorem ullamcorper, fringilla tellus sit amet, facilisis nibh. Suspendisse id metus elementum, dignissim risus ac, blandit risus. Cras non ex at augue varius rhoncus. Pellentesque at ex nibh. Cras porttitor lobortis imperdiet. Vestibulum quis euismod turpis.";

        let qr = MultiQrCode::default(LIPSUM).unwrap();
        qr.save("./test.png");

        assert!(true);
    }
}