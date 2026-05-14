//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1015/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1015<F: Float>(t3448: F, t6138: F, t3451: F, t6144: F, t18225: F, t4908: F, t11583: F, t5392: F, t3449: F, t18221: F, t15320: F, t4904: F, t15313: F, t4919: F, t11531: F, t15265: F, t15376: F, t18404: F, t18410: F, t18413: F, t3447: F, t4901: F) -> (F,) {
    let t18416 = t3448 * t6138;
    let t18417 = t18416 * t3451;
    let t18420 = t3448 * t6144;
    let t18421 = t18420 * t3451;
    let t18424 = t4908 * t18225;
    let t18427 = t11583 * t5392;
    let t18428 = t3449 * t18427;
    let t18431 = t4908 * t18221;
    let t18434 = t15320 * t4904;
    let t18437 = t4919 * t15313;
    let t18442 = 0.37037037037037037036e-3 * t3447 * t18404 - 0.19753086419753086419e-2 * t15376 * t4901 + 0.27777777777777777777e-3 * t3447 * t18410 - 0.55555555555555555554e-3 * t3447 * t18413 + 0.27777777777777777777e-3 * t3447 * t18417 + 0.27777777777777777777e-3 * t3447 * t18421 - 0.11111111111111111111e-2 * t3447 * t18424 + 0.55555555555555555554e-3 * t3447 * t18428 - 0.16666666666666666666e-2 * t3447 * t18431 + 0.55555555555555555554e-3 * t3447 * t18434 + 0.55555555555555555554e-3 * t3447 * t18437 + 0.6172839506172839506e-4 * t11531 + 0.98765432098765432093e-3 * t15265;
    (t18442,)
}
