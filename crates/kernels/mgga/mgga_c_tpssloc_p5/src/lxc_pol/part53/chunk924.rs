//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 924/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk924<F: Float>(t25: F, t265: F, t394: F, t34030: F, t1408: F, t1409: F, t1877: F, t2522: F, t32034: F, t32047: F, t33991: F, t34004: F, t40: F, t7114: F, t7475: F, t7545: F, t8744: F, t8748: F, t8760: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t34031 = piecewise3::<F>(t395, F::cast_from(0.0_f64), t34030);
    let t34036 = piecewise3::<F>(t115, F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8744 * t7475 + t1877 * t33991 * t25 / F::cast_from(2.0_f64) - t1877 * t32034 * t7545 / F::cast_from(2.0_f64) + t1877 * t8744 * t1408 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8748 * t7475 - t1877 * t7114 * t34004 + t1877 * t32047 * t7545 - t1877 * t8748 * t1408 / F::cast_from(2.0_f64), t8760 * t1409 / F::cast_from(2.0_f64) + t34031 * t40 / F::cast_from(2.0_f64));
    (t34031, t34036)
}
