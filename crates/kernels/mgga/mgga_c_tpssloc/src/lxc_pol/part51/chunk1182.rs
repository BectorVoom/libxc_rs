//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1182/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1182<F: Float>(t118413: F, t25927: F, t118466: F, t23788: F, t1081: F, t7540: F, t118953: F, t1649: F, t6665: F, t25353: F, t28: F, t118454: F, t2314: F, t32677: F, t4034: F, t5107: F, t652: F, t8326: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t119700 = t25927 * t118413;
    let t119719 = t23788 * t118466;
    let t119737 = t1081 * t7540;
    let t119743 = t25927 * t118953;
    let t119746 = t1649 * t6665;
    let t119766 = t28 * t25353;
    let t119780 = t23788 * t118454;
    let t119824 = 2.0 * t2314 * t32677;
    let t119826 = 2.0 * t4034 * t32677;
    let t119830 = 2.0 * t652 * t5107 * t8326;
    (t119700, t119719, t119737, t119743, t119746, t119766, t119780, t119824, t119826, t119830)
}
