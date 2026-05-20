//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1247;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta340<F: Float>(t13969: F, t4584: F, t1041: F, t4589: F, t2960: F, t4603: F, t1606: F, t698: F, t973: F, t1043: F, t2770: F, t10277: F, t3061: F, t10216: F, t10969: F, t135: F, t4608: F, t10868: F, t1539: F, t248: F, t1009: F, t4552: F, t1011: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14136, t14139, t14158, t14160, t14164, t14172) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1247::<F>(t13969, t4584, t1041, t4589, t2960, t4603, t1606, t698, t973, t1043, t2770, t10277, t3061);
        let (t14187, t14194, t14203, t14205, t14206) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1248::<F>(t10216, t10969, t135, t4608, t973, t10868, t1539, t248, t1041, t1009, t4552, t1011);
    (t14136, t14139, t14158, t14160, t14164, t14172, t14187, t14194, t14203, t14205, t14206)
}
