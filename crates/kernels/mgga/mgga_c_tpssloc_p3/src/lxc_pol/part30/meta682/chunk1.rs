//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2149/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2149<F: Float>(t1336: F, t22873: F, t28171: F, t28174: F, t3777: F, t5230: F, t6420: F, t7747: F, t91002: F, t91011: F, t93605: F, t93615: F, t97119: F, t97124: F, t97129: F, t97135: F, t97137: F, t97142: F, t97146: F, t97148: F, t97152: F) -> F {
    let t97154 = -t93605 + F::cast_from(0.3289868133696452873e-1_f64) * t97119 - t3777 * t28174 - t1336 * t22873 * t6420 - F::cast_from(0.76763589786250567037e-1_f64) * t97124 - F::cast_from(0.16449340668482264365e-1_f64) * t97129 + F::new(2.0) * t5230 * t7747 + F::cast_from(0.9869604401089358619e-1_f64) * t97135 + F::cast_from(0.38381794893125283518e-1_f64) * t97137 + F::new(2.0) * t3777 * t28171 + F::cast_from(0.41123351671205660912e-2_f64) * t97142 - t93615 - F::cast_from(0.23029076935875170111e0_f64) * t91002 - F::cast_from(0.6579736267392905746e-1_f64) * t97146 + F::cast_from(0.19190897446562641759e-1_f64) * t97148 + F::cast_from(0.16449340668482264365e-1_f64) * t97152 + t91011;
    t97154
}
