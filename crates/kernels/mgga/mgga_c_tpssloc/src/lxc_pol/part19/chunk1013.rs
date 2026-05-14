//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1013/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1013<F: Float>(t2617: F, t4177: F, t2628: F, t836: F, t812: F, t242: F, t9972: F, t820: F, t9645: F, t4290: F, t808: F, t68: F, t9971: F, t226: F, t4280: F, t776: F, t868: F) -> (F, F, F, F, F, F, F, F) {
    let t13254 = t2617 * t4177;
    let t13257 = t2628 * t836;
    let t13258 = t812 * t13257;
    let t13261 = t9972 * t242;
    let t13262 = t812 * t13261;
    let t13350 = t9645 * t820;
    let t13390 = t808 * t4290;
    let t13396 = t68 * t9971;
    let t13397 = t226 * t13396;
    let t13453 = t808 * t4280;
    let t13487 = t776 * t868;
    (t13254, t13258, t13262, t13350, t13390, t13397, t13453, t13487)
}
