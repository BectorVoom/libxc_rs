//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1953/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1953<F: Float>(t1858: F, t7222: F, t1396: F, t16546: F, t1852: F, t2099: F, t24486: F, t27286: F, t3932: F, t5364: F, t5381: F, t7223: F, t7240: F, t7961: F, t84031: F, t85394: F, t85397: F, t91830: F, t91832: F, t91834: F) -> F {
    let t91842 = F::new(2.0) * t7222 * t1858;
    let t91846 = F::new(2.0) * t1396 * t27286 + t16546 * t2099 + t1852 * t24486 + t3932 * t7961 + F::new(2.0) * t5364 * t7240 + F::new(2.0) * t5381 * t7223 + t84031 + t85394 + F::new(2.0) * t85397 + t91830 + t91832 + t91834 + t91842;
    t91846
}
