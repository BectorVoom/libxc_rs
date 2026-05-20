//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta532<F: Float>(t1799: F, t5286: F, t1388: F, t6330: F, t6463: F, t1307: F, t5449: F, t671: F, t1851: F, t1372: F, t794: F, t213: F, t225: F) -> (F, F, F, F, F, F, F, F) {
        let (t74677, t75203, t75210, t75214, t75560, t75795, t80645, t80650) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1746::<F>(t1799, t5286, t1388, t6330, t6463, t1307, t5449, t671, t1851, t1372, t794, t213, t225);
    (t74677, t75203, t75210, t75214, t75560, t75795, t80645, t80650)
}
