//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1142/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1142<F: Float>(t1692: F, t1812: F, t1989: F, t1288: F, t18728: F, t18807: F, t19672: F, t19678: F, t19681: F, t19685: F, t19810: F, t19819: F, t19821: F, t19825: F, t19829: F, t19836: F, t20417: F, t20510: F, t20514: F, t20526: F, t2439: F, t30: F, t5539: F, t5591: F, t580: F, t5849: F, t5853: F, t6120: F, t6153: F, t6354: F) -> (F, F) {
    let t20544 = t1692 * t1812 * t1989;
    let t20545 = 3.0 * t20417 * t19672 + 3.0 / 2.0 * t2439 * t5849 * t6120 - 3.0 / 2.0 * t18728 * t19678 + 3.0 / 2.0 * t2439 * t1812 * t19681 + 3.0 / 2.0 * t2439 * t1812 * t19685 + 3.0 / 2.0 * t2439 * t6354 * t5539 + t1692 * t20510 * t30 / 2.0 - t1692 * t20514 * t5591 / 2.0 + t1692 * t6354 * t580 / 2.0 - 3.0 / 2.0 * t18728 * t19810 - t1692 * t18807 * t6153 / 2.0 + t20526 * t19819 - t1692 * t5853 * t19821 / 2.0 - t1692 * t5853 * t19825 / 2.0 + 3.0 / 2.0 * t2439 * t1812 * t19829 + t1692 * t5849 * t1288 / 2.0 - t1692 * t5853 * t19836 / 2.0 + t20544;
    (t20544, t20545)
}
