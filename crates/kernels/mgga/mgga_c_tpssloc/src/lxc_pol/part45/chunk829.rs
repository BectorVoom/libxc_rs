//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 829/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk829<F: Float>(t22990: F, t23000: F, t23002: F, t23006: F, t23022: F, t23026: F, t23028: F, t23038: F, t24246: F, t24250: F, t24251: F, t24256: F, t2617: F, t7102: F, t812: F) -> F {
    let t24260 = F::new(0.6579736267392905746e-1) * t22990 + F::new(0.3289868133696452873e-1) * t23000 + F::new(0.76763589786250567036e-1) * t23002 - F::new(0.16449340668482264365e-1) * t23006 + t24246 + F::new(0.16449340668482264365e-1) * t23022 - F::new(0.16449340668482264365e-1) * t23026 - F::new(0.76763589786250567036e-1) * t23028 + t24250 - t812 * t24251 - F::new(2.0) * t2617 * t7102 + F::new(2.0) * t812 * t24256 + F::new(0.9869604401089358619e-1) * t23038;
    t24260
}
