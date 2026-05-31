//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2409/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2409<F: Float>(t47707: F, t48096: F, t41831: F, t41833: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F) -> F {
    let t49306 = F::cast_from(0.45908888888888888888e0_f64) * t47707;
    let t49317 = F::cast_from(0.34731666666666666667e0_f64) * t48096;
    let t49318 = -t49306 + F::cast_from(0.68863333333333333333e0_f64) * t47709 + F::cast_from(0.34431666666666666666e0_f64) * t47711 + F::cast_from(0.57386111111111111111e0_f64) * t47713 - F::cast_from(0.20659e1_f64) * t47715 - F::cast_from(0.103295e1_f64) * t47717 - F::cast_from(0.17215833333333333333e1_f64) * t47722 - F::cast_from(0.20658999999999999999e1_f64) * t47724 - F::cast_from(0.123954e2_f64) * t47728 + F::cast_from(0.69463333333333333333e0_f64) * t41831 + F::cast_from(0.41678000000000000001e0_f64) * t41833 - t49317;
    t49318
}
