//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1409;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1410;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta377<F: Float>(t1113: F, t136: F, t43800: F, t43804: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43777: F, t43833: F, t43835: F, t43837: F, t43839: F, t43842: F, t43845: F, t2403: F, t3298: F, t11220: F, t699: F, t1114: F, t9709: F, t3304: F, t3301: F, t1102: F, t11258: F, t3270: F, t3287: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t43848, t43851, t43853) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1409::<F>(t1113, t136, t43800, t43804, t43759, t43766, t43768, t43770, t43773, t43777, t43833, t43835, t43837, t43839, t43842, t43845);
        let (t43855, t43857, t43859, t43861, t43863, t43866, t43869) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1410::<F>(t2403, t3298, t11220, t699, t1114, t9709, t3304, t3301, t1102, t11258, t3270, t3287);
    (t43848, t43851, t43853, t43855, t43857, t43859, t43861, t43863, t43866, t43869)
}
