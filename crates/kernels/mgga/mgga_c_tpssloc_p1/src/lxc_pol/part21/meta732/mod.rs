//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta732 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2589;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta732<F: Float>(t44620: F, t461: F, t60: F, t15394: F, t1714: F, t3439: F, t3447: F, t4724: F, t697: F, t11590: F, t15376: F, t11554: F, t1706: F, t44579: F, t4904: F, t11545: F, t134: F, t14726: F, t11579: F, t15338: F, t4899: F, t4928: F, t11570: F, t12648: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t52096, t52100, t52109, t52122, t52124) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2589::<F>(t44620, t461, t60, t15394, t1714, t3439, t3447, t4724, t697, t11590, t15376, t11554, t1706);
        let (t52127, t52133, t52135, t52138, t52140, t52161) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2590::<F>(t3447, t44579, t4904, t11545, t134, t461, t14726, t11579, t15338, t4899, t4928, t11570, t12648);
    (t52096, t52100, t52109, t52122, t52124, t52127, t52133, t52135, t52138, t52140, t52161)
}
