//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1082/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1082<F: Float>(t17191: F, t324: F, t300: F, t5689: F, t892: F, t914: F, t11094: F, t5950: F, t3216: F, t5946: F, t4483: F, t4498: F) -> (F, F, F, F, F, F) {
    let t17192 = t17191 * t324;
    let t17194 = F::new(0.19751673498613801407e-1) * t300 * t17192;
    let t17195 = t5689 * t892;
    let t17197 = F::new(1.0) * t17195 * t914;
    let t17198 = t5950 * t11094;
    let t17202 = t5946 * t3216;
    let t17209 = F::new(0.34631718211362927517e2) * t4483 * t4498;
    (t17192, t17194, t17197, t17198, t17202, t17209)
}
