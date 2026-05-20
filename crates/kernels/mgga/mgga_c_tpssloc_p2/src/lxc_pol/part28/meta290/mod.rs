//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta290<F: Float>(t597: F, t61: F, t241: F, t244: F, t248: F, t238: F, t154: F, t9569: F, t222: F, t2606: F, t9573: F, t805: F, t9541: F) -> (F, F, F, F, F, F, F) {
        let (t10022, t10024, t10026, t10027, t10029, t10030, t10036) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1199::<F>(t597, t61, t241, t244, t248, t238, t154, t9569, t222, t2606, t9573, t805, t9541);
    (t10022, t10024, t10026, t10027, t10029, t10030, t10036)
}
