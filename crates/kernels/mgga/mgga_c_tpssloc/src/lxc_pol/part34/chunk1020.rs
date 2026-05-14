//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1020/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1020<F: Float>(t23041: F, t5614: F, t1512: F, t87261: F, t23083: F, t28372: F, t28395: F, t81782: F, t81783: F, t22690: F, t5527: F, t81792: F, t841: F, t236: F, t5584: F, t23109: F, t2632: F, t81914: F) -> (F, F, F, F, F, F, F) {
    let t98736 = t23041 * t5614;
    let t98738 = t87261 * t1512;
    let t98746 = t23083 * t28372;
    let t98750 = t81782 * t81783 * t28395;
    let t98774 = t81792 * t22690 * t841 * t5527;
    let t98779 = t236 * t5584;
    let t98782 = t23109 * t81914 * t98779 * t2632;
    (t98736, t98738, t98746, t98750, t98774, t98779, t98782)
}
