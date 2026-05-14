//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1333/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1333<F: Float>(t1675: F, t1678: F, t1680: F, t18324: F, t18345: F, t18347: F, t18356: F, t18360: F, t19380: F, t19381: F, t19425: F, t5483: F, t5489: F, t5502: F, t6090: F, t65396: F, t65400: F, t65403: F, t65406: F, t65410: F, t65413: F, t65417: F) -> (F,) {
    let t65424 = -t5483 * t19381 / 3.0 - t1675 * t18324 * t6090 / 6.0 - t1675 * t5502 * t19380 / 3.0 - t1675 * t1678 * t65396 / 6.0 + 2.0 / 3.0 * t65400 * t1680 + t65403 * t1680 / 3.0 - 5.0 * t65406 * t18347 - 5.0 * t18345 * t65410 + 5.0 / 3.0 * t65413 * t5489 + 5.0 / 3.0 * t65417 * t5489 + 5.0 / 3.0 * t19425 * t18356 + 5.0 / 6.0 * t19425 * t18360;
    (t65424,)
}
