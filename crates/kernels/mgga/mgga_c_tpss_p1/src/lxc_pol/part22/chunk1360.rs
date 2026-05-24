//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1360/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1360<F: Float>(t33: F, t259: F, t479: F, t66751: F, t10353: F, t1289: F, t1826: F, t18888: F, t1992: F, t20632: F, t3431: F, t57: F, t581: F, t5889: F, t6393: F, t66796: F, t66833: F, t66870: F, t66897: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t66900 = piecewise3::<F>(t480, F::new(0.0), t66751);
    let t66912 = piecewise3::<F>(t386, t66796 + t66833 + t66870 + t66897, t66900 * t57 / F::new(2.0) - t20632 * t581 - t6393 * t1992 / F::new(2.0) - t18888 * t1289 / F::new(2.0) - t5889 * t3431 - t1826 * t10353 / F::new(2.0));
    t66912
}
