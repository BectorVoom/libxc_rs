//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1233/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1233<F: Float>(t25: F, t265: F, t394: F, t33512: F, t1408: F, t1409: F, t1877: F, t24191: F, t2522: F, t26744: F, t26756: F, t31434: F, t32899: F, t33466: F, t33477: F, t33484: F, t33486: F, t40: F, t7114: F, t7475: F, t7545: F, t8566: F, t8569: F, t8580: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t33513 = piecewise3::<f64>(t395, F::new(0.0), t33512);
    let t33518 = piecewise3::<f64>(t115, F::new(3.0) / F::new(2.0) * t2522 * t8566 * t7475 + t1877 * t33466 * t25 / F::new(2.0) - t1877 * t31434 * t7545 / F::new(2.0) + t1877 * t8566 * t1408 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t24191 * t33477 - t1877 * t26744 * t8569 / F::new(2.0) + t26756 * t33484 - t1877 * t7114 * t33486 / F::new(2.0) - t1877 * t7114 * t32899 / F::new(2.0), t8580 * t1409 / F::new(2.0) + t33513 * t40 / F::new(2.0));
    (t33513, t33518)
}
