//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1342/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1342<F: Float>(t4596: F, t60: F, t13330: F, t13335: F, t14906: F, t14911: F, t1675: F, t1679: F, t19192: F, t19213: F, t19229: F, t19352: F, t19380: F, t20760: F, t20768: F, t20772: F, t20777: F, t20780: F, t21129: F, t22144: F, t22145: F, t22148: F, t3426: F, t3431: F, t5483: F, t5506: F, t581: F, t5971: F, t6073: F, t6090: F, t61976: F, t63556: F, t6471: F, t6475: F, t68056: F, t72: F) -> (F,) {
    let t72962 = t4596 * t60;
    let t72997 = -t6073 * t20772 / 3.0 - t19352 * t6475 / 3.0 - t6073 * t20777 / 3.0 - t6073 * t20780 / 3.0 - t5483 * t22145 / 6.0 - t1675 * (-220.0 / 27.0 * t72962 * t581 - 40.0 / 27.0 * t68056 * t3426 + 40.0 / 9.0 * t20760 * t3431 + 5.0 / 108.0 * t63556 * t14906 + 5.0 / 9.0 * t19213 * t14911 + 5.0 / 18.0 * t19213 * t13330 - 5.0 / 6.0 * t5971 * t13335 + t61976) * t72 * t1679 / 6.0 - t1675 * t22144 * t5506 / 6.0 - t5483 * t22148 / 3.0 - t1675 * t20768 * t6090 / 3.0 - t1675 * t6471 * t19380 / 3.0 + 5.0 / 3.0 * t19229 * t21129 + 5.0 / 3.0 * t19192 * t21129;
    (t72997,)
}
