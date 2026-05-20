//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1838;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta564<F: Float>(t1519: F, t794: F, t23164: F, t6555: F, t23035: F, t23241: F, t25224: F, t7480: F, t81632: F, t25038: F, t25040: F, t82159: F, t23030: F, t25035: F, t23228: F, t7479: F, t81573: F, t22986: F, t23270: F, t25191: F, t2742: F, t25059: F, t6562: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t86893, t86895, t86901, t86903, t86909) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1838::<F>(t1519, t794, t23164, t6555, t23035, t23241, t25224, t7480, t81632, t25038, t25040, t82159);
        let (t86911, t86916, t86923, t86928) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1839::<F>(t23030, t25035, t23228, t7479, t81573, t22986, t23270, t25191, t2742, t25059, t6562, t794);
    (t86893, t86895, t86901, t86903, t86909, t86911, t86916, t86923, t86928)
}
