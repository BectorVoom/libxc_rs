//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 671/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk671<F: Float>(t1509: F, t2868: F, t1027: F, t2836: F, t2872: F, t4044: F, t4049: F, t4054: F, t4058: F, t1025: F, t2885: F, t1032: F, t1515: F, t673: F) -> (F, F, F, F, F, F, F, F) {
    let t4071 = t2868 * t1509;
    let t4072 = t4071 * t1027;
    let t4079 = t2872 - t2836 / 9.0 - t4044 / 9.0 - 2.0 / 9.0 * t4049 + 2.0 / 3.0 * t4054 + t4058 / 3.0;
    let t4080 = t1025 * t4079;
    let t4087 = t2885 * t1509;
    let t4088 = t4087 * t1027;
    let t4090 = t1032 * t4079;
    let t4093 = t673 * t1515;
    (t4071, t4072, t4079, t4080, t4087, t4088, t4090, t4093)
}
