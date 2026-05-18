//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1022/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1022<F: Float>(t111: F, t32348: F, t114387: F, t114388: F, t114405: F, t114413: F, t114415: F, t115821: F, t2039: F, t2363: F, t23917: F, t24932: F, t27888: F, t32350: F, t671: F, t7056: F, t7266: F, t85428: F, t94248: F, t96222: F) -> (F, F) {
    let t117533 = t32348 * t111;
    let t117550 = F::new(4.0) * t117533 * t671 + F::new(2.0) * t2039 * t85428 + F::new(2.0) * t2039 * t94248 + F::new(4.0) * t2039 * t96222 + F::new(2.0) * t2363 * t32350 + F::new(2.0) * t23917 * t7266 + F::new(4.0) * t24932 * t7056 + F::new(4.0) * t27888 * t7056 + t114387 + t114388 + t114405 + t114413 + t114415 + t115821;
    (t117533, t117550)
}
