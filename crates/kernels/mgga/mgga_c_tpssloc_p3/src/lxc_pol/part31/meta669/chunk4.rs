//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1981/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1981<F: Float>(t16753: F, t26662: F, t4166: F, t5575: F, t7101: F, t7104: F, t812: F, t87167: F, t87177: F, t92551: F, t92556: F, t92560: F, t92561: F, t92564: F, t92565: F, t98505: F, t98513: F, t98516: F, t98520: F, t98530: F, t98534: F) -> F {
    let t101687 = t87167 + F::cast_from(0.76763589786250567037e-1_f64) * t98505 + F::cast_from(0.9869604401089358619e-1_f64) * t98513 - F::cast_from(0.49348022005446793095e-1_f64) * t98516 - F::cast_from(0.6579736267392905746e-1_f64) * t98520 + F::cast_from(0.3289868133696452873e-1_f64) * t87177 - t92551 + t92556 - F::cast_from(0.16449340668482264365e-1_f64) * t98530 + t92560 + t5575 * t7104 + F::cast_from(0.3289868133696452873e-1_f64) * t98534 + t92561 - t92564 - t92565 - t812 * t7101 * t16753 - F::cast_from(2.0_f64) * t4166 * t26662;
    t101687
}
