//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2652/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2652<F: Float>(t74077: F, t54411: F, t120: F, t20553: F, t12283: F, t20454: F, t20489: F) -> (F, F, F, F, F) {
    let t74078 = F::cast_from(0.5848223622634646207e0_f64) * t74077;
    let t74086 = F::new(3.0) * t54411;
    let t74090 = t120 * t20553;
    let t74110 = t12283 * t20454;
    let t74120 = t120 * t20489;
    (t74078, t74086, t74090, t74110, t74120)
}
