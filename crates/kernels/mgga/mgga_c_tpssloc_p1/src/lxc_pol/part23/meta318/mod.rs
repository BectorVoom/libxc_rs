//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1077;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta318<F: Float>(t21749: F, t4908: F, t18420: F, t4904: F, t20246: F, t338: F, t11556: F, t15300: F, t15364: F, t15376: F, t18447: F, t18452: F, t18455: F, t18458: F, t18460: F, t18489: F, t18530: F, t18533: F, t18536: F, t3447: F, t463: F, t4889: F, t6123: F, t6127: F, t6131: F, t22085: F, t225: F, t68: F, t484: F, t1177: F, t1196: F, t20217: F, t974: F, t11848: F, t20234: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22090, t22095, t22104, t22112) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1077::<F>(t21749, t4908, t18420, t4904, t20246, t338, t11556, t15300, t15364, t15376, t18447, t18452, t18455, t18458, t18460, t18489, t18530, t18533, t18536, t3447, t463, t4889, t6123, t6127, t6131);
        let (t22113, t22114, t22115, t22116, t22119, t22128, t22129, t22132) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1078::<F>(t22085, t22112, t225, t68, t484, t1177, t21749, t1196, t20217, t974, t11848, t20234);
    (t22090, t22095, t22104, t22113, t22114, t22115, t22116, t22119, t22128, t22129, t22132)
}
