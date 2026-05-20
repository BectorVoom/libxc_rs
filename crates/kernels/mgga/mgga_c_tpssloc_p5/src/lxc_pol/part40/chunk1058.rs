//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1058/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1058<F: Float>(t16949: F, t2701: F, t820: F, t2697: F, t5628: F, t210: F, t5567: F, t776: F, t1495: F, t4119: F, t5571: F, t13223: F, t5591: F) -> (F, F, F, F, F, F) {
    let t16951 = t2701 * t820 * t16949;
    let t16954 = t2697 * t5628;
    let t16957 = t210 * t5567 * t776;
    let t16961 = t210 * t1495 * t4119;
    let t16965 = t210 * t5571 * t776;
    let t16968 = t13223 * t5591;
    (t16951, t16954, t16957, t16961, t16965, t16968)
}
