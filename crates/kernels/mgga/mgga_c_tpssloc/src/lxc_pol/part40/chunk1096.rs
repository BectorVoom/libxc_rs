//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1096/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1096<F: Float>(t17488: F, t291: F, t2932: F, t5790: F, t950: F, t4471: F, t4475: F, t10632: F, t5774: F, t13727: F, t4359: F, t13520: F, t4400: F) -> (F, F, F, F, F, F) {
    let t17490 = F::new(0.621814e-1) * t17488 * t291;
    let t17492 = t5790 * t2932;
    let t17493 = t17492 * t950;
    let t17496 = t4475 * t4471;
    let t17499 = t5774 * t10632;
    let t17500 = t17499 * t950;
    let t17504 = F::new(4.0) * t13727 * t4359;
    let t17506 = F::new(0.32163958997385070134e2) * t13520 * t4400;
    (t17490, t17493, t17496, t17500, t17504, t17506)
}
