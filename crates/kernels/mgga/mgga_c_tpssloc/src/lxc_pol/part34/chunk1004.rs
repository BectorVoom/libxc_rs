//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1004/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1004<F: Float>(t1860: F, t23992: F, t7445: F, t26012: F, t7031: F, t193: F, t201: F, t7844: F, t2627: F, t7823: F, t10143: F, t3787: F, t7918: F, t531: F, t7939: F, t111: F, t7945: F) -> (F, F, F, F, F, F, F, F) {
    let t92003 = t1860 * t23992 * t7445;
    let t92047 = t7031 * t26012;
    let t92319 = t193 * t201 * t7844;
    let t92521 = t2627 * t7823;
    let t93000 = t7844 * t10143;
    let t93798 = t3787 * t7918;
    let t93966 = t531 * t7939;
    let t94170 = t7945 * t111;
    (t92003, t92047, t92319, t92521, t93000, t93798, t93966, t94170)
}
