//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1092/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1092<F: Float>(t15139: F, t15162: F, t15213: F, t15232: F, t300: F, t3411: F, t4875: F, t14958: F, t14963: F, t14969: F, t14971: F, t15038: F, t15040: F, t15043: F, t15046: F, t15048: F, t15050: F, t15053: F, t15056: F, t15059: F, t15063: F, t15066: F, t15070: F) -> (F, F, F) {
    let t15235 = t300 * (t15139 + t15162 + t15213 + t15232);
    let t15237 = 0.23392894490538584828e1 * t3411 * t4875;
    let t15238 = -t14958 + t14963 - t14969 - t14971 - t15038 - t15040 - t15043 + t15046 - t15048 + t15050 - t15053 - t15056 - t15059 + t15063 + t15066 + t15070 + t15235 + t15237;
    (t15235, t15237, t15238)
}
