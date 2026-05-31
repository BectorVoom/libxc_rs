//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1226/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1226<F: Float>(t33: F, t259: F, t479: F, t18847: F, t1826: F, t18887: F, t1992: F, t57: F, t581: F, t5889: F, t18855: F, t116: F, t5798: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t18888 = piecewise3::<F>(t480, F::cast_from(0.0_f64), t18847);
    let t18895 = piecewise3::<F>(t386, t18887, t18888 * t57 / F::cast_from(2.0_f64) - t5889 * t581 - t1826 * t1992 / F::cast_from(2.0_f64));
    let t18896 = t18855 + t18895;
    let t18898 = t5798 * t116;
    (t18888, t18896, t18898)
}
