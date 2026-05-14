//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1209/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1209<F: Float>(t18454: F, t5389: F, t5410: F, t5721: F, t5415: F, t5420: F, t5728: F, t5424: F, t18435: F, t18462: F, t20142: F, t20146: F, t20151: F, t21036: F, t21038: F, t21040: F) -> (F,) {
    let t21042 = t18454 * t5389;
    let t21044 = t5721 * t5410;
    let t21046 = t5721 * t5415;
    let t21048 = t5728 * t5420;
    let t21050 = t5728 * t5424;
    let t21052 = t18435 + t20142 + t21036 / 16.0 - t21038 / 48.0 + t21040 / 768.0 + t20146 + t21042 / 192.0 - t21044 / 1536.0 - t21046 / 1536.0 + t18462 + t20151 + 5.0 / 384.0 * t21048 - t21050 / 384.0;
    (t21052,)
}
