//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 764/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk764<F: Float>(t1081: F, t5177: F, t3001: F, t5161: F, t1054: F, t1073: F, t1531: F, t1543: F, t2930: F, t2955: F, t2974: F, t2999: F, t4120: F, t4158: F, t421: F, t5078: F, t5080: F, t5084: F, t5116: F, t5119: F, t5124: F, t5130: F, t5146: F, t5149: F, t5157: F, t5162: F) -> (F, F, F) {
    let t5178 = t5177 * t1081;
    let t5181 = t5161 * t3001;
    let t5184 = -0.310907e-1 * t5124 * t421 + 2.0 * t4120 * t1531 - 2.0 * t2930 * t5130 + 1.0 * t1054 * t5146 + 0.32163958997385070134e2 * t2955 * t5149 + t5078 - t5080 + t5084 - t5116 - t5119 - 0.19751673498613801407e-1 * t5157 + 0.11696447245269292414e1 * t4158 * t1543 - 0.11696447245269292414e1 * t2974 * t5162 + 0.5848223622634646207e0 * t1073 * t5178 + 0.17315859105681463759e2 * t2999 * t5181;
    (t5178, t5181, t5184)
}
