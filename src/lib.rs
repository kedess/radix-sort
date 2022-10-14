use num_traits::{FromPrimitive, PrimInt, ToPrimitive, Unsigned};

pub fn lsd_sort<T>(src: &mut [T]) where T: Unsigned + Copy + ToPrimitive + PrimInt + FromPrimitive {
    let mut dst = src.to_vec();
    let mut cnt = vec![0usize; 256];
    let mut cat = 0u32;
    for _ in 0..std::mem::size_of::<T>() {
        for value in dst.iter() {
            let val = value.unsigned_shr(cat) & FromPrimitive::from_u32(0xFF).unwrap();
            cnt[val.to_usize().unwrap()] += 1;
        }
        for idx in 1..256 {
            cnt[idx] += cnt[idx - 1];
        }
        let mut tmp = vec![T::zero(); dst.len()];
        for value in dst.iter().rev() {
            let val = value.unsigned_shr(cat) & FromPrimitive::from_u32(0xFF).unwrap();
            let idx = val.to_usize().unwrap();
            tmp[cnt[idx] - 1] = *value;
            cnt[idx] -= 1;
        }
        dst = tmp;
        cnt.fill(0);
        cat += 8;
    }
    src.copy_from_slice(&dst[..src.len()]);
}
