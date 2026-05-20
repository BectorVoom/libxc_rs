//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1986/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1986<F: Float>(t16242: F, t5248: F, t5250: F, t12240: F, t5249: F, t3856: F, t12283: F, t5303: F, t1352: F, t3851: F, t1340: F, t16060: F) -> (F, F, F, F, F, F, F) {
    let t16257 = t5248 * t16242 * t5250;
    let t16261 = t5248 * t5249 * t12240;
    let t16265 = t5248 * t5249 * t3856;
    let t16269 = F::new(7.0) / F::new(576.0) * t12283 * t5303;
    let t16271 = t5248 * t16242 * t1352;
    let t16275 = t5248 * t5249 * t3851;
    let t16278 = t16060 * t1340;
    (t16257, t16261, t16265, t16269, t16271, t16275, t16278)
}
