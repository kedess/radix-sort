pub fn lsd_sort_u64(src: &mut[u64]) {
    let mut dst = src.to_vec();
    let mut cat = 0u64;
    let mut cnt = vec![0usize; 256];
    for _ in 0..8 {
        for value in dst.iter() {
            cnt [((value >> cat) & 0xFF) as usize] += 1;
        }
        for idx in 1..256 {
            cnt[idx] += cnt[idx - 1];
        }
        let mut tmp = vec![0u64;dst.len()];
        for value in dst.iter().rev() {
            let idx = ((value >> cat) & 0xFF) as usize;
            tmp[cnt[idx] - 1] = *value;
            cnt[idx] -= 1;
        }
        dst = tmp;
        cnt.fill(0);
        cat += 8;
    }
    for idx in 0..src.len() {
        src[idx] = dst[idx];
    }
}

pub fn lsd_sort_u32(src: &mut[u32]) {
    let mut dst = src.to_vec();
    let mut cat = 0u32;
    let mut cnt = vec![0usize; 256];
    for _ in 0..4 {
        for value in dst.iter() {
            cnt[((value >> cat) & 0xFF) as usize] += 1;
        }
        for idx in 1..256 {
            cnt[idx] += cnt[idx - 1];
        }
        let mut tmp = vec![0u32;dst.len()];
        for value in dst.iter().rev() {
            let idx = ((value >> cat) & 0xFF) as usize;
            tmp[cnt[idx] - 1] = *value;
            cnt[idx] -= 1;
        }
        dst = tmp;
        cnt.fill(0);
        cat += 8;
    }
    for idx in 0..src.len() {
        src[idx] = dst[idx];
    }
}

pub fn lsd_sort_u16(src: &mut[u16]) {
    let mut dst = src.to_vec();
    let mut cat = 0u16;
    let mut cnt = vec![0usize; 256];
    for _ in 0..2 {
        for value in dst.iter() {
            cnt [((value >> cat) & 0xFF) as usize] += 1;
        }
        for idx in 1..256 {
            cnt[idx] += cnt[idx - 1];
        }
        let mut tmp = vec![0u16;dst.len()];
        for value in dst.iter().rev() {
            let idx = ((value >> cat) & 0xFF) as usize;
            tmp[cnt[idx] - 1] = *value;
            cnt[idx] -= 1;
        }
        dst = tmp;
        cnt.fill(0);
        cat += 8;
    }
    for idx in 0..src.len() {
        src[idx] = dst[idx];
    }
}
