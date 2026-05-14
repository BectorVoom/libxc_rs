//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1331/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1331<F: Float>(t10353: F, t10412: F, t10416: F, t11476: F, t1289: F, t1675: F, t1679: F, t18305: F, t18314: F, t18317: F, t18331: F, t18332: F, t18356: F, t18360: F, t19349: F, t19368: F, t19369: F, t19372: F, t19377: F, t19393: F, t3426: F, t3431: F, t5483: F, t5497: F, t5506: F, t6073: F, t6086: F, t6087: F, t6091: F, t61943: F, t61961: F, t61964: F, t61969: F, t61976: F, t72: F) -> (F,) {
    let t65387 = 5.0 / 3.0 * t19393 * t18356 + 5.0 / 6.0 * t19393 * t18360 - t6073 * t18332 / 6.0 - 5.0 / 3.0 * t19349 * t61943 - t18305 * t6087 / 6.0 - t5483 * t19369 / 3.0 - t5483 * t19372 / 3.0 - t1675 * (220.0 / 27.0 * t61961 * t1289 - 40.0 / 27.0 * t61964 * t3426 - 40.0 / 9.0 * t18314 * t3431 - 5.0 / 108.0 * t61969 * t11476 + 5.0 / 9.0 * t18317 * t10416 + 5.0 / 18.0 * t18317 * t10412 + 5.0 / 6.0 * t5497 * t10353 + t61976) * t72 * t1679 / 6.0 - t1675 * t19368 * t5506 / 3.0 - t1675 * t6086 * t18331 / 6.0 - t18305 * t6091 / 6.0 - t5483 * t19377 / 3.0;
    (t65387,)
}
