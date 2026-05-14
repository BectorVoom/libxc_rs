//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 987/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk987<F: Float>(t23076: F, t281: F, t6597: F, t2690: F, t6612: F, t812: F, t59: F, t9971: F, t240: F, t23061: F, t6604: F, t1891: F, t1895: F, t213: F, t39041: F, t1887: F, t206: F, t80845: F) -> (F, F, F, F, F, F) {
    let t81792 = t6597 * t23076 * t281;
    let t81807 = t812 * t6612 * t2690;
    let t81816 = t9971 * t59;
    let t81818 = t812 * t81816 * t240;
    let t81835 = t23061 * t6604;
    let t81849 = t39041 * t1891 * t213 * t1895;
    let t81852 = t80845 * t206 * t1887;
    (t81792, t81807, t81818, t81835, t81849, t81852)
}
