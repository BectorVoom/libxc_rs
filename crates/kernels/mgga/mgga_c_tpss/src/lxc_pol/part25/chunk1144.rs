//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1144/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1144<F: Float>(t30: F, t259: F, t379: F, t20576: F, t1289: F, t1819: F, t20545: F, t3431: F, t45: F, t581: F, t5870: F, t6374: F, t1006: F, t1497: F, t1692: F, t1812: F, t18728: F, t18807: F, t20012: F, t20018: F, t20021: F, t20025: F, t20041: F, t20048: F, t20050: F, t20054: F, t20058: F, t20065: F, t20417: F, t20510: F, t20514: F, t20526: F, t20544: F, t2439: F, t33: F, t5671: F, t5678: F, t5849: F, t5853: F, t6207: F, t6214: F, t6354: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t20577 = piecewise3(t380, 0.0, t20576);
    let t20584 = piecewise3(t120, t20545, t5870 * t1289 / 2.0 + t1819 * t3431 / 2.0 + t20577 * t45 / 2.0 + t6374 * t581 / 2.0);
    let t20631 = 3.0 * t20417 * t20012 + 3.0 / 2.0 * t2439 * t5849 * t6207 - 3.0 / 2.0 * t18728 * t20018 + 3.0 / 2.0 * t2439 * t1812 * t20021 + 3.0 / 2.0 * t2439 * t1812 * t20025 + 3.0 / 2.0 * t2439 * t6354 * t5671 + t1692 * t20510 * t33 / 2.0 - t1692 * t20514 * t5678 / 2.0 + t1692 * t6354 * t1006 / 2.0 - 3.0 / 2.0 * t18728 * t20041 - t1692 * t18807 * t6214 / 2.0 + t20526 * t20048 - t1692 * t5853 * t20050 / 2.0 - t1692 * t5853 * t20054 / 2.0 + 3.0 / 2.0 * t2439 * t1812 * t20058 + t1692 * t5849 * t1497 / 2.0 - t1692 * t5853 * t20065 / 2.0 - t20544;
    (t20577, t20584, t20631)
}
