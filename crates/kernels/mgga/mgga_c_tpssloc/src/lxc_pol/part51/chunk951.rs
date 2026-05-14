//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 951/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk951<F: Float>(t225: F, t387: F, t4657: F, t345: F, t7569: F, t1921: F, t25749: F, t986: F, t7593: F, t990: F, t25705: F, t349: F, t1066: F, t1920: F, t23346: F, t23385: F, t23387: F, t23389: F, t3026: F, t3169: F, t388: F, t4557: F, t4660: F, t4665: F, t6687: F, t6771: F, t6776: F, t6816: F, t7554: F, t7566: F, t7600: F, t7625: F) -> (F,) {
    let t25766 = t4657 * t225 * t387;
    let t25767 = t345 * t25766;
    let t25778 = t7569 * t225;
    let t25784 = t1921 * t25749;
    let t25785 = t986 * t25784;
    let t25789 = t990 * t7593;
    let t25791 = t349 * t25705;
    let t25794 = -t4660 * t6816 + 0.82246703342411321825e-2 * t1920 * t25767 + 2.0 * t6771 * t4665 + 2.0 * t3169 * t7600 + 0.21932454224643019153e-1 * t23346 * t7566 + 2.0 * t4557 * t6776 - t25778 * t1066 - 0.27415567780803773942e-2 * t23385 - 0.27415567780803773942e-2 * t23387 - 0.73108180748810063845e-2 * t23346 * t7554 + 0.82246703342411321825e-2 * t6687 * t25785 - 0.73108180748810063845e-2 * t23389 + t25789 * t388 + t25791 * t388 - t3026 * t7625;
    (t25794,)
}
