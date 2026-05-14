//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1306/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1306<F: Float>(t11707: F, t11733: F, t11743: F, t1464: F, t1705: F, t1733: F, t18142: F, t18150: F, t18171: F, t18172: F, t18175: F, t18187: F, t18190: F, t19889: F, t19892: F, t19899: F, t19909: F, t19910: F, t19918: F, t19927: F, t19932: F, t19946: F, t2724: F, t2777: F, t2778: F, t3949: F, t3997: F, t5631: F, t5632: F, t5639: F, t5642: F, t5646: F, t61285: F, t61292: F, t61489: F, t61522: F, t6172: F, t64548: F, t64550: F, t64557: F, t64565: F, t64573: F, t64590: F, t64613: F, t935: F, t948: F, t949: F, t985: F, t990: F, t991: F) -> (F,) {
    let t64617 = 2.0 * t19892 * t2778 + 8.0 * t64548 * t64550 * t2724 * t990 * t948 - 4.0 * t64557 * t64550 * t949 * t990 - 12.0 * t64565 * t19909 * t5642 - t1705 * t11707 * t935 * t1733 - t6172 * t18190 - 2.0 * t5639 * t64573 * t5642 - 12.0 * t18142 * t19910 - 12.0 * t5631 * t18150 * t19918 * t990 - 6.0 * t61285 * t61489 * t1464 * t11733 + 6.0 * t61285 * t19932 * t11743 - 2.0 * t64590 * t991 - 6.0 * t61522 * t19927 * t985 * t2777 - 4.0 * t18171 * t61292 * t1464 * t3997 - 4.0 * t18171 * t18172 * t3949 * t3997 - 2.0 * t19899 * t5646 + 4.0 * t5631 * t5632 * t19889 * t990 + 4.0 * t18142 * t19946 - 2.0 * t64613 * t18175 + t64613 * t18187;
    (t64617,)
}
