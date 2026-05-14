//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 674/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk674<F: Float>(t12108: F, t69045: F, t12111: F, t69788: F, t3046: F, t507: F, t7262: F, t12117: F, t13966: F, t2046: F, t8475: F, t2050: F, t2408: F, t31: F, t2039: F, t2406: F, t270: F, t638: F) -> (F, F, F, F, F, F) {
    let t73938 = t69045 * t12108;
    let t73940 = t69788 * t12111;
    let t73943 = t507 * t7262 * t3046;
    let t73944 = t73943 * t12117;
    let t73949 = t2046 * t13966 * t8475;
    let t73953 = t2046 * t2050 * t2408 * t31;
    let t73957 = t638 * t2039 * t2406 * t270;
    (t73938, t73940, t73944, t73949, t73953, t73957)
}
