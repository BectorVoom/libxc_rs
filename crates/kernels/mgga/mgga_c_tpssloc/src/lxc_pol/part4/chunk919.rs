//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 919/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk919<F: Float>(t2639: F, t5619: F, t5614: F, t1484: F, t4119: F, t2701: F, t820: F, t5544: F, t776: F, t2697: F, t5628: F, t210: F, t5567: F, t1495: F, t5571: F, t13223: F, t5591: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16940 = t2639 * t5619;
    let t16942 = t2639 * t5614;
    let t16944 = t1484 * t4119;
    let t16946 = t2701 * t820 * t16944;
    let t16949 = t5544 * t776;
    let t16951 = t2701 * t820 * t16949;
    let t16954 = t2697 * t5628;
    let t16957 = t210 * t5567 * t776;
    let t16961 = t210 * t1495 * t4119;
    let t16965 = t210 * t5571 * t776;
    let t16968 = t13223 * t5591;
    (t16940, t16942, t16946, t16951, t16954, t16957, t16961, t16965, t16968)
}
