//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1162/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1162<F: Float>(t21804: F, t1838: F, t5432: F, t18490: F, t18967: F, t21074: F, t1656: F, t6419: F, t5740: F, t5448: F, t5380: F, t18511: F, t3260: F, t1639: F, t520: F, t5745: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21805 = param_beta * t21804;
    let t21819 = t1838 * t5432;
    let t21820 = t18490 * t21819;
    let t21823 = t18967 * t21074;
    let t21826 = t6419 * t1656;
    let t21827 = t5740 * t21826;
    let t21830 = t1838 * t5448;
    let t21831 = t5740 * t21830;
    let t21834 = t1838 * t5380;
    let t21836 = t18511 * t21834 * t3260;
    let t21840 = t6419 * t1639 * t520;
    let t21841 = t5745 * t21840;
    (t21805, t21819, t21820, t21823, t21826, t21827, t21830, t21831, t21834, t21836, t21841)
}
