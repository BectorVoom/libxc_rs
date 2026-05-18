//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1089/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1089<F: Float>(t17297: F, t942: F, t951: F, t959: F, t2940: F, t5812: F, t5811: F, t952: F, t10296: F, t10556: F, t10784: F, t10785: F, t13552: F, t13566: F, t14287: F, t14291: F, t17173: F, t17180: F, t17185: F) -> (F, F, F, F) {
    let t17299 = t942 * t17297 * t951;
    let t17301 = F::new(0.5848223622634646207e0) * t959 * t17299;
    let t17303 = F::new(0.17315859105681463759e2) * t2940 * t5812;
    let t17304 = t5811 * t952;
    let t17306 = F::new(0.35089341735807877242e1) * t959 * t17304;
    let t17325 = F::new(0.20659e1) * t17173 - t14287 + F::new(0.4630888888888888889e-1) * t13552 + t14291 - F::new(0.68863333333333333332e0) * t13566 - F::new(0.11577222222222222222e0) * t10296 - t10784 - t10785 - F::new(0.34431666666666666667e0) * t17180 + F::new(0.103295e1) * t17185 - F::new(0.22954444444444444444e0) * t10556;
    (t17301, t17303, t17306, t17325)
}
