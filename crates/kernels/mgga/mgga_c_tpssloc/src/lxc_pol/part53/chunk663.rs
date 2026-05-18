//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 663/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk663<F: Float>(t1814: F, t2002: F, t559: F, t1827: F, t6945: F, t1831: F, t6952: F, t1799: F, t6968: F, t6637: F, t6888: F, t5335: F, t550: F) -> (F, F, F, F, F, F, F, F) {
    let t7715 = t1814 * t2002;
    let t7716 = t7715 * t559;
    let t7718 = t6945 * t1827;
    let t7720 = t6952 * t1831;
    let t7732 = t6968 * t1799;
    let t7733 = t6637 * t7732;
    let t7734 = t6888 * t7733;
    let t7736 = t5335 * t550;
    (t7715, t7716, t7718, t7720, t7732, t7733, t7734, t7736)
}
