//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1244;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta318<F: Float>(t3247: F, t460: F, t2244: F, t1176: F, t134: F, t1184: F, t3451: F, t3447: F, t3448: F, t3475: F, t1239: F, t68: F, t225: F, t3484: F, t1222: F, t3567: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F, t485: F, t248: F, t3516: F, t3570: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11583, t11584, t11588, t11589, t11591, t11593, t11606) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1244::<F>(t3247, t460, t2244, t1176, t134, t1184, t3451, t3447, t3448, t3475, t1239, t68);
        let (t11613, t11642, t11644, t11649, t11651) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1245::<F>(t225, t3484, t1222, t3567, t1203, t3540, t2393, t374, t486, t485, t248, t3516, t3570);
    (t11583, t11584, t11588, t11589, t11591, t11593, t11606, t11613, t11642, t11644, t11649, t11651)
}
