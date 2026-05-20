//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta381<F: Float>(t17161: F, t2826: F, t136: F, t10304: F, t17152: F, t17167: F, t908: F, t17171: F, t17183: F, t17178: F, t10556: F, t10577: F, t13598: F, t13600: F, t13601: F, t13603: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F, t17180: F, t17185: F, t17189: F) -> (F, F, F, F, F, F, F) {
        let (t17241, t17244, t17247, t17250, t17253, t17256, t17271) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1346::<F>(t17161, t2826, t136, t10304, t17152, t17167, t908, t17171, t17183, t17178, t10556, t10577, t13598, t13600, t13601, t13603, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
    (t17241, t17244, t17247, t17250, t17253, t17256, t17271)
}
