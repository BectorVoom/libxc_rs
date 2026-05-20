//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 426/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk426<F: Float>(t241: F, t2690: F, t244: F, t248: F, t238: F, t835: F, t841: F, t812: F, t849: F, t1891: F, t67: F, t225: F, t853: F) -> (F, F, F, F, F, F, F) {
    let t2691 = t2690 * t241;
    let t2693 = t2691 * t244 * t248;
    let t2695 = F::new(119.0) / F::new(13824.0) * t238 * t2693;
    let t2696 = t841 * t835;
    let t2697 = t812 * t2696;
    let t2698 = t2697 * t849;
    let t2700 = t241 * t1891;
    let t2701 = t2700 * t67;
    let t2713 = t853 * t225;
    (t2691, t2693, t2695, t2697, t2698, t2701, t2713)
}
