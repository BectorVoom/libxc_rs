//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1453;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1454;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1455;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta294<F: Float>(t1036: F, t4617: F, t10422: F, t4574: F, t3070: F, t1597: F, t4509: F, t10189: F, t344: F, t4343: F, t2986: F, t134: F, t2978: F) -> (F, F, F, F, F, F, F, F) {
        let (t13758, t13765, t13767, t13769) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1453::<F>(t1036, t4617, t10422, t4574, t3070, t1597, t4509);
        let t13779 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1454::<F>(t10189, t344);
        let (t13782, t13783, t13784) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1455::<F>(t13779, t4343, t2986, t134, t2978, t344);
    (t13758, t13765, t13767, t13769, t13779, t13782, t13783, t13784)
}
