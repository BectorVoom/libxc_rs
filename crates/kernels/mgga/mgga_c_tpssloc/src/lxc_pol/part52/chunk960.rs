//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 960/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk960<F: Float>(t25605: F, t25631: F, t25672: F, t25703: F, t383: F, t4673: F, t7619: F, t1598: F, t984: F, t23478: F, t6785: F, t4347: F, t6784: F, t2770: F, t381: F, t3961: F) -> (F, F, F, F, F, F, F) {
    let t25705 = t25605 + t25631 + t25672 + t25703;
    let t25706 = t383 * t25705;
    let t25708 = t7619 * t4673;
    let t25712 = t1598 * t984;
    let t25713 = t23478 * t6785;
    let t25714 = t25712 * t25713;
    let t25717 = t6785 * t4347;
    let t25718 = t6784 * t25717;
    let t25721 = t381 * t2770;
    let t25722 = t25721 * t3961;
    (t25705, t25706, t25708, t25712, t25714, t25718, t25722)
}
