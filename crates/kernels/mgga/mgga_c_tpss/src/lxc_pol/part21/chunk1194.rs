//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1194/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1194<F: Float>(t1266: F, t1772: F, t1775: F, t18472: F, t18474: F, t18481: F, t18483: F, t18492: F, t18496: F, t18500: F, t18504: F, t18508: F, t18514: F, t18519: F, t18524: F, t18527: F, t18530: F, t3367: F, t3385: F, t538: F, t5734: F, t5737: F, t5739: F, t5742: F, t5748: F, t5751: F) -> (F,) {
    let t18532 = -2.0 * t1266 * t18474 - t1772 * t18530 - t1775 * t18481 + t18472 * t538 + 4.0 * t18483 * t5742 + 2.0 * t18483 * t5748 - 6.0 * t18492 * t5739 - 4.0 * t18496 * t18500 + 4.0 * t18504 * t5739 + 2.0 * t18508 * t5739 - 2.0 * t18514 * t5739 + 2.0 * t18519 * t5739 + t18524 * t5739 + t18527 * t5739 + 2.0 * t3367 * t5734 - t3385 * t5734 - 2.0 * t5737 * t5751;
    (t18532,)
}
