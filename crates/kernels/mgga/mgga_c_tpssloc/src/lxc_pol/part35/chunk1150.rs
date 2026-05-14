//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1150/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1150<F: Float>(t240: F, t812: F, t81816: F, t23061: F, t6604: F, t1891: F, t1895: F, t213: F, t39041: F, t1887: F, t206: F, t80845: F, t23102: F, t80782: F, t23093: F, t281: F) -> (F, F, F, F, F, F) {
    let t81818 = t812 * t81816 * t240;
    let t81835 = t23061 * t6604;
    let t81849 = t39041 * t1891 * t213 * t1895;
    let t81850 = 0.10173934535723378495e0 * t81849;
    let t81852 = t80845 * t206 * t1887;
    let t81853 = 455.0 / 1296.0 * t81852;
    let t81876 = t23102 * t80782;
    let t81882 = t23093 * t281;
    (t81818, t81835, t81850, t81853, t81876, t81882)
}
