//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 666/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk666<F: Float>(t1015: F, t7582: F, t23472: F, t25637: F, t343: F, t23562: F, t344: F, t7573: F, t6740: F, t23384: F, t7566: F, t1054: F, t1634: F, t225: F, t7594: F, t7569: F) -> (F, F, F, F, F, F, F) {
    let t25641 = t1015 * t7582;
    let t25642 = t23472 * t25641;
    let t25644 = t25637 * t343;
    let t25645 = t23562 * t25644;
    let t25682 = t7573 * t344;
    let t25683 = t6740 * t25682;
    let t25736 = t23384 * t7566;
    let t25749 = t1054 * t1634;
    let t25755 = t7594 * t225;
    let t25778 = t7569 * t225;
    (t25642, t25645, t25683, t25736, t25749, t25755, t25778)
}
