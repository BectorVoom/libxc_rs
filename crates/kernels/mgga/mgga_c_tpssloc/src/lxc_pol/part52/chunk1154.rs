//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1154/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1154<F: Float>(t30728: F, t858: F, t798: F, t8347: F, t225: F, t8348: F, t218: F, t30725: F, t6547: F, t8336: F, t1912: F, t23278: F, t23281: F, t259: F, t2597: F, t2713: F, t30673: F, t6627: F, t6663: F, t8353: F, t8363: F, t855: F, t866: F) -> (F, F, F, F, F, F) {
    let t30729 = t858 * t30728;
    let t30732 = t798 * t8347;
    let t30741 = t8348 * t225;
    let t30745 = t218 * t30725;
    let t30748 = F::new(0.38381794893125283518e-1) * t6547 * t8336;
    let t30751 = -F::new(2.0) * t1912 * t23278 - F::new(2.0) * t1912 * t23281 + t259 * t30732 + t259 * t30745 + F::new(2.0) * t2597 * t8353 - t2597 * t8363 + F::new(2.0) * t2713 * t8353 - t2713 * t8363 - t30729 * t855 - t30741 * t866 - F::new(2.0) * t6627 * t6663 - t30673 + t30748;
    (t30729, t30732, t30741, t30745, t30748, t30751)
}
