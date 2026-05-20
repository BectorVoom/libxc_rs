//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1629;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta375<F: Float>(t3131: F, t4649: F, t4593: F, t4582: F, t16558: F, t998: F, t974: F, t13835: F, t4531: F, t13769: F, t13839: F, t1539: F, t6733: F, t4540: F, t7577: F, t4546: F, t343: F, t5842: F, t984: F, t2970: F, t5824: F, t973: F, t10226: F, t13782: F, t13787: F, t13790: F, t13825: F, t2960: F, t2986: F, t5825: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17732, t17733, t17734, t17737, t17738, t17742, t17745, t17748) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1629::<F>(t3131, t4649, t4593, t4582, t16558, t998, t974, t13835, t4531, t13769, t13839, t1539, t6733);
        let (t17752, t17757, t17763, t17764, t17766) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1630::<F>(t17748, t4531, t4540, t7577, t4546, t343, t5842, t984, t2970, t5824, t973, t10226, t13782, t13787, t13790, t13825, t17742, t17745, t2960, t2986, t5825);
    (t17732, t17733, t17734, t17737, t17738, t17748, t17752, t17757, t17763, t17764, t17766)
}
