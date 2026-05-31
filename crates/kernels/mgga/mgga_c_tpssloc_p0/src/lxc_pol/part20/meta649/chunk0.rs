//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2388/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2388<F: Float>(t10599: F, t2799: F, t4370: F, t10595: F, t10596: F, t1547: F, t41935: F, t41942: F, t41887: F, t41889: F, t48134: F, t48137: F, t48142: F, t48145: F, t48148: F, t49009: F) -> (F, F, F, F, F) {
    let t49012 = t10599 * t4370 * t2799;
    let t49015 = t10595 * t4370 * t2799;
    let t49018 = t41935 * t1547 * t10596;
    let t49021 = t41942 * t1547 * t10596;
    let t49026 = -F::cast_from(0.27385555555555555556e-1_f64) * t48134 - F::cast_from(0.85199506172839506175e-1_f64) * t48137 + F::cast_from(0.49293999999999999999e0_f64) * t48142 - F::cast_from(0.147882e1_f64) * t48145 - F::cast_from(0.9494625e0_f64) * t49009 - F::cast_from(0.230371875e0_f64) * t49012 + F::cast_from(0.427258125e1_f64) * t49015 - F::cast_from(0.3560484375e1_f64) * t49018 + F::cast_from(0.1151859375e0_f64) * t49021 - F::cast_from(0.10954222222222222222e0_f64) * t48148 - F::cast_from(0.32862666666666666666e0_f64) * t41887 + F::cast_from(0.54771111111111111111e-1_f64) * t41889;
    (t49012, t49015, t49018, t49021, t49026)
}
