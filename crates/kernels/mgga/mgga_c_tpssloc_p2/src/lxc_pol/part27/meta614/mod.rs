//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2089;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta614<F: Float>(t23437: F, t3103: F, t10472: F, t10474: F, t10478: F, t23535: F, t10948: F, t23540: F, t6753: F, t10961: F, t6754: F, t3077: F, t6764: F, sigma0: F, t1937: F, t607: F, t6722: F, t10375: F, t1942: F, t1036: F, t23551: F, t23562: F, t343: F, t83032: F, t210: F, t23322: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t83046, t83054, t83058, t83061, t83065, t83068, t83071) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2089::<F>(t23437, t3103, t10472, t10474, t10478, t23535, t10948, t23540, t6753, t10961, t6754, t3077, t6764, sigma0);
        let (t83075, t83080, t83082, t83085, t83092) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2090::<F>(t1937, t607, t6722, t10375, t1942, t1036, t23551, t23562, t343, t83032, t210, t23322);
    (t83046, t83054, t83058, t83061, t83065, t83068, t83071, t83075, t83080, t83082, t83085, t83092)
}
