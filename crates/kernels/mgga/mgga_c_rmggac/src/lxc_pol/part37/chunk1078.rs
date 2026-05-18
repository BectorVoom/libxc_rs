//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1078/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1078<F: Float>(t71486: F, t75440: F, t75455: F, t77593: F, t77595: F, t77597: F, t77598: F, t77605: F, t77606: F, t77608: F, t77614: F, t77620: F, t77621: F, t77624: F, t77625: F, t77626: F) -> F {
    let t80253 = t77593 + t77595 - t75440 + t77597 + t77598 - t77605 - t77606 + t77608 + t77614 + t77620 - t71486 + t77621 - t77624 - t77625 + t77626 - F::new(0.70077224371605468748e-6) * t75455;
    t80253
}
