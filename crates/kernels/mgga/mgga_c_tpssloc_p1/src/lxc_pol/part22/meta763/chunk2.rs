//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2572/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2572<F: Float>(t43816: F, t51349: F, t51354: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t64074: F, t64076: F, t64087: F, t64089: F) -> F {
    let t71989 = -F::cast_from(0.5356037037037037037e0_f64) * t43816 + t51349 - t51354 + F::cast_from(0.13772666666666666667e1_f64) * t63361 + F::cast_from(0.68863333333333333332e0_f64) * t63382 + F::cast_from(0.20658999999999999999e1_f64) * t63384 - F::new(0.20659e1) * t63398 - F::new(0.309885e1) * t63400 + F::cast_from(0.13892666666666666667e0_f64) * t64074 + F::cast_from(0.41678000000000000001e0_f64) * t64076 - F::cast_from(0.83356000000000000002e0_f64) * t64087 - F::new(0.125034e1) * t64089;
    t71989
}
