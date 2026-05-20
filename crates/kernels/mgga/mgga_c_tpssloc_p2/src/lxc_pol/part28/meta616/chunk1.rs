//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1933/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1933<F: Float>(t26322: F, t80855: F, t91152: F, t236: F, t26318: F, t91005: F, t22782: F, t5234: F, t1369: F, t26257: F, t3876: F, t1831: F, t80849: F) -> (F, F, F, F, F) {
    let t91154 = t91152 * t80855 * t26322;
    let t91158 = t91152 * t91005 * t236 * t26318;
    let t91160 = t5234 * t22782;
    let t91161 = t91160 * t1369;
    let t91163 = t26257 * t3876;
    let t91165 = t80849 * t1831;
    (t91154, t91158, t91161, t91163, t91165)
}
