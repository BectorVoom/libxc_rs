//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2534/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2534<F: Float>(t50948: F, t50946: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t50968: F, t50970: F, t50972: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t44275: F, t50976: F, t50978: F, t50987: F, t50990: F, t50994: F) -> (F, F) {
    let t51310 = F::cast_from(0.13772666666666666666e1_f64) * t50948;
    let t51320 = F::new(0.123954e2) * t50946 + t51310 + F::cast_from(0.68863333333333333333e0_f64) * t50950 + F::cast_from(0.34431666666666666666e0_f64) * t50952 + F::new(0.20659e1) * t50954 - F::new(0.103295e1) * t50957 - F::new(0.103295e1) * t50961 - F::cast_from(0.61977000000000000001e1_f64) * t50966 + F::cast_from(0.13892666666666666667e0_f64) * t50968 + F::cast_from(0.69463333333333333334e-1_f64) * t50970 + F::cast_from(0.41678000000000000001e0_f64) * t50972;
    let t51332 = t44275 - F::cast_from(0.10805407407407407407e0_f64) * t50976 - F::cast_from(0.92617777777777777778e-1_f64) * t50978 + F::cast_from(0.68863333333333333332e0_f64) * t43780 + F::cast_from(0.13772666666666666666e1_f64) * t43782 + F::cast_from(0.68863333333333333332e0_f64) * t43784 - F::new(0.103295e1) * t43786 - F::cast_from(0.17215833333333333333e0_f64) * t43788 - F::cast_from(0.16068111111111111111e1_f64) * t43816 + F::cast_from(0.13892666666666666667e0_f64) * t50987 + F::cast_from(0.55570666666666666666e0_f64) * t50990 - F::new(0.61977e1) * t50994;
    (t51320, t51332)
}
