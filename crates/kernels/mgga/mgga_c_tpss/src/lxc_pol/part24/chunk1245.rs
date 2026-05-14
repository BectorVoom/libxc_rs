//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1245/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1245<F: Float>(t198: F, t206: F, t5585: F, t33457: F, t33459: F, t1729: F, t980: F, t2715: F, t5623: F, t33858: F, t347: F, t5610: F, t8471: F, t1718: F, t9040: F, t5620: F, t8953: F) -> (F, F, F, F, F, F, F) {
    let t61269 = t198 * t206 * t5585;
    let t61283 = t33457 * t33459;
    let t61285 = t1729 * t61283 * t980;
    let t61292 = t2715 * t5623;
    let t61296 = t33858 * t347;
    let t61322 = t5610 * t8471;
    let t61329 = 5.0 / 1296.0 * t1718 * t9040;
    let t61341 = t5620 * t8953;
    (t61269, t61285, t61292, t61296, t61322, t61329, t61341)
}
