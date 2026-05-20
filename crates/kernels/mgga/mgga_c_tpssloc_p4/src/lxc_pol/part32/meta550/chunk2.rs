//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1907/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1907<F: Float>(t1458: F, t24932: F, t26109: F, t26111: F, t26113: F, t26116: F, t26119: F, t26121: F, t26123: F, t26125: F, t26137: F, t27371: F, t27863: F, t27888: F, t4072: F, t671: F, t7266: F) -> F {
    let t27903 = F::new(2.0) * t1458 * t24932 + F::new(2.0) * t1458 * t27888 + F::new(2.0) * t27863 * t671 + F::new(2.0) * t4072 * t7266 + t26109 + t26111 + t26113 + t26116 + t26119 + t26121 + t26123 + t26125 + t26137 + t27371;
    t27903
}
