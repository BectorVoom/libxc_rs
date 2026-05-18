//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 648/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk648<F: Float>(t457: F, t6144: F, t460: F, t974: F, t1174: F, t1710: F, t1717: F, t3430: F, t3447: F, t463: F, t4887: F, t4889: F, t4897: F, t4917: F, t6109: F, t6120: F, t6123: F, t6127: F, t6131: F, t6141: F) -> (F, F, F) {
    let t6145 = t457 * t6144;
    let t6146 = t6145 * t460;
    let t6147 = t974 * t6146;
    let t6150 = F::new(0.81481481481481481481e-2) * t6109 * t463 - F::new(0.14814814814814814814e-2) * t4887 + F::new(0.14814814814814814814e-2) * t4889 * t1710 + F::new(0.44444444444444444444e-2) * t4889 * t1717 - t3430 - F::new(0.18518518518518518518e-3) * t4897 - F::new(0.55555555555555555554e-3) * t4917 + F::new(0.37037037037037037036e-3) * t1174 * t6120 + F::new(0.55555555555555555554e-3) * t3447 * t6123 - F::new(0.55555555555555555554e-3) * t1174 * t6127 - F::new(0.27777777777777777777e-3) * t1174 * t6131 - F::new(0.83333333333333333332e-3) * t1174 * t6141 - F::new(0.83333333333333333332e-3) * t1174 * t6147;
    (t6146, t6147, t6150)
}
