//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 674/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk674<F: Float>(t109: F, t107: F, t240: F, t625: F, t656: F, t666: F, t2331: F, t63: F, t2332: F, t2358: F, t6530: F) -> (F, F, F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t22468 = t240 * t107;
    let t22469 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t22468;
    let t22470 = t625 * t656;
    let t22471 = t22470 * t666;
    let t22472 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t22471;
    let t22473 = t63 * t2331;
    let t22474 = t22473 * t2332;
    let t22476 = t6530 * t2358;
    let t22479 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t22469 + t22472 + t22474 / F::cast_from(4.0_f64) - t22476 / F::cast_from(8.0_f64));
    (t22468, t22471, t22474, t22476, t22479)
}
