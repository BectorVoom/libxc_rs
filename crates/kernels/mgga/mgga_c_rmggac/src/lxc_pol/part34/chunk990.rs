//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 990/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk990<F: Float>(t77366: F, t530: F, t71760: F, t74981: F, t74983: F, t74986: F, t77337: F, t77340: F, t77343: F, t77347: F, t77349: F, t77352: F, t77353: F, t77357: F, t77361: F, t77362: F, t77363: F, t77365: F) -> F {
    let t77367 = F::new(0.68186654135613354322e-2) * t77366;
    let t77368 = t77337 - t77340 + t77343 + t77347 - t74981 + t77349 - t74983 - F::new(0.2363e1) * t530 * t71760 + t77352 + t74986 - t77353 + t77357 - t77361 + t77362 + t77363 + t77365 + t77367;
    t77368
}
