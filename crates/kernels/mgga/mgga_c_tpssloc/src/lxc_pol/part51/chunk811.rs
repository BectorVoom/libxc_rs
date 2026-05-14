//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 811/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk811<F: Float>(t652: F, t8533: F, t2047: F, t225: F, t258: F, t214: F, t1880: F, t8340: F, t8345: F) -> (F, F, F, F, F) {
    let t8535 = 2.0 * t652 * t8533;
    let t8537 = t2047 * t225 * t258;
    let t8538 = t214 * t8537;
    let t8539 = t1880 * t8538;
    let t8543 = 0.16149102437656156341e-2 * t8340 + t8345 / 768.0;
    (t8535, t8537, t8538, t8539, t8543)
}
