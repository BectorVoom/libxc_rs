//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 608/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk608<F: Float>(t2595: F, t2621: F, t2475: F, t2478: F, t2485: F, t2528: F, t2536: F, t2542: F, t2545: F, t2550: F, t2552: F, t2570: F, t2575: F, t2578: F, t2587: F, t2589: F, t2594: F, t2596: F, t2614: F, t2619: F, t305: F, t877: F, t886: F, t896: F, t905: F) -> (F, F) {
    let t2622 = t2595 * t2621;
    let t2625 = -0.310907e-1 * t2542 * t305 + 2.0 * t2545 * t886 - 2.0 * t2550 * t2552 + 1.0 * t877 * t2570 + 0.32163958997385070134e2 * t2575 * t2578 + t2475 - t2478 + t2485 - t2528 - t2536 - 0.19751673498613801407e-1 * t2587 + 0.11696447245269292414e1 * t2589 * t905 - 0.11696447245269292414e1 * t2594 * t2596 + 0.5848223622634646207e0 * t896 * t2614 + 0.17315859105681463759e2 * t2619 * t2622;
    (t2622, t2625)
}
