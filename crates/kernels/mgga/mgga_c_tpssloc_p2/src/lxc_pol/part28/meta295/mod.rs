//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1205;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta295<F: Float>(t2250: F, t2989: F, t2775: F, t343: F, t2244: F, t2987: F, t3014: F, t2262: F, t972: F, t2960: F, t2971: F, t2970: F, t2995: F, t973: F, t2769: F, t40: F, t698: F, t986: F, t135: F, t3010: F, t241: F, t625: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10245, t10254, t10255, t10259, t10263, t10267, t10273) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1205::<F>(t2250, t2989, t2775, t343, t2244, t2987, t3014, t2262, t972, t2960, t2971, t2970, t2995);
        let (t10274, t10277, t10287, t10290, t10292, t10294) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1206::<F>(t10273, t973, t2769, t40, t698, t986, t135, t3010, t241, t625, t281, t283);
    (t10245, t10254, t10255, t10259, t10263, t10267, t10274, t10277, t10287, t10290, t10292, t10294)
}
