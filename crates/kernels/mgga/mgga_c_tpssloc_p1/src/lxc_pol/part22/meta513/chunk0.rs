//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1973/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1973<F: Float>(t22032: F, t457: F, t460: F, t974: F, t1714: F, t6144: F, t1178: F, t20217: F, t1177: F, t6138: F, t4934: F, t11516: F, t20234: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22034 = t457 * t22032 * t460;
    let t22035 = t974 * t22034;
    let t22038 = t6144 * t1714;
    let t22040 = t457 * t22038 * t460;
    let t22041 = t974 * t22040;
    let t22046 = t1178 * t20217;
    let t22047 = t1177 * t22046;
    let t22051 = t6138 * t1714 * t460;
    let t22052 = t4934 * t22051;
    let t22055 = t11516 * t20234;
    (t22034, t22035, t22038, t22040, t22041, t22046, t22047, t22051, t22052, t22055)
}
