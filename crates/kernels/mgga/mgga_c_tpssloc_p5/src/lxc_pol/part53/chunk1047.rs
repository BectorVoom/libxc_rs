//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1047/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1047<F: Float>(t115925: F, t117006: F, t1266: F, t1845: F, t19456: F, t19577: F, t1983: F, t2039: F, t22574: F, t2314: F, t24995: F, t26161: F, t26558: F, t26870: F, t26974: F, t27171: F, t27226: F, t32200: F, t32220: F, t33854: F, t33874: F, t33886: F, t33893: F, t34067: F, t34682: F, t34685: F, t38024: F, t4028: F, t4034: F, t5308: F, t652: F, t671: F, t6876: F, t6999: F, t7042: F, t7216: F, t7458: F, t7802: F, t8721: F, t9003: F) -> F {
    let t124428 = -F::new(2.0) * t33893 * t1266 - t1983 * t33854 * t6999 + F::new(2.0) * t6876 * t33886 - F::new(6.0) * t115925 * t26974 + F::new(6.0) * t24995 * t38024 * t5308 + F::new(4.0) * t26161 * t26558 * t1845 * t7216 + F::new(6.0) * t22574 * t117006 * t19577 - F::new(4.0) * t34682 * t7802 - F::new(4.0) * t34685 * t7802 - F::new(4.0) * t9003 * t27226 - F::new(4.0) * t7042 * t27171 - F::new(4.0) * t7458 * t32200 - F::new(4.0) * t2314 * t33874 - F::new(4.0) * t4034 * t33874 - F::new(4.0) * t652 * t26870 * t2039 - F::new(2.0) * t652 * t34067 * t671 - F::new(4.0) * t19456 * t8721 - F::new(4.0) * t4028 * t32220;
    t124428
}
