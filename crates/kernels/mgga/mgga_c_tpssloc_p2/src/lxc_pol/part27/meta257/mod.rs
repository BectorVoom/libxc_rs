//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1244;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1245;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1246;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1247;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1248;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1249;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1250;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta257<F: Float>(t1336: F, t6944: F, t1354: F, t1358: F, t2003: F, t552: F, t59: F, t240: F, t1369: F, t6915: F, t6917: F, t6922: F, t6929: F, t6935: F, t6938: F, t6941: F, t539: F, t2007: F, t225: F, t1385: F, t2015: F, t3887: F, t2010: F, t6883: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t6945 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1244::<F>(t1336, t6944);
        let (t6946, t6949, t6950, t6951) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1245::<F>(t1354, t6945, t1358, t2003, t552, t59, t240);
        let t6952 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1246::<F>(t1336, t6951);
        let t6955 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1247::<F>(t1369, t6952, t6915, t6917, t6922, t6929, t6935, t6938, t6941, t6946, t6949);
        let (t6956, t6958) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1248::<F>(t539, t6955, t2007, t225);
        let t6963 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1249::<F>(t1385, t2015, t3887);
        let (t6967, t6968) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1250::<F>(t2010, t6883, t552, t562);
    (t6945, t6949, t6950, t6951, t6952, t6955, t6956, t6958, t6963, t6967, t6968)
}
