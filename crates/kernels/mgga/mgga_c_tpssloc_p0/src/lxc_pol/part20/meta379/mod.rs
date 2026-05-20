//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1734;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1735;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta379<F: Float>(t13093: F, t13099: F, t13111: F, t13138: F, t225: F, t68: F, t822: F, t1484: F, t1891: F, t2379: F, t4119: F, t845: F, t776: F, t2553: F, t4226: F, t12971: F, t824: F, t1504: F, t1506: F, t228: F, t230: F, t2667: F, t2672: F, t2675: F, t4219: F, t4225: F, t4227: F, t4230: F, t825: F, t232: F, t819: F, t820: F, t4162: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13141, t13151, t13156, t13157, t13160) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1734::<F>(t13093, t13099, t13111, t13138, t225, t68, t822, t1484, t1891, t2379, t4119, t845);
        let (t13161, t13164, t13167, t13170) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1735::<F>(t13160, t776, t2553, t4226, t12971, t824, t13141, t13151, t13157, t1504, t1506, t228, t230, t2667, t2672, t2675, t4219, t4225, t4227, t4230, t822, t825);
        let (t13171, t13173, t13176) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1736::<F>(t13170, t232, t819, t820, t4162, t68);
    (t13141, t13151, t13156, t13157, t13160, t13161, t13164, t13167, t13170, t13171, t13173, t13176)
}
