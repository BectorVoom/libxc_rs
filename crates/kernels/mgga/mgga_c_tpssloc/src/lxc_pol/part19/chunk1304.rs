//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1304/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1304<F: Float>(t3333: F, t3351: F, t3374: F, t3399: F, t440: F, t3256: F, t3263: F, t3266: F, t1094: F, t11189: F, t11192: F, t11275: F, t3315: F, t43970: F, t3395: F, t1124: F, t11349: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44142 = t3333 * t3333;
    let t44146 = t3351 * t3351;
    let t44154 = 1.0 / t3399 / t3374;
    let t44155 = t440 * t44154;
    let t44159 = t3256 * t3263;
    let t44161 = 12.0 * t44159 * t3266;
    let t44162 = t1094 * t11189;
    let t44164 = 0.3859675079686208416e3 * t44162 * t11192;
    let t44167 = 0.57895126195293126241e3 * t11275 * t43970 * t3315;
    let t44168 = t3395 * t3395;
    let t44172 = t1124 * t11349;
    (t44142, t44146, t44154, t44155, t44161, t44164, t44167, t44168, t44172)
}
