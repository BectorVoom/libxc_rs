//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 965/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk965<F: Float>(t14618: F, t8368: F, t14421: F, t2868: F, t75119: F, t75124: F, t2010: F, t2265: F, t8342: F, t2415: F, t8048: F, t8188: F) -> (F, F, F, F, F, F, F) {
    let t77457 = t8368 * t14618;
    let t77458 = F::new(0.34093327067806677161e-2) * t77457;
    let t77463 = F::new(0.11974241701863808564e0) * t2868 * t14421;
    let t77464 = F::new(0.1702583995731913576e-4) * t75119;
    let t77465 = F::new(0.85129199786595678799e-5) * t75124;
    let t77467 = t2010 * t8342 * t2265;
    let t77468 = F::new(0.36021158228745895953e-3) * t77467;
    let t77470 = t2010 * t2415 * t8048;
    let t77471 = F::new(0.36021158228745895953e-3) * t77470;
    let t77473 = t2010 * t2415 * t8188;
    (t77458, t77463, t77464, t77465, t77468, t77471, t77473)
}
