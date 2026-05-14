//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1128/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1128<F: Float>(t28: F, t15952: F, t15955: F, t15956: F, t16: F, t3231: F, t3673: F, t5142: F, t5145: F, t517: F, t584: F, t157: F, t15951: F, t182: F, t1787: F, t2516: F, t17: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t15966 = piecewise3(t29, 0.0, -8.0 / 27.0 * t15952 * t3673 - 16.0 / 9.0 * t15955 * t15956 + 4.0 / 9.0 * t5142 * t3231 - 8.0 / 3.0 * t517 * t584 + 8.0 * t5145 * t16);
    let t15968 = (t15951 + t15966) * t157;
    let t15970 = 0.19751673498613801407e-1 * t15968 * t182;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    (t15968, t15970, t15972)
}
