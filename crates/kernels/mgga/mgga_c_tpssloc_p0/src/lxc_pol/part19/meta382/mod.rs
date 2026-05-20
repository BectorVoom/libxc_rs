//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1429;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1430;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta382<F: Float>(t3333: F, t3351: F, t3374: F, t3399: F, t440: F, t3256: F, t3263: F, t3266: F, t1094: F, t11189: F, t11192: F, t11275: F, t3315: F, t43970: F, t3395: F, t1124: F, t11349: F, t3355: F, t427: F, t3358: F, t11176: F, t1147: F, t3368: F, t3400: F, t11285: F, t11300: F, t11307: F, t11353: F, t11356: F, t11361: F, t11365: F, t1137: F, t11400: F, t11415: F, t11420: F, t1156: F, t1157: F, t3332: F, t3357: F, t3359: F, t3371: F, t3396: F, t3401: F, t3403: F, t3404: F, t43679: F) -> (F, F, F, F, F, F, F) {
        let (t44142, t44146, t44154, t44155, t44161, t44164, t44167) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1429::<F>(t3333, t3351, t3374, t3399, t440, t3256, t3263, t3266, t1094, t11189, t11192, t11275, t3315, t43970);
        let (t44168, t44198) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1430::<F>(t3395, t1124, t11349, t3355, t427, t3358, t11176, t1147, t3368, t3400, t11285, t11300, t11307, t11353, t11356, t11361, t11365, t1137, t11400, t11415, t11420, t1156, t1157, t3332, t3357, t3359, t3371, t3396, t3401, t3403, t3404, t43679, t44142, t44146, t44155, t44161, t44164, t44167);
    (t44142, t44154, t44161, t44164, t44167, t44168, t44198)
}
