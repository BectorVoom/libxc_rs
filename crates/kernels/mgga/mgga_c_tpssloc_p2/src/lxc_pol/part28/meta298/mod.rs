//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1210;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta298<F: Float>(t3069: F, t3180: F, t3036: F, t67: F, t3067: F, t3186: F, t3062: F, t820: F, t3200: F, t3051: F, t3072: F, t3070: F, t1005: F, t3082: F, t1036: F, t3094: F, t3089: F, t248: F, t2780: F, t1041: F, t121: F, t3061: F, t2771: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10390, t10401, t10403, t10408, t10413, t10422, t10424) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1210::<F>(t3069, t3180, t3036, t67, t3067, t3186, t3062, t820, t3200, t3051, t3072, t3070);
        let (t10436, t10441, t10449, t10455, t10459) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1211::<F>(t1005, t3082, t1036, t3094, t3089, t248, t2780, t3051, t1041, t121, t3061, t2771);
    (t10390, t10401, t10403, t10408, t10413, t10422, t10424, t10436, t10441, t10449, t10455, t10459)
}
