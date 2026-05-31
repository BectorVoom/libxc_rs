//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 826/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk826<F: Float>(t25: F, t265: F, t394: F, t652: F, t8675: F, t8424: F, t40: F, t8374: F, t1873: F, t7266: F, t191: F, t2167: F, t192: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t8676 = t652 * t8675;
    let t8678 = piecewise3::<F>(t395, F::cast_from(0.0_f64), t8424);
    let t8681 = piecewise3::<F>(t115, t8374, t8678 * t40 / F::cast_from(2.0_f64));
    let t8684 = t7266 * t1873;
    let t8689 = t2167 * t191;
    let t8690 = t8689 * t192;
    (t8676, t8678, t8681, t8684, t8689, t8690)
}
