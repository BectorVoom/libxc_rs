//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 601/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk601<F: Float>(t2629: F, t914: F, t2593: F, t2595: F, t904: F, t912: F, t2613: F, t895: F, t2618: F, t2621: F, t2464: F, t928: F, t1985: F, t926: F, t359: F, t361: F, t651: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2631 = 0.11696447245269292414e1 * t2629 * t914;
    let t2633 = t2593 * t2595 * t904;
    let t2635 = 0.11696447245269292414e1 * t912 * t2633;
    let t2637 = t895 * t2613 * t904;
    let t2639 = 0.5848223622634646207e0 * t912 * t2637;
    let t2640 = t2618 * t2595;
    let t2641 = t2640 * t2621;
    let t2643 = 0.17315859105681463759e2 * t912 * t2641;
    let t2644 = t928 * t2464;
    let t2645 = t2644 * t1985;
    let t2646 = t926 * t2645;
    let t2650 = t359 * t651 * t361;
    (t2631, t2633, t2635, t2637, t2639, t2641, t2643, t2645, t2646, t2650)
}
