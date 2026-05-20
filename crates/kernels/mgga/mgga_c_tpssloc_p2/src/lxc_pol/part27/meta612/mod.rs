//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2086;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta612<F: Float>(t1012: F, t10515: F, t6753: F, t1933: F, t23479: F, t82916: F, t23413: F, t344: F, t6740: F, t1016: F, t3034: F, t1930: F, t1015: F, t23472: F, t23503: F, t10423: F, t23419: F, t23418: F, t3180: F, t10401: F, t23417: F, t3186: F, t3158: F, t6712: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t82964, t82971, t82981, t82986) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2086::<F>(t1012, t10515, t6753, t1933, t23479, t82916, t23413, t344, t6740, t1016, t3034, t1930);
        let (t82996, t83004, t83008, t83015, t83016, t83025) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2087::<F>(t1015, t23472, t23503, t10423, t23419, t23418, t3180, t10401, t23417, t3186, t3158, t6712);
    (t82964, t82971, t82981, t82986, t82996, t83004, t83008, t83015, t83016, t83025)
}
