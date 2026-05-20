//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2006/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2006<F: Float>(t27937: F, t7032: F, t1860: F, t2031: F, t2032: F, t26028: F, t26945: F, t27979: F, t28935: F, t6486: F, t7035: F, t7428: F, t7782: F, t84285: F, t92049: F, t92056: F, t96379: F, t96383: F, t96646: F) -> F {
    let t102303 = t27937 * t7032;
    let t102305 = -F::new(2.0) / F::new(3.0) * t96646 * t2032 - F::new(2.0) / F::new(3.0) * t27979 * t7035 + t6486 * t28935 / F::new(3.0) + t1860 * t2031 * t96379 / F::new(3.0) + F::new(88.0) / F::new(27.0) * t84285 - t92049 - t92056 + t96383 * t2032 / F::new(3.0) + t27937 * t7035 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t26028 * t7782 + F::new(2.0) / F::new(3.0) * t7428 * t26945 - F::new(8.0) / F::new(9.0) * t102303;
    t102305
}
