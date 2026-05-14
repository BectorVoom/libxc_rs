//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1023/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1023<F: Float>(t23061: F, t6604: F, t23099: F, t6605: F, t815: F, t9661: F, t23097: F, t232: F, t47320: F, t1891: F, t1895: F, t213: F, t39041: F, t1887: F, t206: F, t80845: F) -> (F, F, F, F, F) {
    let t81835 = t23061 * t6604;
    let t81836 = t81835 * t23099;
    let t81839 = t6605 * t815 * t9661;
    let t81843 = t23097 * t815 * t47320 * t232;
    let t81849 = t39041 * t1891 * t213 * t1895;
    let t81852 = t80845 * t206 * t1887;
    (t81836, t81839, t81843, t81849, t81852)
}
