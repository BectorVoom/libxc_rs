//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1204/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1204<F: Float>(t1639: F, t4516: F, t520: F, t1232: F, t5448: F, t1265: F, t5381: F, t12828: F, t4459: F, t43101: F, t5413: F, t1640: F, t43602: F, t5408: F, t19809: F, t63840: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t69691 = t4516 * t1639 * t520;
    let t69699 = t5448 * t1232 * t520;
    let t69704 = t5381 * t1265;
    let t69708 = t12828 * t4459;
    let t69727 = t43101 * t520;
    let t69730 = t5413 * t1265;
    let t69734 = t1640 * t4459;
    let t69738 = t43602 * t520;
    let t69741 = t5408 * t1265;
    let t69789 = t63840 * t19809;
    (t69691, t69699, t69704, t69708, t69727, t69730, t69734, t69738, t69741, t69789)
}
