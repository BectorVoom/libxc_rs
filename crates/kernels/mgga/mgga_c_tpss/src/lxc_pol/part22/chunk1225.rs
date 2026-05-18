//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1225/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1225<F: Float>(t30: F, t259: F, t379: F, t18847: F, t1819: F, t18823: F, t1992: F, t45: F, t581: F, t5870: F, t1006: F, t1692: F, t1812: F, t18239: F, t18247: F, t18250: F, t18254: F, t18265: F, t18268: F, t18271: F, t18728: F, t18803: F, t18807: F, t18812: F, t2439: F, t2829: F, t33: F, t3552: F, t5671: F, t5678: F, t5849: F, t5853: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t18848 = piecewise3::<f64>(t380, F::new(0.0), t18847);
    let t18855 = piecewise3::<f64>(t120, t18823, t18848 * t45 / F::new(2.0) + t5870 * t581 + t1819 * t1992 / F::new(2.0));
    let t18887 = F::new(3.0) * t3552 * t1812 * t18239 + F::new(3.0) * t2439 * t5849 * t5671 - F::new(3.0) * t18728 * t18247 + F::new(3.0) * t2439 * t1812 * t18250 + F::new(3.0) / F::new(2.0) * t2439 * t1812 * t18254 + t1692 * t18803 * t33 / F::new(2.0) - t1692 * t18807 * t5678 + t1692 * t5849 * t1006 + t1692 * t18812 * t18265 - t1692 * t5853 * t18268 - t1692 * t5853 * t18271 / F::new(2.0) + t1692 * t1812 * t2829 / F::new(2.0);
    (t18848, t18855, t18887)
}
