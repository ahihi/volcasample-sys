#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::ptr;
    
    #[test]
    fn convert_pattern_data() {
        let mut parts = [unsafe { mem::zeroed::<VolcaSample_Part_Data>() }; VOLCASAMPLE_NUM_OF_PART as usize];
        for i in 0..VOLCASAMPLE_NUM_OF_PART as usize {
            parts[i] = VolcaSample_Part_Data {
                SampleNum: i as u16,
                StepOn: 1 << i,
                Accent: 0,
                Reserved: 0,
                Level: 127,
                Param: [
                    // 0 : LEVEL           0~127, (127)
                    100,
                    // 1 : PAN             1~127, 64=Center (64)
                    64,
                    // 2 : SPEED           40~88, 64=Center (64) *changes speed in semitones (FUNC+SPEED operation)
                    //                     129~255, 192=Centre   *changes speed continuously
                    64,
                    // 3 : AMP EG ATTACK   0~127 (0)
                    0,
                    // 4 : AMP EG DECAY    0~127 (127)
                    127,
                    // 5 : PITCH EG INT    1~127, 64=Center (64)
                    64,
                    // 6 : PITCH EG ATTACK 0~127 (0)
                    0,
                    // 7 : PITCH EG DECAY  0~127 (127)
                    127,
                    // 8 : START POINT     0~127 (0)
                    0,
                    // 9 : LENGTH          0~127 (127)
                    127,
                    // 10: HI CUT          0~127 (127)
                    127
                ],
                FuncMemoryPart: 0,
                Padding1: [0; 11],
                Motion: [[0; VOLCASAMPLE_NUM_OF_STEP as usize]; VOLCASAMPLE_NUM_OF_MOTION as usize]
            }
        }
                
        let mut pattern = VolcaSample_Pattern_Data {
            Header: VOLCASAMPLE_PATTERN_HEADER,
            DevCode: VOLCASAMPLE_PATTERN_DEVCODE as u16,
            Reserved: [0; 2],
            ActiveStep: 0b1111_1111_1111_1111,
            Padding1: [0; 22],
            Part: parts,
            Padding2: [0; 28],
            Footer: VOLCASAMPLE_PATTERN_FOOTER
        };
        
        let mut data = SyroData {
            DataType: DataType_Pattern,
            pData: &mut pattern as *mut _ as *mut u8,
            Number: 0,
            Size: mem::size_of::<VolcaSample_Pattern_Data>() as u32,
            Quality: 16,
            Fs: 0,
            SampleEndian: LittleEndian
        };

        let mut handle: SyroHandle = ptr::null_mut();
        let mut n_frames: u32 = 0;
        
        let status = unsafe { SyroVolcaSample_Start(
            &mut handle, 
            &mut data, 
            1,
            0,
            &mut n_frames
        ) };
        assert_eq!(status, Status_Success);
        assert_eq!(n_frames, 138448);
        
        for _ in 0..n_frames {
            let mut left: i16 = 0;
            let mut right: i16 = 0;
            let status = unsafe { SyroVolcaSample_GetSample(handle, &mut left, &mut right) };
            assert_eq!(status, Status_Success);
        }
        
        let status = unsafe { SyroVolcaSample_End(handle) };
        assert_eq!(status, Status_Success);
    }
}
