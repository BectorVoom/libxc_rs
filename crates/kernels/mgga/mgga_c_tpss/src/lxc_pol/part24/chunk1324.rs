//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1324/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1324<F: Float>(t18172: F, t4977: F, t5036: F, t5640: F, t21398: F, t5570: F, t1464: F, t6174: F, t18142: F, t18145: F, t18150: F, t18156: F, t18178: F, t19899: F, t19904: F, t19929: F, t19940: F, t19950: F, t21407: F, t21408: F, t21415: F, t21418: F, t21419: F, t21427: F, t21431: F, t21432: F, t21438: F, t2786: F, t3997: F, t4008: F, t5017: F, t5623: F, t5629: F, t5631: F, t5634: F, t5639: F, t5642: F, t61296: F, t61540: F, t6179: F, t6183: F, t64529: F, t64548: F, t64557: F, t64690: F, t990: F) -> (F, F, F) {
    let t70487 = t18172 * t4977;
    let t70497 = t5640 * t5036;
    let t70524 = t21398 * t5570;
    let t70527 = t6174 * t1464;
    let t70541 = 4.0 * t61540 * t70487 * t2786 * t990 - 2.0 * t19904 * t19940 - t18145 * t21432 - 2.0 * t19899 * t6183 + 2.0 * t18156 * t70497 * t5642 + 4.0 * t64529 * t19929 - 6.0 * t5631 * t18150 * t5623 * t5017 - 6.0 * t5631 * t18150 * t21418 * t990 + 4.0 * t18142 * t21415 + 4.0 * t18156 * t64690 * t6179 + 2.0 * t18142 * t21419 - 2.0 * t18145 * t21427 + 24.0 * t5631 * t61296 * t21407 * t990 + 2.0 * t70524 * t5634 + 8.0 * t64548 * t70527 * t3997 - 4.0 * t64557 * t70527 * t4008 - t5629 * t21438 - 6.0 * t18142 * t21408 - 2.0 * t19904 * t19950 - t5639 * t18178 * t21431;
    (t70487, t70527, t70541)
}
