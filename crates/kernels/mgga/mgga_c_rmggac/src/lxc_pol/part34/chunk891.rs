//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 891/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk891<F: Float>(t14125: F, t14131: F, t8431: F, t739: F, t74292: F, t7577: F, t1326: F, t15144: F, t321: F, t68729: F, t333: F, t70585: F) -> (F, F, F, F, F, F, F) {
    let t75946 = t14131 * t14125 * t8431;
    let t75951 = F::new(0.5987120850931904282e-1) * t739 * t7577 * t74292;
    let t75953 = t1326 * t15144 * t321;
    let t75954 = t68729 * t75953;
    let t75956 = t15144 * t333;
    let t75957 = t1326 * t75956;
    let t75958 = t70585 * t75957;
    (t75946, t75951, t75953, t75954, t75956, t75957, t75958)
}
