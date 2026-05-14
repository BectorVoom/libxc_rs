//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 702/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk702<F: Float>(t226: F, t4758: F, t773: F, t774: F, t4715: F, t2389: F, t4706: F, t4701: F, t801: F, t2142: F, t2147: F, t2160: F, t2173: F, t2381: F, t3615: F, t3635: F, t3681: F, t4708: F, t4712: F, t4718: F, t4724: F, t761: F, t771: F, t797: F) -> (F, F, F, F, F, F, F) {
    let t4759 = t4758 * t226;
    let t4761 = t773 * t774 * t4759;
    let t4764 = t4715 * t226;
    let t4766 = t773 * t774 * t4764;
    let t4771 = t2389 * t774 * t4706;
    let t4775 = t801 * t774 * t4701;
    let t4778 = t2142 + 7.0 / 72.0 * t3615 + t2147 * t4708 / 16.0 - t761 * t4712 / 48.0 + t2160 * t4718 / 1536.0 + 7.0 / 2304.0 * t3635 + t2173 * t4724 / 384.0 - t771 * t4761 / 3072.0 - t771 * t4766 / 3072.0 + t2381 + 7.0 / 576.0 * t3681 + 5.0 / 768.0 * t797 * t4771 - t797 * t4775 / 768.0;
    (t4759, t4761, t4764, t4766, t4771, t4775, t4778)
}
