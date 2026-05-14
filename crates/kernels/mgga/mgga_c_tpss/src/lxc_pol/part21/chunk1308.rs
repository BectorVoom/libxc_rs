//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1308/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1308<F: Float>(t1482: F, t18178: F, t4016: F, t5640: F, t18172: F, t2715: F, t6167: F, t11436: F, t11726: F, t1483: F, t18133: F, t18140: F, t18142: F, t18156: F, t18171: F, t18174: F, t18186: F, t19913: F, t19919: F, t19923: F, t19929: F, t19932: F, t19939: F, t19949: F, t19953: F, t2777: F, t342: F, t345: F, t3994: F, t4008: F, t4017: F, t5626: F, t5629: F, t5631: F, t5639: F, t5642: F, t61296: F, t61540: F, t61564: F, t61567: F, t6174: F, t6183: F, t64518: F, t9089: F) -> (F,) {
    let t64690 = t18178 * t1482;
    let t64694 = t5640 * t4016;
    let t64702 = t18172 * t1482;
    let t64714 = t2715 * t6167;
    let t64725 = 4.0 * t18142 * t19923 + 2.0 * t18171 * t64518 * t4008 - t5639 * t5640 * t11436 * t342 * t345 - t61564 * t1483 - 2.0 * t18133 * t4017 + 4.0 * t18142 * t19919 - t18140 * t6183 - 2.0 * t5629 * t19953 + 4.0 * t61567 * t19929 + 4.0 * t18156 * t64690 * t5642 + 4.0 * t18156 * t64694 * t5642 + 24.0 * t5631 * t61296 * t6174 * t2777 + 4.0 * t61540 * t64702 * t18174 - 2.0 * t61540 * t19913 * t18186 + 4.0 * t18133 * t3994 - 2.0 * t18171 * t19932 * t9089 - 2.0 * t18171 * t64714 * t18174 + t18171 * t19949 * t18186 + 4.0 * t5626 * t11726 - 2.0 * t5639 * t18178 * t19939;
    (t64725,)
}
