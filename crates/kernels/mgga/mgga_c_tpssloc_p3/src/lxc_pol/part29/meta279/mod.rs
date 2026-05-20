//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1288;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1289;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta279<F: Float>(t7320: F, t8034: F, t1734: F, t68: F, t475: F, t7328: F, t1730: F, t2140: F, t1742: F, t2139: F, t471: F, t1726: F, t1737: F, t1748: F, t2134: F, t2136: F, t467: F, t488: F, t7309: F, t7310: F, t7315: F, t7326: F, t7339: F, t7343: F, t7345: F, t8020: F, t8028: F, t8031: F, t466: F, t1760: F, t2154: F, t3598: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t8035, t8038, t8039, t8040) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1288::<F>(t7320, t8034, t1734, t68, t475, t7328);
        let (t8043, t8048, t8049, t8054) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1289::<F>(t1730, t2140, t1742, t2139, t471, t1726, t1737, t1748, t2134, t2136, t467, t488, t7309, t7310, t7315, t7326, t7339, t7343, t7345, t8020, t8028, t8031, t8035, t8040);
        let (t8055, t8061) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1290::<F>(t466, t8054, t1760, t2154, t3598);
    (t8035, t8038, t8039, t8040, t8043, t8048, t8049, t8054, t8055, t8061)
}
