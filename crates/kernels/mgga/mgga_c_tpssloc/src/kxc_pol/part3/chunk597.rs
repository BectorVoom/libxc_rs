//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 597/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk597<F: Float>(t2880: F, t932: F, t922: F, t302: F, t310: F, t2862: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F, t324: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2881 = t2880 * t932;
    let t2884 = t922 * t922;
    let t2885 = F::new(1.0) / t2884;
    let t2886 = t302 * t2885;
    let t2887 = t310 * t310;
    let t2888 = F::new(1.0) / t2887;
    let t2889 = t2862 * t2888;
    let t2892 = F::new(0.12361111111111111111e-1) * t2764;
    let t2897 = t2892 + F::new(0.61805555555555555556e-2) * t2766 - F::new(0.61805555555555555555e-2) * t2773 + F::new(0.18541666666666666667e-1) * t2778 - F::new(0.92708333333333333333e-2) * t2782;
    let t2898 = t2897 * t324;
    (t2881, t2884, t2885, t2886, t2887, t2888, t2889, t2892, t2897, t2898)
}
