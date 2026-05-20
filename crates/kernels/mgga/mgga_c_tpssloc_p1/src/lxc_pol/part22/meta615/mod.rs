//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2143;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta615<F: Float>(t11365: F, t300: F, t1714: F, t44583: F, t3447: F, t3451: F, t44584: F, t4904: F, t11588: F, t4928: F, t461: F, t4729: F, t15418: F, t11557: F, t4889: F, t1174: F, t1716: F, t2402: F, t4930: F, t698: F, t44620: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51848, t51968, t51971, t51981, t52036, t52057) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2143::<F>(t11365, t300, t1714, t44583, t3447, t3451, t44584, t4904, t11588, t4928, t461, t4729);
        let (t52058, t52059, t52074, t52081, t52085, t52096) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2144::<F>(t52057, t15418, t1714, t11557, t4889, t1174, t1716, t2402, t4930, t698, t44620, t461, t60);
    (t51848, t51968, t51971, t51981, t52036, t52058, t52059, t52074, t52081, t52085, t52096)
}
