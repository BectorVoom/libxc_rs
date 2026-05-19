//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 432/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk432<F: Float>(t1949: F, t345: F, t1945: F, t383: F, t1920: F, t353: F) -> (F, F, F) {
    let t1950 = t345 * t1949;
    let t1953 = t383 * t1945;
    let t1955 = F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t1950 + t353 * t1953;
    (t1950, t1953, t1955)
}
