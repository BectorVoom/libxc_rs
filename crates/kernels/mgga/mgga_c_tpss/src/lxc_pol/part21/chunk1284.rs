//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1284/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1284<F: Float>(t63928: F, t10602: F, t17964: F, t10575: F, t61051: F, t10606: F, t10813: F, t10790: F, t19703: F, t1381: F, t61050: F, t61025: F, t61034: F, t61054: F, t61058: F, t61060: F) -> (F,) {
    let t63929 = 7.0 / 1152.0 * t63928;
    let t63930 = t17964 * t10602;
    let t63932 = t17964 * t10575;
    let t63935 = 119.0 / 3456.0 * t61051;
    let t63939 = t17964 * t10606;
    let t63941 = t17964 * t10813;
    let t63943 = t19703 * t10790;
    let t63945 = t61050 * t1381;
    let t63947 = t63929 + t63930 / 192.0 - t61025 - 5.0 / 192.0 * t63932 - 7.0 / 288.0 * t61034 - t63935 + 7.0 / 2304.0 * t61054 - 7.0 / 1152.0 * t61058 + 7.0 / 2304.0 * t61060 + t63939 / 384.0 - 5.0 / 384.0 * t63941 - t63943 / 192.0 - 119.0 / 6912.0 * t63945;
    (t63947,)
}
