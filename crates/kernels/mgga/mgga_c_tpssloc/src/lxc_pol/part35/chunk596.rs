//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 596/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk596<F: Float>(t5790: F, t951: F, t2932: F, t5774: F, t1569: F, t1581: F, t2861: F, t2886: F, t2905: F, t2930: F, t311: F, t4411: F, t4449: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5737: F, t5743: F, t5759: F, t5762: F, t5770: F, t5775: F, t924: F, t943: F) -> (F, F, F) {
    let t5791 = t5790 * t951;
    let t5794 = t5774 * t2932;
    let t5797 = -0.310907e-1 * t5737 * t311 + 2.0 * t4411 * t1569 - 2.0 * t2861 * t5743 + 1.0 * t924 * t5759 + 0.32163958997385070134e2 * t2886 * t5762 + t5691 - t5693 + t5697 - t5729 - t5732 - 0.19751673498613801407e-1 * t5770 + 0.11696447245269292414e1 * t4449 * t1581 - 0.11696447245269292414e1 * t2905 * t5775 + 0.5848223622634646207e0 * t943 * t5791 + 0.17315859105681463759e2 * t2930 * t5794;
    (t5791, t5794, t5797)
}
