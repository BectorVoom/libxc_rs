//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2229;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta543<F: Float>(t1216: F, t5971: F, t11668: F, t1090: F, t6225: F, t3578: F, t11697: F, t6191: F, t3577: F, t248: F, t3570: F, t6219: F, t1213: F, t5979: F, t5975: F, t11678: F, t11709: F, t11734: F, t1227: F, t15438: F, t15569: F, t18342: F, t18346: F, t18357: F, t18360: F, t3490: F, t4954: F, t4984: F, t5014: F, t5019: F, t6203: F, t6227: F, t6232: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18363, t18364, t18367, t18368, t18371, t18372, t18375) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2229::<F>(t1216, t5971, t11668, t1090, t6225, t3578, t11697, t6191, t3577, t248, t3570, t6219);
        let (t18382, t18383, t18386, t18387, t18390) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2230::<F>(t1213, t18375, t1216, t5979, t3578, t5975, t11678, t11709, t11734, t1227, t15438, t15569, t18342, t18346, t18357, t18360, t18364, t18368, t18372, t3490, t3577, t4954, t4984, t5014, t5019, t6203, t6227, t6232);
    (t18363, t18364, t18367, t18368, t18371, t18375, t18382, t18383, t18386, t18387, t18390)
}
