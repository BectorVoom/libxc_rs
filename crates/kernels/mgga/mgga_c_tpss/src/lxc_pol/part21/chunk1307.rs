//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1307/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1307<F: Float>(t1464: F, t18178: F, t19898: F, t5638: F, t11722: F, t11767: F, t11771: F, t11789: F, t1482: F, t1726: F, t1730: F, t1731: F, t18130: F, t18145: F, t18156: F, t18171: F, t18179: F, t18184: F, t19904: F, t19914: F, t19927: F, t19928: F, t19933: F, t19940: F, t19942: F, t19949: F, t347: F, t4008: F, t4016: F, t5623: F, t5626: F, t5631: F, t5632: F, t5639: F, t5642: F, t5643: F, t61285: F, t61305: F, t61476: F, t61498: F, t61537: F, t61567: F, t6175: F, t6179: F, t6180: F, t64493: F, t990: F) -> (F,) {
    let t64623 = t18178 * t1464;
    let t64645 = t19898 * t5638;
    let t64670 = -4.0 * t61476 * t19933 + 2.0 * t61476 * t19942 + 4.0 * t18156 * t64623 * t19928 + 2.0 * t61537 * t6175 + 2.0 * t18171 * t64623 * t4008 - t1730 * t1731 * t347 * t64493 - 6.0 * t5626 * t11722 + 4.0 * t18156 * t19949 * t990 * t5642 - t61285 * t19927 * t11771 - t5639 * t61498 * t6179 - 2.0 * t64645 * t5643 - 2.0 * t19904 * t18179 - t19904 * t18184 + 4.0 * t61567 * t19914 - t61305 * t6180 + t18171 * t19927 * t11767 + 4.0 * t5631 * t5632 * t5623 * t4016 + 2.0 * t5631 * t5632 * t18130 * t1482 - 2.0 * t18145 * t19940 + 2.0 * t5631 * t5632 * t1726 * t11789;
    (t64670,)
}
