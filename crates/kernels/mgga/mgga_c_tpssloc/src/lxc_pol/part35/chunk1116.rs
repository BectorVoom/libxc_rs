//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1116/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1116<F: Float>(t25: F, t265: F, t394: F, t28755: F, t1409: F, t2116: F, t28469: F, t40: F, t5398: F, t7992: F, t1760: F, t8087: F, t3598: F, t2154: F, t6267: F, t7301: F, t7300: F, t2123: F, t6140: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t29507 = piecewise3(t395, 0.0, t28755);
    let t29514 = piecewise3(t115, t28469, t29507 * t40 / 2.0 + t7992 * t1409 + t2116 * t5398 / 2.0);
    let t29531 = t8087 * t1760;
    let t29532 = t3598 * t29531;
    let t29535 = t2154 * t6267;
    let t29536 = t3598 * t29535;
    let t29545 = t7301 * t6267;
    let t29546 = t7300 * t29545;
    let t29551 = t6140 * t2123;
    (t29507, t29514, t29532, t29536, t29545, t29546, t29551)
}
