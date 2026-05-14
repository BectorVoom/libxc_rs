//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1304/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1304<F: Float>(t64334: F, t64362: F, t64386: F, t64404: F, t64428: F, t64450: F, t64470: F, t64490: F, t19898: F, t5570: F, t3949: F, t5640: F, t18155: F, t6171: F, t11730: F, t11790: F, t18145: F, t18150: F, t18152: F, t18156: F, t18158: F, t18162: F, t18166: F, t18183: F, t19892: F, t19901: F, t19913: F, t19922: F, t19927: F, t19928: F, t19936: F, t19949: F, t19950: F, t2777: F, t2804: F, t2805: F, t373: F, t5626: F, t5631: F, t5632: F, t5634: F, t5639: F, t6167: F, t6174: F, t985: F, t990: F) -> (F, F, F) {
    let t64493 = t64334 + t64362 + t64386 + t64404 + t64428 + t64450 + t64470 + t64490;
    let t64515 = t19898 * t5570;
    let t64518 = t5640 * t3949;
    let t64529 = t6171 * t18155;
    let t64543 = 2.0 * t19901 * t18166 + param_beta * t64493 * t373 - 2.0 * t18145 * t19950 - 6.0 * t5631 * t18150 * t6167 * t2777 - t19892 * t2805 - 2.0 * t18145 * t19936 - 12.0 * t5631 * t18150 * t19922 * t990 - t5639 * t19949 * t18183 + 2.0 * t18156 * t19927 * t985 * t2804 + 4.0 * t64515 * t5634 + 4.0 * t18156 * t64518 * t19928 + 2.0 * t5631 * t5632 * t6167 * t2804 + 2.0 * t18156 * t19913 * t18183 + 4.0 * t64529 * t18158 - 6.0 * t19901 * t18152 + 4.0 * t19901 * t18162 - t5626 * t11790 + 2.0 * t5626 * t11730 - 6.0 * t5631 * t18150 * t6174 * t2804;
    (t64493, t64518, t64543)
}
