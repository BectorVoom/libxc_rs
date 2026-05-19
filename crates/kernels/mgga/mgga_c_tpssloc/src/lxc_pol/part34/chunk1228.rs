//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1228/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1228<F: Float>(t108249: F, t108268: F, t108290: F, t108309: F, t101708: F, t105661: F, t105665: F, t105669: F, t105674: F, t105685: F, t13228: F, t1499: F, t20853: F, t226: F, t235: F, t29041: F, t4281: F, t5575: F, t7101: F, t7839: F, t812: F, t84995: F, t85003: F, t85027: F, t87635: F, t87653: F, t87666: F, t87718: F, t98564: F, t98884: F) -> (F, F) {
    let t108311 = t108249 + t108268 + t108290 + t108309;
    let t108321 = F::cast_from(0.23029076935875170111e0_f64) * t98564 + F::new(6.0) * t4281 * t101708 * t13228 - t84995 - F::cast_from(0.76763589786250567036e0_f64) * t87635 - F::cast_from(0.49348022005446793095e-1_f64) * t87653 + t85003 + F::new(3.0) * t5575 * t7839 + F::cast_from(0.9869604401089358619e-1_f64) * t105661 + F::cast_from(0.19739208802178717238e0_f64) * t105665 + F::cast_from(0.9869604401089358619e-1_f64) * t105669 + F::new(3.0) * t1499 * t29041 + t226 * t235 * t108311 - t812 * t7101 * t20853 - F::cast_from(0.38381794893125283518e0_f64) * t87666 - F::cast_from(0.39478417604357434476e0_f64) * t105674 + F::cast_from(0.9869604401089358619e-1_f64) * t105685 - t85027 - F::cast_from(0.31253747270116302294e0_f64) * t87718 + F::cast_from(0.24674011002723396548e-1_f64) * t98884;
    (t108311, t108321)
}
