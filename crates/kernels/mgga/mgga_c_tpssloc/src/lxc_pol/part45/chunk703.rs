//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 703/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk703<F: Float>(t1307: F, t1377: F, t1385: F, t22635: F, t22633: F, t154: F, t835: F, t3748: F) -> (F, F, F, F) {
    let t22637 = t1377 * t1307 * t1385;
    let t22638 = t22635 * t22637;
    let t22639 = t22633 * t22638;
    let t22641 = t835 * t154;
    let t22642 = t22641 * t3748;
    (t22637, t22639, t22641, t22642)
}
