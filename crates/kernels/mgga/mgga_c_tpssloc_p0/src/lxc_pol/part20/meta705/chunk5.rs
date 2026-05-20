//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2683/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2683<F: Float>(t54643: F, t40343: F, t40347: F, t40350: F, t40351: F, t40356: F, t54631: F, t54633: F, t54635: F, t54638: F, t54639: F, t40360: F, t40366: F, t40372: F, t40376: F, t40387: F, t40401: F, t40402: F, t40404: F, t40407: F, t40410: F, t40415: F, t40422: F) -> (F, F) {
    let t54644 = F::cast_from(0.14999999999999999999e-1_f64) * t54643;
    let t54647 = -t40343 + t40347 + t40350 - F::cast_from(0.38888888888888888887e-1_f64) * t54631 + F::cast_from(0.32870370370370370369e-1_f64) * t54633 + F::cast_from(0.11666666666666666666e-1_f64) * t54635 - t54638 + F::cast_from(0.56172839506172839502e-1_f64) * t54639 - t54644 - F::cast_from(0.59999999999999999997e-1_f64) * t40351 - F::new(0.15e-1) * t40356;
    let t54658 = F::cast_from(0.49999999999999999998e-2_f64) * t40360 - F::cast_from(0.34999999999999999998e-1_f64) * t40366 + F::cast_from(0.83333333333333333331e-3_f64) * t40372 - F::new(0.75e-2) * t40376 + F::cast_from(0.11666666666666666666e0_f64) * t40387 - t40401 + F::cast_from(0.16851851851851851851e0_f64) * t40402 - F::cast_from(0.38888888888888888889e-1_f64) * t40404 + F::cast_from(0.98611111111111111108e-1_f64) * t40407 + F::cast_from(0.47499999999999999998e-1_f64) * t40410 + F::new(0.1e-1) * t40415 + t40422;
    (t54647, t54658)
}
