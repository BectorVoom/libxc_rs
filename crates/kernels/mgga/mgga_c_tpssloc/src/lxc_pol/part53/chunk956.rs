//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 956/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk956<F: Float>(t114864: F, t114891: F, t112834: F, t112840: F, t112850: F, t112855: F, t225: F, t31974: F, t114932: F, t114943: F, t114672: F, t31984: F, t814: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t116557 = F::new(0.25587863262083522346e0) * t114864;
    let t116578 = F::new(0.10417915756705434098e0) * t114891;
    let t116608 = F::new(0.84334201618871038669e-2) * t112834;
    let t116610 = F::new(0.26915170729426927235e-3) * t112840;
    let t116613 = F::new(119.0) / F::new(1728.0) * t112850;
    let t116615 = F::new(0.18086994730174895102e0) * t112855;
    let t116645 = t31974 * t225;
    let t116648 = F::new(0.3289868133696452873e-1) * t114932;
    let t116654 = F::new(0.3289868133696452873e-1) * t114943;
    let t116673 = F::new(0.10417915756705434098e0) * t114672;
    let t116681 = t814 * t31984;
    (t116557, t116578, t116608, t116610, t116613, t116615, t116645, t116648, t116654, t116673, t116681)
}
