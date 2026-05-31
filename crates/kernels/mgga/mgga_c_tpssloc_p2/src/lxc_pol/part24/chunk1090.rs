//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1090/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1090<F: Float>(t1022: F, t607: F, t1307: F, t1388: F, t1351: F, t2319: F, t576: F, t671: F, t1874: F, t9348: F, t111: F, t6514: F) -> (F, F, F, F, F, F, F) {
    let t14227 = t607 * t1022;
    let t15904 = t1388 * t1307;
    let t16312 = t1351 * t1307;
    let t16535 = t576 * t2319;
    let t20173 = t576 * t671;
    let t22460 = F::cast_from(2.0_f64) * t9348 * t1874;
    let t22461 = t6514 * t111;
    (t14227, t15904, t16312, t16535, t20173, t22460, t22461)
}
