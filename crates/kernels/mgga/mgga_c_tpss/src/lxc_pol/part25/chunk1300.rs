//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1300/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1300<F: Float>(t1284: F, t6441: F, t1659: F, t4519: F, t1270: F, t13671: F, t1268: F, t5458: F, t1206: F, t21011: F, t19619: F, t6242: F) -> (F, F, F, F, F, F) {
    let t67879 = F::new(2.0) * t6441 * t1284;
    let t68798 = t1659 * t4519;
    let t68823 = t1270 * t13671;
    let t68827 = t5458 * t1268;
    let t68838 = t21011 * t1206;
    let t68868 = t6242 * t19619;
    (t67879, t68798, t68823, t68827, t68838, t68868)
}
