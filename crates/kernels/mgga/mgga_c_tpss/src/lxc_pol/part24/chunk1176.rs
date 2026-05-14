//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1176/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1176<F: Float>(t13554: F, t1688: F, t3493: F, t5531: F, t19305: F, t3537: F, t93: F, t6234: F, t2056: F, t6112: F, t4347: F, t1165: F, t19596: F, t1338: F, t17916: F, t19431: F, t19457: F, t19462: F, t19649: F, t5514: F, t645: F) -> (F, F) {
    let t19651 = 2.0 * t13554 * t1688;
    let t19653 = 2.0 * t3493 * t5531;
    let t19655 = 2.0 * t19305 * t1688;
    let t19656 = t93 * t3537;
    let t19658 = 2.0 * t19656 * t1688;
    let t19660 = 2.0 * t6234 * t5531;
    let t19662 = 2.0 * t2056 * t6112;
    let t19664 = 2.0 * t4347 * t6112;
    let t19666 = 2.0 * t1165 * t19596;
    let t19667 = 2.0 * t1338 * t17916 + 2.0 * t1338 * t19457 + 2.0 * t19462 * t645 + 2.0 * t3537 * t5514 + t19431 + t19649 + t19651 + t19653 + t19655 + t19658 + t19660 + t19662 + t19664 + t19666;
    (t19656, t19667)
}
