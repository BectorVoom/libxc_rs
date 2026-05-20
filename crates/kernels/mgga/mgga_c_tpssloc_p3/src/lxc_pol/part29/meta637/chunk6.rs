//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2097/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2097<F: Float>(t225: F, t25051: F, t23012: F, t7489: F, t82120: F, t13460: F, t1880: F, t6553: F, t6571: F, t1527: F, t23190: F, t25160: F, t259: F, t2591: F, t2718: F, t7510: F, t798: F, t82108: F, t82115: F, t82123: F, t855: F, t866: F, t86983: F) -> F {
    let t86988 = t25051 * t225;
    let t86991 = t23012 * t7489;
    let t86994 = F::cast_from(0.3289868133696452873e-1_f64) * t82120;
    let t86997 = t1880 * t6553 * t6571 * t13460;
    let t87005 = t86983 + F::new(2.0) * t798 * t25160 * t259 - F::cast_from(0.24674011002723396547e-1_f64) * t82108 - F::new(2.0) * t86988 * t866 - F::cast_from(0.63969658155208805863e-1_f64) * t86991 - F::cast_from(0.76763589786250567036e-1_f64) * t82115 + t86994 - t82123 - F::cast_from(0.82246703342411321825e-2_f64) * t86997 + F::new(2.0) * t855 * t2718 * t23190 * t1527 + t2591 * t7510 * t259;
    t87005
}
