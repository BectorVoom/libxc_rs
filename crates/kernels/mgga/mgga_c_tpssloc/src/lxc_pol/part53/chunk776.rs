//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 776/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk776<F: Float>(t225: F, t7085: F, t23251: F, t23261: F, t2752: F, t7109: F) -> (F, F, F, F) {
    let t24305 = t7085 * t225;
    let t24318 = F::new(0.52089578783527170489e-1) * t23251;
    let t24321 = F::new(0.12793931631041761173e0) * t23261;
    let t24339 = t7109 * t2752;
    (t24305, t24318, t24321, t24339)
}
