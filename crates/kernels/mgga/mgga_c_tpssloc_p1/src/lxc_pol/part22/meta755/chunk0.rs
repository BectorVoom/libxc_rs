//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2537/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2537<F: Float>(t71371: F, t71389: F, t1107: F, t63332: F, t63334: F, t63336: F, t63886: F, t63888: F, t63893: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F) -> (F, F, F) {
    let t71390 = t71371 + t71389;
    let t71391 = t1107 * t71390;
    let t71396 = F::cast_from(0.10064166666666666667e1_f64) * t71124 - F::cast_from(0.26837777777777777777e0_f64) * t63332 + F::cast_from(0.40256666666666666668e0_f64) * t63334 - F::cast_from(0.30192500000000000001e0_f64) * t63336 - F::new(0.36231e1) * t71130 - F::new(0.16557e0) * t63886 - F::cast_from(0.91983333333333333334e-1_f64) * t63888 + F::new(0.5519e0) * t63893 + F::new(0.16504875e0) * t71391 + F::cast_from(0.40256666666666666666e1_f64) * t71135 - F::cast_from(0.20128333333333333333e0_f64) * t71140 + F::cast_from(0.20128333333333333333e0_f64) * t71142;
    (t71390, t71391, t71396)
}
