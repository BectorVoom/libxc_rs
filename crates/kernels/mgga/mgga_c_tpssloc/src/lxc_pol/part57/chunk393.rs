//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 393/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk393<F: Float>(t1098: F, t1657: F, t1667: F, t699: F, t1128: F, t1675: F, t1147: F, t1687: F, t300: F, t1171: F, t1706: F, t1420: F, t972: F, t1709: F, t3431: F, t1174: F) -> (F, F, F, F, F, F, F, F) {
    let t4740 = t1657 * t1098;
    let t4770 = t699 * t1667;
    let t4797 = t1675 * t1128;
    let t4835 = t1687 * t1147;
    let t4869 = t300 * t1687;
    let t4887 = t1706 * t1171;
    let t4889 = t1420 * t972;
    let t4896 = t3431 * t1709;
    let t4897 = t1174 * t4896;
    (t4740, t4770, t4797, t4835, t4869, t4887, t4889, t4897)
}
