//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1470/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1470<F: Float>(t120067: F, t121195: F, t121197: F, t121199: F, t121201: F, t121203: F, t121204: F, t121211: F, t121224: F, t123194: F, t26559: F, t31055: F, t31057: F, t31060: F, t31832: F, t7943: F) -> F {
    let t124951 = F::new(2.0) * t123194 * t26559 - t31832 * t7943 - t120067 - t121195 - t121197 - t121199 - t121201 + t121203 - t121204 + t121211 - t121224 - t31055 - t31057 - t31060;
    t124951
}
