//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1213/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1213<F: Float>(t19817: F, t19818: F, t1398: F, t580: F, t30: F, t3724: F, t1288: F, t750: F, t821: F, t1692: F, t1713: F, t1989: F, t17929: F, t18047: F, t19670: F, t19672: F, t19678: F, t19681: F, t19685: F, t19798: F, t19802: F, t19810: F, t19816: F, t2439: F, t5539: F, t5586: F, t5590: F, t5591: F, t6120: F, t6149: F, t6153: F) -> (F, F, F, F, F, F, F) {
    let t19819 = t19817 * t19818;
    let t19821 = t580 * t1398;
    let t19825 = t30 * t3724;
    let t19829 = t1288 * t750;
    let t19836 = t1288 * t821;
    let t19841 = t1692 * t1713 * t1989;
    let t19842 = 3.0 * t19670 * t19672 + 3.0 / 2.0 * t2439 * t5586 * t6120 - 3.0 / 2.0 * t17929 * t19678 + 3.0 / 2.0 * t2439 * t1713 * t19681 + 3.0 / 2.0 * t2439 * t1713 * t19685 + 3.0 / 2.0 * t2439 * t6149 * t5539 + t1692 * t19798 * t30 / 2.0 - t1692 * t19802 * t5591 / 2.0 + t1692 * t6149 * t580 / 2.0 - 3.0 / 2.0 * t17929 * t19810 - t1692 * t18047 * t6153 / 2.0 + t19816 * t19819 - t1692 * t5590 * t19821 / 2.0 - t1692 * t5590 * t19825 / 2.0 + 3.0 / 2.0 * t2439 * t1713 * t19829 + t1692 * t5586 * t1288 / 2.0 - t1692 * t5590 * t19836 / 2.0 + t19841;
    (t19819, t19821, t19825, t19829, t19836, t19841, t19842)
}
