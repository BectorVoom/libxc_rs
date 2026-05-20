//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1938/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1938<F: Float>(t23133: F, t5628: F, t23041: F, t5614: F, t1512: F, t87261: F, t16944: F, t25119: F, t841: F, t23083: F, t28372: F, t28395: F, t81782: F, t81783: F) -> (F, F, F, F, F, F) {
    let t98733 = t23133 * t5628;
    let t98736 = t23041 * t5614;
    let t98738 = t87261 * t1512;
    let t98744 = t25119 * t841 * t16944;
    let t98746 = t23083 * t28372;
    let t98750 = t81782 * t81783 * t28395;
    (t98733, t98736, t98738, t98744, t98746, t98750)
}
