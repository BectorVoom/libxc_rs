//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 925/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk925<F: Float>(t8956: F, t938: F, t941: F, t2655: F, t357: F, t339: F, t349: F, t2677: F, t2682: F, t2719: F, t956: F, t2713: F, t2716: F, t2751: F, t774: F, t2742: F) -> (F, F, F, F, F, F, F) {
    let t8958 = t938 * t941 * t8956;
    let t8961 = t2655 * t357;
    let t8963 = t339 * t349 * t8961;
    let t8966 = t2682 * t2677;
    let t8970 = t956 * t2719;
    let t8972 = t2713 * t2716 * t8970;
    let t8976 = t2713 * t941 * t8970;
    let t8983 = t774 * t2751;
    let t8984 = t8983 * t2742;
    (t8958, t8963, t8966, t8972, t8976, t8983, t8984)
}
