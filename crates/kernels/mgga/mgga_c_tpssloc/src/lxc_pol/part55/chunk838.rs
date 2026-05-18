//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 838/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk838<F: Float>(t28: F, t265: F, t504: F, t1256: F, t193: F, t336: F, t3640: F, t8424: F, t8900: F, t8904: F, t52: F, t8434: F, t8681: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8909 = piecewise3::<f64>(t505, t1256 * t193 * t336 * t8900 - t193 * t336 * t3640 * t8904, t8424);
    let t8912 = piecewise3::<f64>(t401, t8434, t8909 * t52 / F::new(2.0));
    let t8913 = t8681 + t8912;
    (t8909, t8913)
}
