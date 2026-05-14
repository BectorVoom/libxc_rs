//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 758/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk758<F: Float>(t1326: F, t75307: F, t1322: F, t880: F, t899: F, t75311: F, t68815: F, t15105: F, t352: F, t68729: F, t14011: F, t14052: F, t8615: F, t13862: F, t3120: F, t8581: F) -> (F, F, F, F, F, F, F, F) {
    let t75770 = t1326 * t75307;
    let t75771 = t899 * t880 * t1322 * t75770;
    let t75773 = t1326 * t75311;
    let t75774 = t68815 * t75773;
    let t75779 = t1326 * t15105 * t352;
    let t75780 = t68729 * t75779;
    let t75789 = t14052 * t14011 * t8615;
    let t75792 = t3120 * t13862 * t8581;
    (t75770, t75771, t75773, t75774, t75779, t75780, t75789, t75792)
}
