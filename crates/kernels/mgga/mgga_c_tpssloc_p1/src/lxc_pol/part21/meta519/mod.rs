//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2170;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta519<F: Float>(t17748: F, t4531: F, t4540: F, t7577: F, t4546: F, t343: F, t5842: F, t984: F, t2970: F, t5824: F, t973: F, t10226: F, t13782: F, t13787: F, t13790: F, t13825: F, t17742: F, t17745: F, t2960: F, t2986: F, t5825: F) -> (F, F, F, F, F, F, F) {
        let (t17749, t17752, t17753, t17757, t17758, t17763, t17766) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2170::<F>(t17748, t4531, t4540, t7577, t4546, t343, t5842, t984, t2970, t5824, t973, t10226, t13782, t13787, t13790, t13825, t17742, t17745, t2960, t2986, t5825);
    (t17749, t17752, t17753, t17757, t17758, t17763, t17766)
}
