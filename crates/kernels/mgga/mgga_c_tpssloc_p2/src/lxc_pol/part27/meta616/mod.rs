//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2093;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta616<F: Float>(t10984: F, t6717: F, t1036: F, t23557: F, t1933: F, t1937: F, t2250: F, t3200: F, t83015: F, t1030: F, t1058: F, t3068: F, sigma0: F, t25511: F, t6743: F, t23592: F, t23631: F, t974: F, t25721: F, t210: F, t23599: F, t23632: F, t23511: F, t23634: F, t23518: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t83167, t83172, t83206, t83215, t83220) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2093::<F>(t10984, t6717, t1036, t23557, t1933, t1937, t2250, t3200, t83015, t1030, t1058, t3068, sigma0);
        let (t83233, t83239, t83240, t83244, t83245, t83246, t83265) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2094::<F>(t25511, t6743, t23592, t23631, t974, t25721, t210, t23599, t23632, t23511, t23634, t23518);
    (t83167, t83172, t83206, t83215, t83220, t83233, t83239, t83240, t83244, t83245, t83246, t83265)
}
