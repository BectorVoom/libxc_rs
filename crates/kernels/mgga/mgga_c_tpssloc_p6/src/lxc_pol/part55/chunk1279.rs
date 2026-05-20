//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1279/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1279<F: Float>(t24721: F, t7337: F, t8039: F, t118002: F, t118005: F, t118007: F, t119243: F, t125453: F, t2134: F, t27580: F, t27654: F, t27704: F, t27714: F, t32428: F, t32429: F, t34260: F, t4973: F, t7316: F, t8031: F, t8875: F) -> F {
    let t125474 = t24721 * t7337 * t8039;
    let t125482 = F::cast_from(0.40372756094140390856e-3_f64) * t8031 * t32429 - F::cast_from(0.40372756094140390856e-3_f64) * t2134 * t27654 * t32428 + F::cast_from(0.40372756094140390856e-3_f64) * t7316 * t34260 - t125453 * t119243 * t4973 / F::new(1152.0) + F::cast_from(0.32298204875312312685e-2_f64) * t27580 * t8875 + F::cast_from(0.40372756094140390856e-3_f64) * t125474 - F::cast_from(0.40372756094140390856e-3_f64) * t27714 * t8875 + t118002 / F::new(2304.0) - t118005 - F::cast_from(0.40372756094140390856e-3_f64) * t118007 - F::cast_from(0.40372756094140390856e-3_f64) * t27704 * t32429;
    t125482
}
