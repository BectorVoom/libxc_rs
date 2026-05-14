//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 911/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk911<F: Float>(t25224: F, t6572: F, t1880: F, t13053: F, t1528: F, t1912: F, t23235: F, t23281: F, t25200: F, t25206: F, t25209: F, t25211: F, t25214: F, t25218: F, t25220: F, t25222: F, t259: F, t2713: F, t7538: F, t855: F) -> (F, F) {
    let t25225 = t25224 * t6572;
    let t25226 = t1880 * t25225;
    let t25228 = 0.19190897446562641759e-1 * t23235 + 2.0 * t855 * t25200 - t2713 * t7538 - t23281 * t1528 + 0.41123351671205660912e-2 * t25206 - t13053 * t1912 + 0.38381794893125283518e-1 * t25209 + 0.19190897446562641759e-1 * t25211 - 0.82246703342411321825e-2 * t25214 - 0.82246703342411321825e-2 * t25218 + t25220 * t259 + t25222 * t259 - 0.82246703342411321825e-2 * t25226;
    (t25226, t25228)
}
