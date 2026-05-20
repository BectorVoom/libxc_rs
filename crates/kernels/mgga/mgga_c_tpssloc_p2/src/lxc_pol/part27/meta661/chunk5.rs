//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2319/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2319<F: Float>(t26421: F, t26446: F, t3734: F, t90591: F, t1336: F, t22710: F, t22874: F, t22877: F, t26403: F, t26456: F, t26458: F, t3777: F, t3793: F, t3851: F, t3856: F, t5234: F, t5250: F, t5334: F, t5344: F, t81160: F, t81184: F, t81189: F, t90946: F, t91025: F, t91029: F, t91036: F, t91043: F, t91045: F, t91048: F) -> F {
    let t91052 = t90591 * t26446 * t26421 * t3734;
    let t91059 = -F::cast_from(0.76763589786250567036e-1_f64) * t81160 - F::cast_from(0.38381794893125283518e-1_f64) * t81184 - F::cast_from(0.16449340668482264365e-1_f64) * t91025 + F::new(2.0) * t5234 * t22710 + F::new(2.0) * t1336 * t91029 * t3793 - F::new(2.0) * t3777 * t26456 + F::cast_from(0.3289868133696452873e-1_f64) * t91036 + F::new(4.0) * t5334 * t90946 * t5250 - t1336 * t26458 * t3856 - t91043 + F::cast_from(0.76763589786250567036e-1_f64) * t81189 + t91045 + F::cast_from(0.9869604401089358619e-1_f64) * t91048 - F::cast_from(0.19739208802178717238e0_f64) * t91052 - t5344 * t26403 * t3851 - F::new(2.0) * t5234 * t22874 - t5234 * t22877;
    t91059
}
