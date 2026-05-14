//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1207/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1207<F: Float>(t31537: F, t7796: F, t31540: F, t27163: F, t8526: F, t119832: F, t26161: F, t26558: F, t15868: F, t1983: F, t8640: F, t1307: F, t24432: F, t24995: F, t33357: F, t33336: F, t6876: F) -> (F, F, F, F, F, F, F) {
    let t121134 = 2.0 * t31537 * t7796;
    let t121136 = 2.0 * t31540 * t7796;
    let t121138 = 2.0 * t8526 * t27163;
    let t121142 = 2.0 * t26161 * t26558 * t119832;
    let t121144 = t1983 * t8640 * t15868;
    let t121159 = 6.0 * t24995 * t24432 * t33357 * t1307;
    let t121160 = t6876 * t33336;
    (t121134, t121136, t121138, t121142, t121144, t121159, t121160)
}
