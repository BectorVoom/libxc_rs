//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 970/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk970<F: Float>(t11347: F, t885: F, t3857: F, t895: F, t1441: F, t2618: F, t10963: F, t11286: F, t11289: F, t11294: F, t1437: F, t1449: F, t2545: F, t2570: F, t2578: F, t2614: F, t2622: F, t305: F, t3822: F, t3845: F, t3860: F, t877: F, t8837: F, t886: F, t8894: F, t905: F) -> (F,) {
    let t11348 = t11347 * t885;
    let t11351 = t3857 * t895;
    let t11356 = t1441 * t2618;
    let t11361 = -0.310907e-1 * t11286 * t305 + 2.0 * t11289 * t886 + 1.0 * t3822 * t2570 + 0.32163958997385070134e2 * t11294 * t2578 + 1.0 * t8837 * t1437 + 2.0 * t2545 * t3845 + 1.0 * t877 * t11348 + 0.11696447245269292414e1 * t11351 * t905 + 0.5848223622634646207e0 * t3860 * t2614 + 0.17315859105681463759e2 * t11356 * t2622 + 0.5848223622634646207e0 * t8894 * t1449 - t10963;
    (t11361,)
}
