//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2719/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2719<F: Float>(t57229: F, t40227: F, t40231: F, t40233: F, t118: F, t2375: F, t6320: F, t54477: F, t40224: F, t40230: F, t57218: F, t57219: F, t57220: F, t57221: F, t57222: F, t57223: F, t57224: F, t57225: F, t57226: F, t57228: F) -> (F, F, F, F, F, F, F) {
    let t57230 = F::new(8.0) * t57229;
    let t57231 = F::new(24.0) * t40227;
    let t57232 = F::new(12.0) * t40231;
    let t57233 = F::new(32.0) * t40233;
    let t57235 = t6320 * t118 * t2375;
    let t57236 = F::cast_from(0.10843581300301739842e-1_f64) * t57235;
    let t57237 = F::new(8.0) * t54477;
    let t57238 = t57218 - t57219 - t57220 + t57221 - t57222 - t57223 - t57224 - t57225 + t40224 - t57226 + t57228 - t57230 - t57231 - t40230 + t57232 + t57233 + t57236 + t57237;
    (t57230, t57231, t57232, t57233, t57236, t57237, t57238)
}
