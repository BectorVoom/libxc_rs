//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1060/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1060<F: Float>(t21753: F, t21808: F, t1118: F, t1099: F, t11277: F, t21723: F, t11275: F, t11136: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F) -> (F, F, F, F, F, F) {
    let t21809 = t21753 + t21808;
    let t21810 = t21809 * t1118;
    let t21812 = F::new(1.0) * t1099 * t21810;
    let t21813 = t21723 * t11277;
    let t21815 = F::cast_from(0.51726012919273400301e3_f64) * t11275 * t21813;
    let t21826 = -t11136 + F::cast_from(0.12361111111111111111e-1_f64) * t14702 + F::cast_from(0.61805555555555555556e-2_f64) * t18203 - F::cast_from(0.18541666666666666667e-1_f64) * t18219 - F::cast_from(0.92708333333333333334e-2_f64) * t18229 + F::cast_from(0.10300925925925925926e-1_f64) * t21760 - F::cast_from(0.37083333333333333333e-1_f64) * t21764 - F::cast_from(0.18541666666666666666e-1_f64) * t21767 + F::cast_from(0.55625000000000000001e-1_f64) * t21771 + F::cast_from(0.55625000000000000001e-1_f64) * t21774 + F::cast_from(0.92708333333333333333e-2_f64) * t21778;
    (t21809, t21810, t21812, t21813, t21815, t21826)
}
