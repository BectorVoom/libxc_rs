//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk594;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta114<F: Float>(t1580: F, t2904: F, t1592: F, t2970: F, t973: F, t2978: F, t60: F, t344: F, t1409: F, t2989: F, t2987: F, t135: F, t1599: F, t1597: F) -> (F, F, F, F, F, F, F, F) {
        let (t4488, t4507, t4509, t4510, t4514) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk594::<F>(t1580, t2904, t1592, t2970, t973, t2978, t60, t344, t1409, t2989);
        let (t4518, t4529, t4531) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk595::<F>(t2987, t344, t135, t1599, t973, t1597);
    (t4488, t4507, t4509, t4510, t4514, t4518, t4529, t4531)
}
