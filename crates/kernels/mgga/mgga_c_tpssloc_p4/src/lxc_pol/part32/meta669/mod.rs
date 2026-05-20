//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2101;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta669<F: Float>(t27498: F, t85853: F, t27533: F, t86094: F, t24826: F, t27521: F, t24574: F, t27462: F, t3030: F, t460: F, t27488: F, t27491: F, t27495: F, t27497: F, t1170: F, t2121: F, t27732: F, t15590: F, t7338: F, t27614: F, t3572: F, t27617: F, t3523: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t95136, t95163, t95165, t95192, t95195, t95197) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2101::<F>(t27498, t85853, t27533, t86094, t24826, t27521, t24574, t27462, t3030, t460, t27488, t27491);
        let (t95201, t95213, t95238, t95242, t95244) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2102::<F>(t27495, t27497, t95195, t1170, t2121, t27732, t15590, t7338, t27614, t3572, t27617, t3523);
    (t95136, t95163, t95165, t95192, t95197, t95201, t95213, t95238, t95242, t95244)
}
