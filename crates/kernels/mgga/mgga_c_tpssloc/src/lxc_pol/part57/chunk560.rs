//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 560/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk560<F: Float>(t1799: F, t6968: F, t6637: F, t6888: F, t5335: F, t550: F, t6976: F, t1992: F, t1834: F, t1998: F, t214: F, t1985: F, t1825: F, t6987: F, t553: F, t7722: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7732 = t6968 * t1799;
    let t7733 = t6637 * t7732;
    let t7734 = t6888 * t7733;
    let t7736 = t5335 * t550;
    let t7737 = t6976 * t7736;
    let t7738 = t1992 * t7737;
    let t7740 = t1998 * t1834;
    let t7741 = t214 * t7740;
    let t7742 = t1985 * t7741;
    let t7745 = t6987 * t1825;
    let t7747 = t553 * t7722;
    (t7732, t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742, t7745, t7747)
}
