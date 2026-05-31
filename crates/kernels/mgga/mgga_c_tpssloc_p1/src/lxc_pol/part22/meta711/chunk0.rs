//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2308/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2308<F: Float>(t58021: F, t46278: F, t1484: F, t4303: F, t16634: F, t4205: F, t40738: F, t40754: F, t12895: F, t2522: F, t40741: F, t40743: F, t40748: F, t40760: F, t4307: F, t5544: F) -> (F, F, F, F, F, F) {
    let t67162 = F::cast_from(0.17544670867903938621e1_f64) * t58021;
    let t67163 = F::cast_from(0.48796115851357829289e-1_f64) * t46278;
    let t67164 = t1484 * t4303;
    let t67169 = F::cast_from(12.0_f64) * t4205 * t16634;
    let t67170 = F::cast_from(0.21687162600603479684e-1_f64) * t40738;
    let t67174 = F::cast_from(0.10389515463408878255e3_f64) * t40754;
    let t67175 = F::cast_from(9.0_f64) * t12895 * t2522 * t5544 - F::cast_from(18.0_f64) * t2522 * t4307 * t67164 - t40741 - t40743 + t40748 + t40760 - t67162 + t67163 + t67169 - t67170 + t67174;
    (t67162, t67163, t67169, t67170, t67174, t67175)
}
