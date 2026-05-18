//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 925/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk925<F: Float>(t28: F, t265: F, t504: F, t7844: F, t34030: F, t1409: F, t1649: F, t1877: F, t2522: F, t32034: F, t32047: F, t33991: F, t52: F, t7114: F, t7649: F, t7656: F, t8744: F, t8748: F, t8770: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t34052 = t28 * t7844;
    let t34061 = piecewise3::<f64>(t505, F::new(0.0), t34030);
    let t34066 = piecewise3::<f64>(t401, F::new(3.0) / F::new(2.0) * t2522 * t8744 * t7649 + t1877 * t33991 * t28 / F::new(2.0) - t1877 * t32034 * t7656 / F::new(2.0) + t1877 * t8744 * t1649 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t2522 * t8748 * t7649 - t1877 * t7114 * t34052 + t1877 * t32047 * t7656 - t1877 * t8748 * t1649 / F::new(2.0), -t8770 * t1409 / F::new(2.0) + t34061 * t52 / F::new(2.0));
    (t34052, t34061, t34066)
}
