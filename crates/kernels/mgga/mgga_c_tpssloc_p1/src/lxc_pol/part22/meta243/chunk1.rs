//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1338/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1338<F: Float>(t10375: F, t370: F, t3158: F, t964: F, t10335: F, t221: F, t339: F, t3069: F, t3180: F) -> (F, F, F, F, F) {
    let t10377 = t370 * t10375 / F::new(10368.0);
    let t10381 = t964 * t3158;
    let t10383 = t221 * t10335;
    let t10385 = F::new(5.0) / F::new(1296.0) * t339 * t10383;
    let t10390 = t3180 * t3069;
    (t10377, t10381, t10383, t10385, t10390)
}
