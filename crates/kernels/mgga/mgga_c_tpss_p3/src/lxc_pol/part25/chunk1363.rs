//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1363/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1363<F: Float>(t71970: F, t72026: F, t72129: F, t72170: F, t823: F, t20526: F, t69855: F, t198: F, t6368: F, t1692: F, t1812: F, t18728: F, t18807: F, t19819: F, t19821: F, t19825: F, t20417: F, t20510: F, t20514: F, t21356: F, t21659: F, t2439: F, t30: F, t3552: F, t5539: F, t6120: F, t69800: F, t69838: F, t69864: F, t70241: F, t70244: F, t70290: F) -> (F, F, F, F, F) {
    let t72172 = t71970 + t72026 + t72129 + t72170;
    let t72173 = t72172 * t823;
    let t72187 = F::new(2.0) * t20526 * t69855;
    let t72188 = t198 * t6368;
    let t72203 = F::new(3.0) * t3552 * t1812 * t69838 + t1692 * t72173 * t30 / F::new(2.0) - t1692 * t20514 * t19825 + F::new(3.0) * t2439 * t20510 * t6120 - F::new(3.0) * t18728 * t70290 - F::new(3.0) / F::new(2.0) * t18728 * t69864 - t72187 + F::new(2.0) * t72188 * t19819 - t1692 * t20514 * t19821 - t1692 * t18807 * t21356 - F::new(3.0) * t20526 * t70244 + t20526 * t70241 - F::new(6.0) * t20417 * t69800 + F::new(3.0) / F::new(2.0) * t2439 * t21659 * t5539;
    (t72172, t72173, t72187, t72188, t72203)
}
