//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1720/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1720<F: Float>(t1388: F, t1799: F, t3792: F, t5286: F, t576: F, t671: F, t1874: F, t9348: F, t111: F, t6514: F) -> (F, F, F, F, F) {
    let t19577 = t1799 * t1388;
    let t19735 = t3792 * t5286;
    let t20173 = t576 * t671;
    let t22460 = F::new(2.0) * t9348 * t1874;
    let t22461 = t6514 * t111;
    (t19577, t19735, t20173, t22460, t22461)
}
