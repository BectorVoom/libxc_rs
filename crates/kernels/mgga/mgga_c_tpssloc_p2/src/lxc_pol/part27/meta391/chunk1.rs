//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1602/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1602<F: Float>(t3287: F, t4756: F, t1102: F, t3279: F, t4764: F, t4772: F, t699: F, t1107: F, t14758: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F) -> (F, F, F, F, F) {
    let t14813 = t3287 * t4756;
    let t14814 = t14813 * t1102;
    let t14816 = t4764 * t3279;
    let t14818 = t699 * t4772;
    let t14824 = t1107 * t14758;
    let t14827 = -F::cast_from(0.258925e1_f64) * t14809 - F::cast_from(0.1294625e1_f64) * t14811 + F::cast_from(0.16504875e0_f64) * t14814 + F::cast_from(0.82524375e-1_f64) * t14816 + F::cast_from(0.36793333333333333334e-1_f64) * t14818 + F::cast_from(0.26837777777777777778e0_f64) * t11137 + F::cast_from(0.67094444444444444447e-1_f64) * t11139 - F::cast_from(0.20128333333333333334e0_f64) * t11141 - F::cast_from(0.10064166666666666667e0_f64) * t11143 + F::cast_from(0.16504875e0_f64) * t14824 + F::cast_from(0.33547222222222222222e0_f64) * t14728;
    (t14814, t14816, t14818, t14824, t14827)
}
