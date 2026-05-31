//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2554/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2554<F: Float>(t43816: F, t51039: F, t51051: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t64074: F, t64076: F, t64087: F, t64089: F) -> F {
    let t71649 = -F::cast_from(0.31003950617283950618e0_f64) * t43816 + F::cast_from(0.54771111111111111112e0_f64) * t51039 - F::cast_from(0.91285185185185185187e-1_f64) * t51051 + F::cast_from(0.79724444444444444444e0_f64) * t63361 + F::cast_from(0.39862222222222222222e0_f64) * t63382 + F::cast_from(0.11958666666666666667e1_f64) * t63384 - F::cast_from(0.11958666666666666667e1_f64) * t63398 - F::cast_from(0.17938e1_f64) * t63400 + F::cast_from(0.10954222222222222222e0_f64) * t64074 + F::cast_from(0.32862666666666666666e0_f64) * t64076 - F::cast_from(0.65725333333333333332e0_f64) * t64087 - F::cast_from(0.98587999999999999998e0_f64) * t64089;
    t71649
}
