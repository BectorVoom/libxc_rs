//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 617/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk617<F: Float>(t1528: F, t2054: F, t259: F, t4147: F, t4268: F, t7067: F, t7069: F, t7087: F, t7481: F, t7486: F, t7490: F, t7815: F, t7824: F, t7830: F, t7842: F, t855: F) -> F {
    let t7844 = -t7067 - F::new(0.3289868133696452873e-1) * t7481 - t7069 + F::new(0.16449340668482264365e-1) * t7486 - F::new(0.16449340668482264365e-1) * t7490 + t7815 * t259 + t7824 * t259 - t7087 * t1528 - t4147 * t2054 - t4268 * t2054 + F::new(2.0) * t855 * t7830 - t855 * t7842;
    t7844
}
