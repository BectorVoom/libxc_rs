//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1218/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1218<F: Float>(t1307: F, t24432: F, t24995: F, t33357: F, t33336: F, t6876: F, t115925: F, t25989: F, t22574: F, t32193: F, t27219: F, t8526: F, t25988: F, t36740: F, t26168: F, t8607: F) -> (F, F, F, F, F, F, F) {
    let t121159 = 6.0 * t24995 * t24432 * t33357 * t1307;
    let t121160 = t6876 * t33336;
    let t121162 = 3.0 * t115925 * t25989;
    let t121165 = 3.0 * t22574 * t32193 * t33357;
    let t121169 = 2.0 * t8526 * t27219;
    let t121174 = 3.0 * t22574 * t36740 * t25988;
    let t121177 = 3.0 * t8607 * t26168;
    (t121159, t121160, t121162, t121165, t121169, t121174, t121177)
}
