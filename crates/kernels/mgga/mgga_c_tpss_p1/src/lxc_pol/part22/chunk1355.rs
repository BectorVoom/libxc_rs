//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1355/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1355<F: Float>(t30: F, t259: F, t379: F, t66704: F, t66750: F, t10353: F, t1289: F, t1819: F, t18848: F, t1992: F, t20577: F, t3431: F, t45: F, t581: F, t5870: F, t6374: F, t66266: F, t66302: F, t66618: F, t66656: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t66751 = t66704 + t66750;
    let t66752 = piecewise3::<F>(t380, F::cast_from(0.0_f64), t66751);
    let t66764 = piecewise3::<F>(t120, t66266 + t66302 + t66618 + t66656, t66752 * t45 / F::cast_from(2.0_f64) + t20577 * t581 + t6374 * t1992 / F::cast_from(2.0_f64) + t18848 * t1289 / F::cast_from(2.0_f64) + t5870 * t3431 + t1819 * t10353 / F::cast_from(2.0_f64));
    (t66751, t66764)
}
