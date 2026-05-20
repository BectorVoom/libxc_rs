//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2039/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2039<F: Float>(t1398: F, t1404: F, t16507: F, t1858: F, t2105: F, t24448: F, t27241: F, t3: F, t3946: F, t580: F, t7946: F, t85379: F, t85381: F, t85392: F, t94106: F, t94113: F, t94118: F, t94120: F, t94122: F, t94160: F, t94202: F) -> F {
    let t94205 = t3 * t94106 * t580 + F::new(2.0) * t27241 * t1404 + t7946 * t3946 + t94113 + t24448 * t1858 + t85392 + t16507 * t2105 + t85379 + F::new(2.0) * t85381 + t94118 + t94120 + t94122 + t1398 * (t94160 + t94202);
    t94205
}
