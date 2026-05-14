//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 846/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk846<F: Float>(t5519: F, t706: F, t13115: F, t157: F, t5398: F, t751: F, t707: F, t5522: F, t67: F, t758: F, t184: F, t5392: F, t1504: F, t68: F, t1891: F, t5527: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16689 = t706 * t5519;
    let t16693 = t13115 * t157;
    let t16701 = t751 * t5398;
    let t16702 = t707 * t16701;
    let t16710 = t5522 * t67;
    let t16711 = t16710 * t758;
    let t16716 = t184 * t5392;
    let t16729 = t1504 * t68;
    let t16736 = t1891 * t5527;
    (t16689, t16693, t16701, t16702, t16710, t16711, t16716, t16729, t16736)
}
