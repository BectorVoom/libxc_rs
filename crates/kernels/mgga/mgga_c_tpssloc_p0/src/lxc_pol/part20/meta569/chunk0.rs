//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2130/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2130<F: Float>(t10508: F, t248: F, t3039: F, t3041: F, t3020: F, t3030: F, t3032: F, t3038: F, t10360: F, t1040: F, t1043: F, t204: F) -> (F, F, F, F, F, F) {
    let t42735 = t3039 * t248 * t10508 * t3041;
    let t42741 = t3020 * t3030;
    let t42742 = t42741 * t3032;
    let t42743 = t42742 * t3038;
    let t42746 = t10360 * t1040;
    let t42749 = t204 * t1043;
    (t42735, t42741, t42742, t42743, t42746, t42749)
}
