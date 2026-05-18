//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1286/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1286<F: Float>(t1458: F, t576: F, t9365: F, t99: F, t64: F, t2331: F, t106: F, t9364: F, t111: F, t5363: F, t6470: F, t19449: F) -> (F, F, F, F, F, F, F) {
    let t33185 = t576 * t1458;
    let t35655 = t9365 * t99;
    let t35656 = t64 * t35655;
    let t35662 = t2331 * t99;
    let t35663 = t64 * t35662;
    let t45435 = F::new(1.0) / t9364 / t106;
    let t55353 = t5363 * t111;
    let t55388 = t6470 * t111;
    let t55943 = t19449 * t111;
    (t33185, t35656, t35663, t45435, t55353, t55388, t55943)
}
