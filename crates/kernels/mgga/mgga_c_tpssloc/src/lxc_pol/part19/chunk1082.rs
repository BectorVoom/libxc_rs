//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1082/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1082<F: Float>(t12313: F, t3726: F, t2559: F, t3732: F, t3766: F, t12214: F, t782: F, t12320: F, t154: F, t1995: F, t205: F, t3734: F, t12290: F, t3777: F, t12247: F, t551: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40012 = t3726 * t12313;
    let t40018 = t2559 * t3732;
    let t40019 = t40018 * t3766;
    let t40021 = t782 * t12214;
    let t40022 = t40021 * t12320;
    let t40024 = t154 * t1995;
    let t40025 = t205 * t40024;
    let t40026 = t3734 * t3734;
    let t40035 = t3777 * t12290;
    let t40041 = 1.0 / t12247 / t551;
    (t40012, t40018, t40019, t40021, t40022, t40025, t40026, t40035, t40041)
}
