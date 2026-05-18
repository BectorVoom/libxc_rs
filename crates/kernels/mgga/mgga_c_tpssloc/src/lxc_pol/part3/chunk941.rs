//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 941/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk941<F: Float>(t3399: F, t445: F, t1143: F, t3375: F, t1124: F, t3331: F, t11282: F, t440: F, t11135: F, t11203: F, t1127: F, t3355: F) -> (F, F, F, F, F, F, F) {
    let t11292 = F::new(1.0) / t3399 / t445;
    let t11297 = t1143 * t3375;
    let t11303 = t1124 * t3331;
    let t11310 = t440 * t11282;
    let t11314 = F::new(0.16068111111111111111e1) * t11135;
    let t11317 = F::new(0.46308888888888888888e0) * t11203;
    let t11349 = F::new(1.0) / t3355 / t1127;
    (t11292, t11297, t11303, t11310, t11314, t11317, t11349)
}
