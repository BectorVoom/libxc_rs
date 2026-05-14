//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 775/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk775<F: Float>(t25: F, t28: F, t1298: F, t3704: F, t5397: F, t6305: F, t1302: F, t3711: F, t5966: F, t6312: F, zeta_threshold: F) -> (F,) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t6339 = piecewise3(t26, 0.0, -2.0 / 9.0 * t3704 * t6305 + 2.0 / 3.0 * t1298 * t5397);
    let t6345 = piecewise3(t29, 0.0, -2.0 / 9.0 * t3711 * t6312 + 2.0 / 3.0 * t1302 * t5966);
    let t6347 = t6339 / 2.0 + t6345 / 2.0;
    (t6347,)
}
