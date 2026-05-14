//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 853/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk853<F: Float>(t33: F, t259: F, t479: F, t1151: F, t1153: F, t198: F, t330: F, t4023: F, t5664: F, t6040: F, t6044: F, t1893: F, t5685: F, t57: F, t581: F, t5999: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t6048 = piecewise3(t480, t1153 * t198 * t330 * t6040 - t1151 * t4023 * t6044, t5664);
    let t6053 = piecewise3(t386, t5685, -t1893 * t581 / 2.0 + t6048 * t57 / 2.0);
    let t6054 = t5999 + t6053;
    (t6048, t6054)
}
