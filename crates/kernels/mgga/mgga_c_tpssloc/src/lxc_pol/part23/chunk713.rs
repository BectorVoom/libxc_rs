//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 713/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk713<F: Float>(t466: F, t6238: F, t1760: F, t3598: F, t491: F, t6224: F, t3612: F, t1734: F, t1751: F, t1246: F, t6218: F, t3625: F, t493: F, t1244: F, t1729: F, t1756: F, t1758: F, t3610: F, t3624: F, t470: F, t494: F, t5064: F, t6168: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6239 = t466 * t6238;
    let t6243 = t1760 * t1760;
    let t6244 = t3598 * t6243;
    let t6252 = t491 * t6224;
    let t6253 = t6252 * t3612;
    let t6256 = t1751 * t1734;
    let t6257 = t6256 * t1246;
    let t6260 = t491 * t6218;
    let t6261 = t6260 * t1246;
    let t6263 = t6252 * t3625;
    let t6265 = t493 * t6238;
    let t6267 = 2.0 * t1244 * t6257 + t1244 * t6261 + 2.0 * t1729 * t1758 + 2.0 * t1756 * t5064 + 2.0 * t3610 * t6253 - t3624 * t6263 + t470 * t6265 + t494 * t6168;
    (t6239, t6243, t6244, t6252, t6253, t6256, t6257, t6260, t6261, t6263, t6265, t6267)
}
