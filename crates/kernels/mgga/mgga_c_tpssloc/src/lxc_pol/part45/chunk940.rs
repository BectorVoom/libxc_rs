//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 940/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk940<F: Float>(t23012: F, t8332: F, t1888: F, t30634: F, t82159: F, t8336: F, t6665: F, t776: F, t22960: F, t606: F, t23285: F, t25: F) -> (F, F, F, F, F, F, F) {
    let t113038 = F::new(0.12793931631041761173e0) * t23012 * t8332;
    let t113041 = F::new(0.6579736267392905746e-1) * t1888 * t82159 * t30634;
    let t113045 = F::new(0.12793931631041761173e0) * t23012 * t8336;
    let t113069 = t776 * t6665;
    let t113070 = t22960 * t113069;
    let t113086 = t606 * t6665;
    let t113114 = t25 * t23285;
    (t113038, t113041, t113045, t113069, t113070, t113086, t113114)
}
