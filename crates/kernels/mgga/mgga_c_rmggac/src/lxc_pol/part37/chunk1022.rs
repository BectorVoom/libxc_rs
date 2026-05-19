//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1022/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1022<F: Float>(t78525: F, t76504: F, t16503: F, t2211: F, t34976: F, t9163: F, t16504: F, t699: F, t8425: F, t3369: F, t8430: F, t72115: F) -> (F, F, F, F, F, F) {
    let t78526 = F::cast_from(0.42564599893297839398e-5_f64) * t78525;
    let t78528 = F::cast_from(0.1702583995731913576e-4_f64) * t76504;
    let t78535 = t16503 * t34976 * t2211 * t9163;
    let t78536 = F::cast_from(0.85129199786595678796e-5_f64) * t78535;
    let t78539 = t16503 * t16504 * t699 * t8425;
    let t78540 = F::cast_from(0.12769379967989351819e-4_f64) * t78539;
    let t78543 = t16503 * t3369 * t699 * t8430;
    let t78544 = F::cast_from(0.12769379967989351819e-4_f64) * t78543;
    let t78545 = F::cast_from(0.90915538847484472429e-2_f64) * t72115;
    (t78526, t78528, t78536, t78540, t78544, t78545)
}
