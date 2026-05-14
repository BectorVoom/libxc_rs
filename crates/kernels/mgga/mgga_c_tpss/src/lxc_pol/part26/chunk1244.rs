//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1244/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1244<F: Float>(t21995: F, t22107: F, t1864: F, t4637: F, t1338: F, t6540: F, t1897: F, t4674: F, t118: F, t1865: F, t21014: F, t21016: F, t21020: F, t21026: F, t21030: F, t21035: F, t21109: F, t21111: F, t21114: F, t21177: F, t21179: F, t21182: F, t3493: F, t4675: F, t485: F, t5314: F, t5986: F, t626: F, t6486: F) -> (F, F, F, F, F) {
    let t22108 = t21995 + t22107;
    let t22110 = t1864 * t4637;
    let t22114 = t6540 * t1338;
    let t22117 = t1897 * t4674;
    let t22124 = -t118 * t22108 - t1865 * t5314 - 2.0 * t22110 * t485 - 4.0 * t22114 * t626 - 2.0 * t22117 * t626 - 4.0 * t3493 * t6486 - 2.0 * t4675 * t5986 - t21014 - t21016 + t21020 + t21026 + t21030 + t21035 + t21109 - t21111 + t21114 - t21177 - t21179 - t21182;
    (t22108, t22110, t22114, t22117, t22124)
}
