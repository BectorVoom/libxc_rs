//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1321/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1321<F: Float>(t18351: F, t6471: F, t1860: F, t65208: F, t42181: F, t5965: F, t42178: F, t10292: F, t19191: F, t18338: F, t18347: F, t18350: F, t18356: F, t1861: F, t19229: F, t19232: F, t19349: F, t19388: F, t20719: F, t20772: F, t5489: F, t5492: F, t63590: F, t6475: F, t65400: F, t65410: F) -> (F,) {
    let t67935 = t6471 * t18351;
    let t67938 = t1860 * t65208;
    let t67953 = t42181 * t5965;
    let t67956 = t42178 * t5965;
    let t67961 = t10292 * t19191;
    let t67964 = -10.0 / 3.0 * t18350 * t67935 - 10.0 / 3.0 * t18350 * t67938 - 5.0 / 3.0 * t19349 * t63590 + 5.0 / 3.0 * t20719 * t18356 + 2.0 / 3.0 * t5492 * t20772 + 5.0 / 3.0 * t19229 * t19388 + 2.0 / 3.0 * t18338 * t6475 - 5.0 * t19232 * t65410 - 5.0 * t67953 * t18347 + 5.0 / 3.0 * t67956 * t5489 + 2.0 / 3.0 * t65400 * t1861 + 5.0 / 3.0 * t67961 * t5489;
    (t67964,)
}
