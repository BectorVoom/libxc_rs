//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 126/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk126<F: Float>(t349: F, t381: F, t362: F, t68: F, t353: F, t254: F, t193: F, t293: F, t328: F, t330: F, t336: F, t265: F) -> (F, F, F, F, F, F, F) {
    let t382 = t349 * t381;
    let t383 = t68 * t362;
    let t384 = t383 * t381;
    let t386 = t353 * t384 + F::new(1.0);
    let t387 = F::new(1.0) / t386;
    let t388 = t254 * t387;
    let t390 = t382 * t388 + F::new(1.0);
    let t391 = f64::ln(t390);
    let t394 = t193 * t336 * t391 - t293 + t328 + t330;
    let t395 = t265 < t394;
    (t382, t383, t384, t386, t388, t390, t394)
}
