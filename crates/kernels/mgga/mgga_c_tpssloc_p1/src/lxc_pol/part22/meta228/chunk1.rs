//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1290/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1290<F: Float>(t849: F, t9601: F, t241: F, t6589: F, t67: F, t2632: F, t776: F, t815: F, t836: F, t812: F) -> (F, F, F, F, F) {
    let t9602 = t9601 * t849;
    let t9607 = t241 * t6589 * t67;
    let t9627 = t2632 * t776;
    let t9637 = t815 * t836;
    let t9638 = t812 * t9637;
    (t9602, t9607, t9627, t9637, t9638)
}
