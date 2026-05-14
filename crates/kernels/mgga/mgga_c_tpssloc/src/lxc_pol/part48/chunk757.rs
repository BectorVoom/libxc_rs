//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 757/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk757<F: Float>(t22574: F, t31300: F, t191: F, t192: F, t7166: F, t2020: F, t6997: F, t8607: F, t8562: F, t865: F, t2718: F, t225: F, t258: F, t7084: F, t214: F, t1880: F) -> (F, F, F, F, F, F, F, F) {
    let t31302 = 3.0 * t22574 * t31300;
    let t31304 = t7166 * t191 * t192;
    let t31305 = t31304 * t2020;
    let t31306 = t8607 * t6997;
    let t31310 = t8562 * t865;
    let t31311 = t2718 * t31310;
    let t31315 = t7084 * t225 * t258;
    let t31316 = t214 * t31315;
    let t31317 = t1880 * t31316;
    (t31302, t31304, t31305, t31306, t31311, t31315, t31316, t31317)
}
