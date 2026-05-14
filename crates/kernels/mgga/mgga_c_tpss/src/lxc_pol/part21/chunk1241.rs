//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1241/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1241<F: Float>(t1483: F, t1733: F, t18133: F, t18142: F, t18156: F, t19890: F, t19892: F, t19899: F, t19901: F, t19904: F, t19910: F, t19914: F, t373: F, t3994: F, t4017: F, t5626: F, t5631: F, t5634: F, t5643: F, t5646: F, t6172: F, t6175: F, t991: F) -> (F,) {
    let t19917 = -t1483 * t18133 - t1733 * t19899 + 2.0 * t18142 * t6175 + 2.0 * t18156 * t19914 + t19890 * t373 - t19892 * t991 + 2.0 * t19901 * t5634 - t19904 * t5643 - 6.0 * t19910 * t5631 + 2.0 * t3994 * t5626 - t4017 * t5626 - t5646 * t6172;
    (t19917,)
}
