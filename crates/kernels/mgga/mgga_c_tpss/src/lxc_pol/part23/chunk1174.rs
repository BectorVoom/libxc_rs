//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1174/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1174<F: Float>(t19150: F, t6034: F, t3073: F, t342: F, t450: F, t6032: F, t19145: F, t1885: F, t19103: F, t452: F, t1149: F, t1884: F, t1887: F, t19104: F, t19106: F, t19113: F, t19115: F, t19118: F, t19125: F, t19129: F, t19131: F, t19135: F, t19139: F, t19143: F, t19147: F, t3120: F, t3145: F, t473: F, t6019: F, t6022: F, t6024: F, t6027: F, t6031: F, t6035: F, t6038: F) -> (F, F, F, F, F, F, F) {
    let t19151 = t19150 * t6034;
    let t19155 = t3073 * t342 * t450;
    let t19156 = t6032 * t19155;
    let t19158 = t19145 * t450;
    let t19159 = t6032 * t19158;
    let t19162 = t1885 * t452 * t19103;
    let t19164 = -2.0 * t1149 * t19106 - t1884 * t19162 - t1887 * t19113 + t19104 * t473 + 4.0 * t19115 * t6027 - 2.0 * t19118 * t6035 - 6.0 * t19125 * t6024 + 4.0 * t19129 * t19131 + 4.0 * t19135 * t6024 + 2.0 * t19139 * t6024 - 2.0 * t19143 * t19147 + t19143 * t19159 - 2.0 * t19151 * t6031 - t19156 * t6031 + 2.0 * t3120 * t6019 - t3145 * t6019 - 2.0 * t6022 * t6038;
    (t19151, t19155, t19156, t19158, t19159, t19162, t19164)
}
