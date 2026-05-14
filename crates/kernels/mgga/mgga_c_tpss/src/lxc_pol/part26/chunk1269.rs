//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1269/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1269<F: Float>(t63964: F, t65567: F, t65624: F, t65647: F, t65440: F, t20985: F, t550: F, t21007: F, t546: F, t18351: F, t6471: F, t1860: F, t65208: F, t42181: F, t5965: F, t42178: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t66422 = 119.0 / 864.0 * t63964;
    let t67148 = 35.0 / 108.0 * t65567;
    let t67173 = 119.0 / 3456.0 * t65624;
    let t67187 = 119.0 / 864.0 * t65647;
    let t67531 = 22.0 / 9.0 * t65440;
    let t67886 = 2.0 * t20985 * t550;
    let t67888 = 2.0 * t546 * t21007;
    let t67935 = t6471 * t18351;
    let t67938 = t1860 * t65208;
    let t67953 = t42181 * t5965;
    let t67956 = t42178 * t5965;
    (t66422, t67148, t67173, t67187, t67531, t67886, t67888, t67935, t67938, t67953, t67956)
}
