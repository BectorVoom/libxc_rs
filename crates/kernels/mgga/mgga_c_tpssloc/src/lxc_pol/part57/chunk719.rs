//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 719/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk719<F: Float>(t5677: F, t6785: F, t23696: F, t1945: F, t5866: F, t1060: F, t25470: F, t7603: F, t1409: F, t1615: F, t6800: F, t23635: F, t1949: F, t5844: F, t5838: F, t1599: F, t7614: F) -> (F, F, F, F, F, F, F) {
    let t28637 = t6785 * t5677;
    let t28638 = t23696 * t28637;
    let t28641 = t1945 * t5866;
    let t28642 = t28641 * t1060;
    let t28648 = t25470 * t7603;
    let t28651 = t1409 * t1615;
    let t28652 = t28651 * t6800;
    let t28653 = t23635 * t28652;
    let t28657 = t5844 * t1949;
    let t28660 = t5838 * t1949;
    let t28663 = t1599 * t7614;
    (t28638, t28642, t28648, t28653, t28657, t28660, t28663)
}
