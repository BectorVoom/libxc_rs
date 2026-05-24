//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 651/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk651<F: Float>(t7203: F, t899: F, t20: F, t4764: F, t132: F, t1327: F, t140: F, t673: F, t465: F, t7472: F, t7344: F, t7552: F) -> (F, F, F, F, F, F) {
    let t34738 = t899 * t7203;
    let t34747 = t20 * t4764;
    let t34750 = t132 * t1327;
    let t34759 = t673 * t140;
    let t34760 = t465 * t34759;
    let t34761 = t7472 * t34760;
    let t34786 = t7344 * t7552;
    (t34738, t34747, t34750, t34760, t34761, t34786)
}
