//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1887/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1887<F: Float>(t1339: F, t26288: F, t550: F, t57172: F, t22827: F, t74366: F, t1307: F, t6415: F, t6420: F, t1825: F, t5286: F, t6936: F) -> (F, F, F, F, F) {
    let t97287 = t26288 * t1339 * t57172 * t550;
    let t97291 = t22827 * t1339 * t74366 * t550;
    let t97295 = t22827 * t1339 * t6415 * t1307;
    let t97299 = t22827 * t1339 * t6420 * t1307;
    let t97303 = t6936 * t1339 * t1825 * t5286;
    (t97287, t97291, t97295, t97299, t97303)
}
