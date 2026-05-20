//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1438/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1438<F: Float>(t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43823: F, t43828: F, t44348: F) -> F {
    let t44355 = -F::cast_from(0.52765432098765432099e-1_f64) * t43811 + F::cast_from(0.47488888888888888888e-1_f64) * t43727 - F::cast_from(0.14246666666666666667e0_f64) * t43729 + F::cast_from(0.11872222222222222222e0_f64) * t43734 - F::cast_from(0.73871604938271604937e-1_f64) * t43816 + t44348 - F::cast_from(0.42739999999999999999e0_f64) * t43737 - F::cast_from(0.35616666666666666666e-1_f64) * t43823 - F::cast_from(0.47488888888888888888e-1_f64) * t43740 + F::new(0.6411e0) * t43743 + F::new(0.10685e0) * t43828 + F::cast_from(0.14246666666666666667e0_f64) * t43746;
    t44355
}
