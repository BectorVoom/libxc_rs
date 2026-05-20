//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta718 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2283;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta718<F: Float>(t100674: F, t100716: F, t100763: F, t100803: F, t24987: F, t7754: F, t1983: F, t2019: F, t57806: F, t25971: F, t91655: F, t26161: F, t26162: F, t75210: F, t25994: F, t7458: F, t28817: F, t6876: F, t28826: F, t83859: F, t26149: F, t7685: F, t16524: F, t26545: F, t1873: F, t66958: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t100805, t100828, t100833, t100835, t100838) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2283::<F>(t100674, t100716, t100763, t100803, t24987, t7754, t1983, t2019, t57806, t25971, t91655, t26161, t26162, t75210);
        let (t100840, t100854, t100861, t100863, t100871, t100873) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2284::<F>(t25994, t7458, t28817, t6876, t1983, t28826, t83859, t26149, t7685, t16524, t26545, t1873, t66958);
    (t100805, t100828, t100833, t100835, t100838, t100840, t100854, t100861, t100863, t100871, t100873)
}
