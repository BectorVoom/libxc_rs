//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1521/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1521<F: Float>(t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13557: F, t13561: F, t13642: F, t13647: F, t13921: F, t13922: F, t13923: F) -> F {
    let t13931 = t10295 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t10296 - t10298 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10300 - t10302 / F::cast_from(9.0_f64) + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t13642 - t13921 + t13922 - t13923 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t13539 - t13557 / F::cast_from(3.0_f64) + t13530 / F::cast_from(9.0_f64) + t13534 / F::cast_from(18.0_f64) + t13561 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13544 - t13548 / F::cast_from(3.0_f64) + t13647 / F::cast_from(6.0_f64);
    t13931
}
