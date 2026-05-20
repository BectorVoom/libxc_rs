//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2111;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta675<F: Float>(t24716: F, t4997: F, t15492: F, t7339: F, t15734: F, t7345: F, t25588: F, t461: F, t7324: F, t1244: F, t1742: F, t3068: F, sigma2: F, t1210: F, t24721: F, t27691: F, t27700: F, t86261: F, t15418: F, t2121: F, t4724: F, t24720: F, t27710: F, t24722: F, t11588: F, t4729: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t95542, t95545, t95550, t95556, t95566) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2111::<F>(t24716, t4997, t15492, t7339, t15734, t7345, t25588, t461, t7324, t1244, t1742, t3068, sigma2);
        let (t95571, t95573, t95587, t95588, t95590, t95593) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2112::<F>(t1210, t24721, t27691, t27700, t86261, t15418, t2121, t4724, t24720, t27710, t24722, t11588, t4729);
    (t95542, t95545, t95550, t95556, t95566, t95571, t95573, t95587, t95588, t95590, t95593)
}
