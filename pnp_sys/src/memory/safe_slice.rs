use core::cmp;

fn get_safe_slice_bounds(slice: &[u8], offset: usize, size: usize) -> (usize, usize) {
    let diff = slice.len().saturating_sub(offset);

    if diff == 0 {
        return (0, 0);
    }

    let len = cmp::min(diff, size);
    (offset, offset + len)
}

pub fn slice(slice: &[u8], offset: usize, size: usize) -> &[u8] {
    let (lower, upper) = get_safe_slice_bounds(slice, offset, size);
    &slice[lower..upper]
}

pub fn slice_mut(slice: &mut [u8], offset: usize, size: usize) -> &mut [u8] {
    let (lower, upper) = get_safe_slice_bounds(slice, offset, size);
    &mut slice[lower..upper]
}

#[cfg(test)]
mod test {
    use super::*;

    mod slice_bounds {
        use super::*;

        #[test]
        fn should_get_slice() {
            let result = get_safe_slice_bounds(&[1, 2, 3, 4], 1, 2);
            assert_eq!(result, (1, 3));
        }

        #[test]
        fn should_handle_size_0() {
            let result = get_safe_slice_bounds(&[1, 2, 3, 4], 1, 0);
            assert_eq!(result, (1, 1));
        }

        #[test]
        fn should_handle_too_large_of_size() {
            let result = get_safe_slice_bounds(&[1, 2, 3, 4], 0, 10);
            assert_eq!(result, (0, 4));
        }

        #[test]
        fn should_handle_too_high_of_offset_for_size() {
            let result = get_safe_slice_bounds(&[1, 2, 3, 4], 1, 10);
            assert_eq!(result, (1, 4));
        }

        #[test]
        fn should_handle_out_of_bounds_offset() {
            let result = get_safe_slice_bounds(&[1, 2, 3, 4], 10, 10);
            assert_eq!(result, (0, 0));
        }
    }

    mod slice {
        use super::*;

        #[test]
        fn should_get_slice() {
            let result = slice(&[1, 2, 3, 4], 1, 2);
            assert_eq!(result, [2, 3]);
        }

        #[test]
        fn should_handle_size_0() {
            let result = slice(&[1, 2, 3, 4], 1, 0);
            assert_eq!(result, []);
        }

        #[test]
        fn should_handle_too_large_of_size() {
            let result = slice(&[1, 2, 3, 4], 0, 10);
            assert_eq!(result, [1, 2, 3, 4]);
        }

        #[test]
        fn should_handle_too_high_of_offset_for_size() {
            let result = slice(&[1, 2, 3, 4], 1, 10);
            assert_eq!(result, [2, 3, 4]);
        }

        #[test]
        fn should_handle_out_of_bounds_offset() {
            let result = slice(&[1, 2, 3, 4], 10, 10);
            assert_eq!(result, []);
        }
    }

    mod slice_mut {
        use super::*;

        #[test]
        fn should_get_slice() {
            let slice = &mut [1, 2, 3, 4];
            let result = slice_mut(slice, 1, 2);
            assert_eq!(result, [2, 3]);
        }

        #[test]
        fn should_handle_size_0() {
            let slice = &mut [1, 2, 3, 4];
            let result = slice_mut(slice, 1, 0);
            assert_eq!(result, []);
        }

        #[test]
        fn should_handle_too_large_of_size() {
            let slice = &mut [1, 2, 3, 4];
            let result = slice_mut(slice, 0, 10);
            assert_eq!(result, [1, 2, 3, 4]);
        }

        #[test]
        fn should_handle_too_high_of_offset_for_size() {
            let slice = &mut [1, 2, 3, 4];
            let result = slice_mut(slice, 1, 10);
            assert_eq!(result, [2, 3, 4]);
        }

        #[test]
        fn should_handle_out_of_bounds_offset() {
            let slice = &mut [1, 2, 3, 4];
            let result = slice_mut(slice, 10, 10);
            assert_eq!(result, []);
        }
    }
}
