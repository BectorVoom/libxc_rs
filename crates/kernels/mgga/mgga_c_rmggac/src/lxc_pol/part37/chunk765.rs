//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 765/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk765<F: Float>(t739: F, t74292: F, t7577: F, t1326: F, t15144: F, t321: F, t68729: F, t333: F, t70585: F, t1322: F, t235: F, t29837: F, t352: F, t27: F, t9145: F, t16129: F, t70489: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t75951 = 0.5987120850931904282e-1 * t739 * t7577 * t74292;
    let t75953 = t1326 * t15144 * t321;
    let t75954 = t68729 * t75953;
    let t75956 = t15144 * t333;
    let t75957 = t1326 * t75956;
    let t75958 = t70585 * t75957;
    let t75961 = t235 * t29837 * t1322;
    let t75962 = t15144 * t352;
    let t75963 = t1326 * t75962;
    let t75964 = t75961 * t75963;
    let t75966 = t27 * t9145;
    let t75968 = t70489 * t16129 * t75966;
    (t75951, t75953, t75954, t75956, t75957, t75958, t75962, t75963, t75964, t75968)
}
