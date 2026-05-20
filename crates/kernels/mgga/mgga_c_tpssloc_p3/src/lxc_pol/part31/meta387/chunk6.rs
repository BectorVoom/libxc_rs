//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1381/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1381<F: Float>(t17766: F, t17798: F, t17852: F, t17873: F, t225: F, t68: F, t369: F, t10457: F, t248: F, t5677: F, t1041: F, t1044: F, t17187: F) -> (F, F, F, F, F) {
    let t17875 = t17766 + t17798 + t17852 + t17873;
    let t17876 = t17875 * t225;
    let t17877 = t17876 * t68;
    let t17878 = t17877 * t369;
    let t17884 = t248 * t10457 * t5677;
    let t17885 = t1041 * t17884;
    let t17890 = t248 * t1044 * t17187;
    (t17875, t17876, t17878, t17885, t17890)
}
