//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1108/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1108<F: Float>(t12075: F, t12086: F, t15737: F, t15740: F, t15744: F, t15751: F, t15754: F, t15757: F, t15760: F, t15764: F, t15767: F, t15771: F, t15788: F, t2955: F, t2974: F, t2999: F, t4163: F, t4185: F, t421: F, t9373: F, t9380: F, t9465: F) -> (F,) {
    let t15791 = 0.32163958997385070134e2 * t2955 * t15737 + 0.64327917994770140268e2 * t2955 * t15740 + 0.2069040516770936012e4 * t9465 * t15744 - 0.23392894490538584828e1 * t12086 * t4163 + 0.34631718211362927517e2 * t12075 * t4185 + 0.35089341735807877242e1 * t2999 * t15751 - 0.23392894490538584828e1 * t2974 * t15754 - 0.10389515463408878255e3 * t9373 * t15757 - 0.11696447245269292414e1 * t2974 * t15760 + 0.17315859105681463759e2 * t2999 * t15764 + 0.34631718211362927518e2 * t2999 * t15767 + 0.10254018858216406658e4 * t9380 * t15771 - 0.310907e-1 * t15788 * t421;
    (t15791,)
}
