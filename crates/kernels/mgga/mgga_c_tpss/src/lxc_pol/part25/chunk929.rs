//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 929/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk929<F: Float>(t30: F, t5458: F, t9895: F, t12758: F, t177: F, t5343: F, t737: F, t3205: F, t10016: F, t5328: F, t9924: F, t3217: F, t4578: F, t1197: F, t13334: F, t1989: F, t4380: F, t580: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t13627 = t5458 * t9895;
    let t13631 = 0.23392894490538584828e1 * t12758;
    let t13635 = t5343 * t177;
    let t13636 = t13635 * t737;
    let t13637 = 0.5848223622634646207e0 * t13636;
    let t13641 = t5458 * t3205;
    let t13645 = 12.0 * t10016;
    let t13646 = t9924 * t5328;
    let t13651 = t3217 * t4578;
    let t13657 = piecewise3(t31, 0.0, 8.0 / 27.0 * t13646 * t580 - 8.0 / 9.0 * t4380 * t1989 - 2.0 / 9.0 * t13651 * t580 + 2.0 / 3.0 * t1197 * t13334);
    (t13627, t13631, t13637, t13641, t13645, t13657)
}
