//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1924/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1924<F: Float>(t16759: F, t1888: F, t6646: F, t17030: F, t22986: F, t2647: F, t17046: F, t1510: F, t87130: F, t25249: F, t4234: F, t23110: F, t28337: F, t81651: F) -> (F, F, F, F, F, F) {
    let t98428 = t1888 * t6646 * t16759;
    let t98432 = t22986 * t6646 * t17030 * t2647;
    let t98435 = t1888 * t6646 * t17046;
    let t98439 = t22986 * t6646 * t87130 * t1510;
    let t98443 = t22986 * t6646 * t25249 * t4234;
    let t98446 = t81651 * t23110 * t28337;
    (t98428, t98432, t98435, t98439, t98443, t98446)
}
