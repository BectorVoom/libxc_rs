//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2402/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2402<F: Float>(t41741: F, t47787: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t59700: F, t59702: F, t59704: F, t59759: F, t59761: F, t68586: F, t68589: F, t68592: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F) -> F {
    let t68756 = F::cast_from(0.55625000000000000001e-1_f64) * t68586 + F::cast_from(0.18541666666666666667e-1_f64) * t68589 - F::cast_from(0.61805555555555555555e-2_f64) * t68592 + F::cast_from(0.12361111111111111111e0_f64) * t68596 - F::cast_from(0.30902777777777777778e-1_f64) * t68599 + F::new(0.11125e0) * t68602 - F::cast_from(0.30902777777777777777e-1_f64) * t68605 - F::new(0.166875e0) * t68608 - F::cast_from(0.18541666666666666667e-1_f64) * t59663 + F::cast_from(0.61805555555555555556e-2_f64) * t59665 + F::cast_from(0.92708333333333333334e-2_f64) * t59680 + F::cast_from(0.24722222222222222223e-1_f64) * t59688 - F::cast_from(0.12361111111111111111e-1_f64) * t59694 + t41741 - F::cast_from(0.37083333333333333333e-1_f64) * t59700 + F::cast_from(0.12361111111111111111e-1_f64) * t59702 + F::cast_from(0.10300925925925925926e-1_f64) * t59704 + F::cast_from(0.28842592592592592592e-1_f64) * t47787 + F::cast_from(0.55625000000000000001e-1_f64) * t59759 - F::cast_from(0.37083333333333333334e-1_f64) * t59761;
    t68756
}
