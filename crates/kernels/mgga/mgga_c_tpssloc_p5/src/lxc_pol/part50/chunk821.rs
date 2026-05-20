//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 821/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk821<F: Float>(t25: F, t265: F, t394: F, t202: F, t8365: F, t8369: F, t193: F, t2752: F, t870: F, t1070: F, t3216: F, t336: F, t8409: F, t8413: F, t40: F, t8374: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t8418 = t202 * t8365;
    let t8421 = t202 * t8369;
    let t8424 = -t193 * t2752 * t8421 + t193 * t8418 * t870;
    let t8425 = piecewise3::<F>(t395, t1070 * t193 * t336 * t8409 - t193 * t3216 * t336 * t8413, t8424);
    let t8428 = piecewise3::<F>(t115, t8374, t8425 * t40 / F::new(2.0));
    (t8418, t8421, t8424, t8425, t8428)
}
