//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1267/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1267<F: Float>(t10786: F, t19703: F, t10809: F, t17964: F, t63899: F, t63901: F, t63903: F, t63905: F, t63908: F, t63909: F, t63911: F, t63914: F, t63918: F, t63921: F, t3671: F, t61033: F) -> (F, F) {
    let t63923 = t19703 * t10786;
    let t63925 = t17964 * t10809;
    let t63927 = -t63899 / 768.0 - t63901 / 1536.0 + t63903 / 192.0 + t63905 / 384.0 - t63908 + t63909 / 384.0 + t63911 / 192.0 - t63914 - t63918 - t63921 / 256.0 + t63923 / 256.0 - t63925 / 1536.0;
    let t63928 = t61033 * t3671;
    (t63927, t63928)
}
