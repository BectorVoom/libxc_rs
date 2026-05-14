//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 653/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk653<F: Float>(t1510: F, t7101: F, t235: F, t7823: F, t1499: F, t2051: F, t226: F, t7095: F, t7097: F, t7522: F, t7526: F, t7530: F, t812: F, t858: F, t1528: F, t2054: F, t259: F, t4147: F, t4268: F, t7067: F, t7069: F, t7087: F, t7481: F, t7486: F, t7490: F, t7815: F, t7824: F, t7830: F, t855: F) -> (F, F, F, F, F) {
    let t7837 = t7101 * t1510;
    let t7839 = t235 * t7823;
    let t7841 = -t7095 - 0.3289868133696452873e-1 * t7522 - t7097 - 0.16449340668482264365e-1 * t7526 + 0.16449340668482264365e-1 * t7530 + t1499 * t2051 - t812 * t7837 + t226 * t7839;
    let t7842 = t858 * t7841;
    let t7844 = -t7067 - 0.3289868133696452873e-1 * t7481 - t7069 + 0.16449340668482264365e-1 * t7486 - 0.16449340668482264365e-1 * t7490 + t7815 * t259 + t7824 * t259 - t7087 * t1528 - t4147 * t2054 - t4268 * t2054 + 2.0 * t855 * t7830 - t855 * t7842;
    (t7837, t7839, t7841, t7842, t7844)
}
