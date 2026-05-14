//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 955/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk955<F: Float>(t10077: F, t1642: F, t10160: F, t1630: F, t125: F, t4459: F, t10117: F, t4473: F, t3256: F, t339: F, t790: F, t4419: F, t10086: F, t236: F, t3267: F, t4462: F) -> (F, F, F, F, F, F, F) {
    let t12846 = t10077 * t1642;
    let t12861 = t10160 * t1630;
    let t12863 = t125 * t4459;
    let t12881 = 7.0 / 576.0 * t10117 * t4473;
    let t12887 = t339 * t3256 * t790;
    let t12889 = 7.0 / 1152.0 * t12887 * t4419;
    let t12891 = t339 * t10086 * t236;
    let t12902 = 7.0 / 2304.0 * t3267 * t4462;
    (t12846, t12861, t12863, t12881, t12889, t12891, t12902)
}
