//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1083/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1083<F: Float>(t16503: F, t2211: F, t34976: F, t9163: F, t16504: F, t699: F, t8425: F, t3369: F, t8430: F, t72115: F, t76506: F, t70506: F) -> (F, F, F, F, F, F) {
    let t78535 = t16503 * t34976 * t2211 * t9163;
    let t78536 = F::new(0.85129199786595678796e-5) * t78535;
    let t78539 = t16503 * t16504 * t699 * t8425;
    let t78540 = F::new(0.12769379967989351819e-4) * t78539;
    let t78543 = t16503 * t3369 * t699 * t8430;
    let t78544 = F::new(0.12769379967989351819e-4) * t78543;
    let t78545 = F::new(0.90915538847484472429e-2) * t72115;
    let t78546 = F::new(0.23948483403727617128e0) * t76506;
    let t78547 = F::new(0.10248087766267884741e-3) * t70506;
    (t78536, t78540, t78544, t78545, t78546, t78547)
}
