//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 495/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk495<F: Float>(t2148: F, t462: F, t2144: F, t493: F, t2121: F, t470: F) -> (F, F, F) {
    let t2149 = t462 * t2148;
    let t2152 = t493 * t2144;
    let t2154 = F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t2149 + t470 * t2152;
    (t2149, t2152, t2154)
}
