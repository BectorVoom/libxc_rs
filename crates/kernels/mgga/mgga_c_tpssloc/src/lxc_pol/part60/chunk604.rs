//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 604/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk604<F: Float>(t1799: F, t6968: F, t6637: F, t6888: F, t5335: F, t550: F, t6976: F, t1992: F, t1834: F, t1998: F, t214: F, t1985: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7732 = t6968 * t1799;
    let t7733 = t6637 * t7732;
    let t7734 = t6888 * t7733;
    let t7736 = t5335 * t550;
    let t7737 = t6976 * t7736;
    let t7738 = t1992 * t7737;
    let t7740 = t1998 * t1834;
    let t7741 = t214 * t7740;
    let t7742 = t1985 * t7741;
    (t7732, t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742)
}
