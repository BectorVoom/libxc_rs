//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2053/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2053<F: Float>(t23562: F, t343: F, t88405: F, t1036: F, t25622: F, t14134: F, t6765: F, t1933: F, t23479: F, t88360: F, t88365: F, t25637: F, t984: F) -> (F, F, F, F, F, F) {
    let t88407 = t23562 * t88405 * t343;
    let t88415 = t25622 * t1036 / F::new(216.0);
    let t88422 = t6765 * t14134 / F::new(864.0);
    let t88425 = F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t88360 * t23479;
    let t88428 = F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t88365 * t23479;
    let t88430 = t23562 * t25637 * t984;
    (t88407, t88415, t88422, t88425, t88428, t88430)
}
