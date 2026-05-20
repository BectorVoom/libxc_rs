//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2858/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2858<F: Float>(t47787: F, t59727: F, t59732: F, t59735: F, t59738: F, t59744: F, t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t59769: F) -> F {
    let t59771 = -F::cast_from(0.10300925925925925926e-1_f64) * t59727 + F::cast_from(0.37083333333333333334e-1_f64) * t59732 - F::cast_from(0.41203703703703703704e-1_f64) * t59735 + F::cast_from(0.14833333333333333333e0_f64) * t59738 + F::cast_from(0.19228395061728395062e-1_f64) * t47787 + F::cast_from(0.37083333333333333333e-1_f64) * t59744 - F::cast_from(0.55625000000000000001e-1_f64) * t59748 + F::cast_from(0.12361111111111111111e0_f64) * t59753 - F::cast_from(0.22249999999999999999e0_f64) * t59757 + F::cast_from(0.37083333333333333334e-1_f64) * t59759 - F::cast_from(0.24722222222222222222e-1_f64) * t59761 - F::cast_from(0.55625000000000000001e-1_f64) * t59765 + F::cast_from(0.37083333333333333334e-1_f64) * t59769;
    t59771
}
