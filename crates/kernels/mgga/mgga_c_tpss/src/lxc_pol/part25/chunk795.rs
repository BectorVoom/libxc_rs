//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 795/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk795<F: Float>(t33: F, t259: F, t479: F, t5869: F, t1006: F, t1692: F, t1812: F, t1826: F, t2439: F, t5671: F, t5678: F, t57: F, t581: F, t5849: F, t5853: F, t5875: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t5889 = piecewise3(t480, 0.0, t5869);
    let t5894 = piecewise3(t386, 3.0 / 2.0 * t2439 * t1812 * t5671 + t1692 * t5849 * t33 / 2.0 - t1692 * t5853 * t5678 / 2.0 + t1692 * t1812 * t1006 / 2.0, -t1826 * t581 / 2.0 + t5889 * t57 / 2.0);
    let t5895 = t5875 + t5894;
    (t5889, t5895)
}
