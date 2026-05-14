//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1203/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1203<F: Float>(t19367: F, t72: F, t1679: F, t5506: F, t6086: F, t5502: F, t6090: F, t3482: F, t76: F, t1678: F, t1675: F, t1680: F, t18345: F, t18350: F, t18352: F, t19342: F, t19346: F, t19349: F, t19352: F, t5483: F, t5492: F, t5503: F, t5507: F, t6073: F, t6087: F, t6091: F) -> (F, F, F, F, F, F, F) {
    let t19368 = t19367 * t72;
    let t19369 = t19368 * t1679;
    let t19372 = t6086 * t5506;
    let t19377 = t5502 * t6090;
    let t19380 = t76 * t3482;
    let t19381 = t1678 * t19380;
    let t19386 = -5.0 * t18345 * t19342 - 5.0 / 3.0 * t18350 * t19346 - 5.0 / 3.0 * t19349 * t18352 - t19352 * t1680 / 6.0 - t6073 * t5503 / 6.0 - t6073 * t5507 / 6.0 - t5483 * t6087 / 6.0 - t1675 * t19369 / 6.0 - t1675 * t19372 / 6.0 - t5483 * t6091 / 6.0 - t1675 * t19377 / 6.0 - t1675 * t19381 / 6.0 + t5492 * t6087 / 3.0;
    (t19368, t19369, t19372, t19377, t19380, t19381, t19386)
}
