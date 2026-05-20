//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2683/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2683<F: Float>(t16081: F, t20586: F, t40422: F, t54663: F, t54668: F, t54676: F, t54702: F, t54725: F, t56535: F, t56537: F, t56539: F, t56548: F, t56550: F) -> F {
    let t74756 = t16081 * t20586;
    let t74765 = t40422 - F::cast_from(0.59999999999999999998e-1_f64) * t54663 + t54668 - F::cast_from(0.34999999999999999998e-1_f64) * t74756 + F::cast_from(0.47499999999999999998e-1_f64) * t56535 + F::cast_from(0.11666666666666666666e-1_f64) * t56537 - F::cast_from(0.15833333333333333333e-1_f64) * t56539 + F::cast_from(0.47499999999999999999e-1_f64) * t54676 + t54702 + F::cast_from(0.13999999999999999999e0_f64) * t56548 - F::cast_from(0.69999999999999999996e-1_f64) * t56550 + F::cast_from(0.8333333333333333333e-3_f64) * t54725;
    t74765
}
