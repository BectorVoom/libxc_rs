//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1216/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1216<F: Float>(t5: F, t121072: F, t121126: F, t112: F, t31304: F, t7688: F, t31537: F, t7796: F, t31540: F, t27163: F, t8526: F, t119832: F, t26161: F, t26558: F, t15868: F, t1983: F, t8640: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t121128 = piecewise3(t8, 0.0, t121072 + t121126);
    let t121129 = t121128 * t112;
    let t121132 = 3.0 * t31304 * t7688;
    let t121134 = 2.0 * t31537 * t7796;
    let t121136 = 2.0 * t31540 * t7796;
    let t121138 = 2.0 * t8526 * t27163;
    let t121142 = 2.0 * t26161 * t26558 * t119832;
    let t121144 = t1983 * t8640 * t15868;
    (t121129, t121132, t121134, t121136, t121138, t121142, t121144)
}
