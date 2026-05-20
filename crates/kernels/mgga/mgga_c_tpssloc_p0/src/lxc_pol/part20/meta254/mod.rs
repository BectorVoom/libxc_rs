//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1383;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta254<F: Float>(t232: F, t2553: F, t2645: F, t2646: F, t2614: F, t838: F, t2693: F, t809: F, t225: F, t9584: F, t237: F, t597: F, t61: F) -> (F, F, F, F, F, F, F) {
        let (t10007, t10009, t10012, t10014, t10016, t10017, t10021) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1383::<F>(t232, t2553, t2645, t2646, t2614, t838, t2693, t809, t225, t9584, t237, t597, t61);
    (t10007, t10009, t10012, t10014, t10016, t10017, t10021)
}
