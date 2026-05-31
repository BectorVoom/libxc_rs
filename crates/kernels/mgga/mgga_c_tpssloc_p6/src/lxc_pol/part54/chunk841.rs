//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 841/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk841<F: Float>(t1720: F, t2144: F, t131: F, t7998: F, t2130: F, t1932: F, t2133: F, t2132: F, t7573: F, t1714: F, t460: F, t7320: F, rho1: F) -> (F, F, F, F, F, F, F, F) {
    let t8018 = t1720 * t2144;
    let t8020 = t7998 * t131;
    let t8025 = t2130 * rho1;
    let t8026 = F::cast_from(1.0_f64) / t8025;
    let t8027 = t8026 * t1932;
    let t8028 = t8027 * t2133;
    let t8031 = t2132 * t7573;
    let t8034 = t1714 * t460;
    let t8035 = t8034 * t7320;
    (t8018, t8020, t8026, t8027, t8028, t8031, t8034, t8035)
}
