//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1190;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1191;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1192;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1193;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1194;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta247<F: Float>(t1926: F, t995: F, t1919: F, t210: F, t1929: F, t1932: F, rho0: F, t1934: F, t1933: F, t40: F, t1937: F, t3: F, t607: F, t343: F, t984: F, t1948: F, t363: F, t3034: F, t334: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6716, t6717) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1190::<F>(t1926, t995, t1919, t210);
        let (t6721, t6722) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1191::<F>(t1929, t1932, rho0);
        let (t6723, t6728, t6729, t6730, t6733) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1192::<F>(t1934, t6722, t1933, t40, t1937, t3, t607, t343, t984);
        let t6734 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1193::<F>(t1948, t363);
        let t6735 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1194::<F>(t6733, t6734);
        let t6739 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1195::<F>(t3034, t334);
    (t6716, t6717, t6721, t6722, t6723, t6728, t6729, t6730, t6733, t6734, t6735, t6739)
}
