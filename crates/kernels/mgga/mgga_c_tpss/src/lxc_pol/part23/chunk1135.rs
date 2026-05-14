//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1135/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1135<F: Float>(t17964: F, t2179: F, t2367: F, t5552: F, t2372: F, t1699: F, t2379: F, t339: F, t5557: F, t789: F) -> (F, F, F, F, F) {
    let t17965 = t17964 * t2179;
    let t17967 = t5552 * t2367;
    let t17969 = t5552 * t2372;
    let t17971 = t1699 * t2379;
    let t17972 = 119.0 / 6912.0 * t17971;
    let t17974 = t339 * t5557 * t789;
    (t17965, t17967, t17969, t17972, t17974)
}
