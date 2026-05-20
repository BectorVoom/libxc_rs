//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1544;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1545;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta303<F: Float>(t11177: F, t300: F, t1098: F, t3256: F, t1119: F, t3259: F, t3308: F, t1094: F, t3312: F, t3316: F, t3311: F, t419: F, t409: F, t1117: F, t3265: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11179, t11180, t11182, t11184, t11185) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1544::<F>(t11177, t300, t1098, t3256, t1119, t3259, t3308, t1094, t3312);
        let (t11187, t11189, t11190) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1545::<F>(t11185, t3316, t3311, t419, t409);
        let t11191 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1546::<F>(t1117, t3265);
    (t11179, t11180, t11182, t11184, t11185, t11187, t11189, t11190, t11191)
}
