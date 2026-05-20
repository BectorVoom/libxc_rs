//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1229/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1229<F: Float>(t16689: F, t4101: F, t16701: F, t4205: F, t20741: F, t706: F, t20234: F, t751: F, t9897: F, t20742: F, t67: F, t758: F) -> (F, F, F, F, F) {
    let t67177 = t16689 * t4101;
    let t67179 = t4205 * t16701;
    let t67181 = t706 * t20741;
    let t67185 = t9897 * t751 * t20234;
    let t67209 = t20742 * t67 * t758;
    (t67177, t67179, t67181, t67185, t67209)
}
