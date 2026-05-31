//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1020/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1020<F: Float>(t113836: F, t113875: F, t116088: F, t116096: F, t116099: F, t116111: F, t116115: F, t116119: F, t117499: F, t117503: F, t117516: F, t117518: F, t117527: F, t2250: F, t2303: F, t31864: F, t32331: F, t32333: F, t32338: F, t63: F, t641: F, t8308: F, t8513: F, t8663: F, t8825: F) -> F {
    let t117528 = -F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8663 * t8513 * t32338 * t2303 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t116096 * t8825 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t116099 * t8825 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t117499 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t116111 * t32333 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t116115 * t113875 * t117503 * t641 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t116119 * t32333 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31864 * t8308 * t32331 * t2250 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t116088 * t8825 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t117516 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t117518 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8663 * t8513 * t113836 * t63 - t117527;
    t117528
}
