//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1916;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta645<F: Float>(t22986: F, t23270: F, t25191: F, t4300: F, t25192: F, t86873: F, t5544: F, t857: F, t865: F, t1527: F, t86849: F, t4272: F, t86969: F, t1520: F, t254: F, t25038: F, t25039: F, t4119: F, t1880: F, t7488: F, t87782: F, t23237: F, t28276: F, t6552: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98248, t98251, t98256, t98264, t98277) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1916::<F>(t22986, t23270, t25191, t4300, t25192, t86873, t5544, t857, t865, t1527, t86849, t4272, t86969);
        let (t98279, t98291, t98305, t98315) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1917::<F>(t1520, t254, t23270, t25038, t25039, t4119, t1880, t7488, t87782, t23237, t28276, t6552);
    (t98248, t98251, t98256, t98264, t98277, t98279, t98291, t98305, t98315)
}
