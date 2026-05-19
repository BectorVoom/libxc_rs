//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1431/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1431<F: Float>(t22674: F, t33296: F, t6897: F, t22751: F, t33307: F, t114178: F, t115530: F, t115540: F, t115551: F, t115619: F, t120327: F, t120334: F, t120337: F, t1843: F, t22656: F, t26477: F, t31555: F, t31655: F, t5215: F, t7199: F, t7937: F, t97740: F) -> F {
    let t122247 = t6897 * t22674 * t33296;
    let t122251 = t22751 * t33307;
    let t122255 = -F::cast_from(0.19190897446562641759e-1_f64) * t115530 - t114178 + F::new(2.0) * t5215 * t31555 + F::new(2.0) * t26477 * t7199 - t115540 + F::cast_from(0.41123351671205660912e-2_f64) * t122247 - F::new(6.0) * t97740 * t31655 + t120327 + F::cast_from(0.38381794893125283518e-1_f64) * t122251 + t115551 - t22656 * t7937 - t115619 * t1843 + t120334 - t120337;
    t122255
}
