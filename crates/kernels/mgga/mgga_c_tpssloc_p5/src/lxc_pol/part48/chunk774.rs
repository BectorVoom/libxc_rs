//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 774/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk774<F: Float>(t22990: F, t23000: F, t23002: F, t23006: F, t23022: F, t23026: F, t23028: F, t23038: F, t24246: F, t24250: F, t24251: F, t24256: F, t2617: F, t7102: F, t812: F) -> F {
    let t24260 = F::cast_from(0.6579736267392905746e-1_f64) * t22990 + F::cast_from(0.3289868133696452873e-1_f64) * t23000 + F::cast_from(0.76763589786250567036e-1_f64) * t23002 - F::cast_from(0.16449340668482264365e-1_f64) * t23006 + t24246 + F::cast_from(0.16449340668482264365e-1_f64) * t23022 - F::cast_from(0.16449340668482264365e-1_f64) * t23026 - F::cast_from(0.76763589786250567036e-1_f64) * t23028 + t24250 - t812 * t24251 - F::cast_from(2.0_f64) * t2617 * t7102 + F::cast_from(2.0_f64) * t812 * t24256 + F::cast_from(0.9869604401089358619e-1_f64) * t23038;
    t24260
}
