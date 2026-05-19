//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1324/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1324<F: Float>(t22986: F, t23270: F, t25191: F, t5657: F, t1528: F, t17052: F, t17090: F, t21050: F, t21053: F, t25168: F, t25169: F, t259: F, t28307: F, t28432: F, t4147: F, t4268: F, t5558: F, t6627: F, t7510: F, t7517: F, t7538: F, t98239: F, t98941: F, t98966: F, t98983: F) -> (F, F) {
    let t105474 = t22986 * t23270 * t25191 * t5657;
    let t105508 = -F::cast_from(0.23029076935875170111e0_f64) * t98941 - F::new(18.0) * t25168 * t25169 * t21053 + F::new(3.0) * t5558 * t7510 * t259 - F::new(3.0) * t4147 * t28432 - F::cast_from(0.24674011002723396548e-1_f64) * t98966 + F::new(6.0) * t17090 * t7517 - F::new(3.0) * t17052 * t7538 - F::new(6.0) * t98239 * t1528 - F::new(6.0) * t6627 * t21050 + F::cast_from(0.12337005501361698274e-1_f64) * t98983 + F::new(12.0) * t4268 * t28307;
    (t105474, t105508)
}
