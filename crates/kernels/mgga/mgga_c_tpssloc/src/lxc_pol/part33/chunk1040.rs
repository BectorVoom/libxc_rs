//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1040/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1040<F: Float>(t1015: F, t1615: F, t344: F, t7573: F, t6740: F, t2770: F, t381: F, t23384: F, t7566: F, t1054: F, t1634: F, t225: F, t7594: F, t254: F, t382: F, t10164: F, t1955: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25658 = t1015 * t1615;
    let t25682 = t7573 * t344;
    let t25683 = t6740 * t25682;
    let t25721 = t381 * t2770;
    let t25736 = t23384 * t7566;
    let t25749 = t1054 * t1634;
    let t25755 = t7594 * t225;
    let t25757 = t382 * t254;
    let t25758 = t10164 * t1955;
    (t25658, t25682, t25683, t25721, t25736, t25749, t25755, t25757, t25758)
}
