//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1435/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1435<F: Float>(t5: F, t25: F, t265: F, t394: F, t108939: F, t108983: F, t109004: F, t109025: F, t112: F, t106606: F, t105830: F, t1409: F, t20217: F, t2116: F, t29507: F, t40: F, t5398: F, t7992: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t109028 = piecewise3::<f64>(t8, F::new(0.0), t108939 + t108983 + t109004 + t109025);
    let t109029 = t109028 * t112;
    let t109045 = piecewise3::<f64>(t395, F::new(0.0), t106606);
    let t109055 = piecewise3::<f64>(t115, t105830, t109045 * t40 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t29507 * t1409 + F::new(3.0) / F::new(2.0) * t7992 * t5398 + t2116 * t20217 / F::new(2.0));
    (t109029, t109055)
}
