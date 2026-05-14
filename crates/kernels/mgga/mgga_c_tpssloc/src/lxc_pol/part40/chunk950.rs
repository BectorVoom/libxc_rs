//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 950/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk950<F: Float>(t16684: F, t16686: F, t16698: F, t16720: F, t225: F, t1504: F, t68: F, t1891: F, t5527: F, t776: F, t4119: F, t4226: F, t5544: F, t845: F, t16662: F, t824: F) -> (F, F, F, F, F, F) {
    let t16723 = (t16684 + t16686 + t16698 + t16720) * t225;
    let t16729 = t1504 * t68;
    let t16736 = t1891 * t5527;
    let t16737 = t16736 * t776;
    let t16740 = t4226 * t4119;
    let t16745 = t845 * t5544;
    let t16746 = t16745 * t776;
    let t16749 = t824 * t16662;
    (t16723, t16729, t16737, t16740, t16746, t16749)
}
