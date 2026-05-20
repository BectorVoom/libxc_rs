//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1175;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1176;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta187<F: Float>(t4343: F, t4518: F, t3966: F, t978: F, t977: F, t135: F, t1599: F, t973: F, t1597: F, t2987: F, t2990: F, t2824: F, t3003: F, t4384: F, t4387: F, t4390: F, t4393: F, t340: F, t343: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4519, t4522, t4523, t4528, t4529, t4531) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1175::<F>(t4343, t4518, t3966, t978, t977, t135, t1599, t973, t1597, t2987);
        let (t4532, t4540) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1176::<F>(t2990, t4531, t2824, t3003, t4384, t4387, t4390, t4393);
        let (t4542, t4543, t4546) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1177::<F>(t340, t4540, t343, t974);
    (t4519, t4522, t4523, t4528, t4529, t4531, t4532, t4540, t4542, t4543, t4546)
}
