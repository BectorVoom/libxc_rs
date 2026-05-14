//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 925/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk925<F: Float>(t16814: F, t17048: F, t858: F, t225: F, t5559: F, t5657: F, t865: F, t2718: F, t17022: F, t218: F, t5636: F, t10110: F, t1527: F, t4300: F, t259: F, t2597: F, t2713: F, t4147: F, t4268: F, t4273: F, t4301: F, t5637: F, t5658: F, t855: F, t866: F) -> (F,) {
    let t17049 = t16814 + t17048;
    let t17050 = t858 * t17049;
    let t17052 = t5559 * t225;
    let t17056 = t5657 * t865;
    let t17057 = t2718 * t17056;
    let t17060 = t218 * t17022;
    let t17063 = t5636 * t865;
    let t17064 = t10110 * t17063;
    let t17069 = t1527 * t4300;
    let t17070 = t2718 * t17069;
    let t17079 = -t17050 * t855 - t17052 * t866 + 2.0 * t17057 * t855 + t17060 * t259 - 6.0 * t17064 * t855 + 4.0 * t17070 * t855 + 2.0 * t2597 * t5637 + 2.0 * t2713 * t5637 - t2713 * t5658 + 4.0 * t4147 * t4273 - 2.0 * t4147 * t4301 - 2.0 * t4268 * t4301;
    (t17079,)
}
