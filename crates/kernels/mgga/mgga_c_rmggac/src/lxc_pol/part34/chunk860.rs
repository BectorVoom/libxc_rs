//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 860/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk860<F: Float>(t75736: F, t75739: F, t1550: F, t2228: F, t2347: F, t1624: F, t3204: F, t71589: F, t71607: F, t739: F, t75733: F, t77803: F, t77804: F, t77807: F, t77810: F, t77812: F, t77816: F, t77819: F, t77820: F, t77823: F) -> (F,) {
    let t77824 = 0.10909864661698136691e0 * t75736;
    let t77825 = 0.21819729323396273382e0 * t75739;
    let t77827 = t1550 * t2228 * t2347;
    let t77828 = 0.2993560425465952141e-1 * t77827;
    let t77829 = t77803 - t77804 + t77807 + t77810 + t77812 + t71589 - 0.11974241701863808564e0 * t1550 * t3204 * t1624 - 0.59871208509319042821e-1 * t739 * t77816 + t77819 - t77820 + 0.29085809927086856923e-4 * t75733 + t77823 + t77824 - t77825 + t77828 + t71607;
    (t77829,)
}
