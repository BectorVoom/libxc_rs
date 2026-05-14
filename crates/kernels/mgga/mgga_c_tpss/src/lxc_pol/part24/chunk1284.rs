//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1284/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1284<F: Float>(t38: F, t42690: F, t1680: F, t19369: F, t19372: F, t19377: F, t19381: F, t19411: F, t19414: F, t19417: F, t21136: F, t5507: F, t6080: F, t6087: F, t6091: F, t13330: F, t13335: F, t14906: F, t14911: F, t1675: F, t1679: F, t18314: F, t18317: F, t19352: F, t21146: F, t21158: F, t21159: F, t21162: F, t4573: F, t4579: F, t5483: F, t5497: F, t5503: F, t5506: F, t6073: F, t61964: F, t61969: F, t61976: F, t72: F) -> (F, F) {
    let t69281 = t42690 * t38;
    let t69284 = 2.0 / 3.0 * t19411 * t6087 + 2.0 / 3.0 * t19414 * t6087 + 2.0 / 3.0 * t19417 * t6087 + 2.0 / 3.0 * t6080 * t19369 + 2.0 / 3.0 * t6080 * t19372 + 2.0 / 3.0 * t19411 * t6091 + 2.0 / 3.0 * t19414 * t6091 + 2.0 / 3.0 * t19417 * t6091 + 2.0 / 3.0 * t6080 * t19377 + 2.0 / 3.0 * t6080 * t19381 + t21136 * t5507 / 3.0 - t69281 * t1680 / 6.0;
    let t69326 = -t21146 * t5503 / 6.0 - t21146 * t5507 / 6.0 - t19352 * t6087 / 3.0 - t6073 * t19369 / 3.0 - t6073 * t19372 / 3.0 - t19352 * t6091 / 3.0 - t6073 * t19377 / 3.0 - t6073 * t19381 / 3.0 - t5483 * t21159 / 6.0 - t1675 * (-20.0 / 27.0 * t61964 * t4573 - 5.0 / 108.0 * t61969 * t14906 + 5.0 / 9.0 * t18317 * t14911 - 20.0 / 9.0 * t18314 * t4579 + 5.0 / 18.0 * t18317 * t13330 + 5.0 / 6.0 * t5497 * t13335 + t61976) * t72 * t1679 / 6.0 - t1675 * t21158 * t5506 / 6.0 - t5483 * t21162 / 3.0;
    (t69284, t69326)
}
