//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1313/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1313<F: Float>(t19327: F, t3499: F, t19626: F, t61801: F, t19577: F, t5755: F, t13223: F, t196: F, t197: F, t1779: F, t19579: F, t19580: F, t43998: F, t13220: F, t93: F, t10456: F, t1165: F, t13146: F, t13554: F, t1688: F, t18403: F, t19305: F, t19596: F, t19656: F, t2056: F, t3493: F, t4347: F, t5531: F, t6112: F, t6234: F, t65094: F, t65097: F, t65458: F, t7798: F) -> (F, F, F, F, F, F) {
    let t65933 = 4.0 * t3499 * t19327;
    let t65935 = 6.0 * t61801 * t19626;
    let t65937 = 2.0 * t19577 * t5755;
    let t65941 = t13223 * t196 * t197;
    let t65942 = t65941 * t1779;
    let t65945 = 4.0 * t19579 * t19580 * t43998;
    let t65956 = t93 * t13220;
    let t65975 = 4.0 * t10456 * t6112 + 2.0 * t1165 * t65458 + 2.0 * t13146 * t6112 + 4.0 * t13554 * t5531 + 2.0 * t1688 * t65094 + 4.0 * t1688 * t65097 + 2.0 * t1688 * t65956 + 2.0 * t18403 * t3493 + 2.0 * t18403 * t6234 + 4.0 * t19305 * t5531 + 4.0 * t19596 * t2056 + 4.0 * t19596 * t4347 + 4.0 * t19656 * t5531 + 2.0 * t6112 * t7798;
    (t65933, t65935, t65937, t65942, t65945, t65975)
}
