//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta301<F: Float>(t3069: F, t3180: F, t3036: F, t67: F, t3067: F, t3186: F, t3062: F, t820: F, t3200: F, t3051: F, t3072: F, t3070: F) -> (F, F, F, F, F, F, F, F) {
        let (t10390, t10401, t10403, t10408, t10413, t10422, t10423, t10424) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1361::<F>(t3069, t3180, t3036, t67, t3067, t3186, t3062, t820, t3200, t3051, t3072, t3070);
    (t10390, t10401, t10403, t10408, t10413, t10422, t10423, t10424)
}
