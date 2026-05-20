//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 816/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk816<F: Float>(t1393: F, t2114: F, t22577: F, t22580: F, t22583: F, t22587: F, t22594: F, t22599: F, t22605: F, t22608: F, t22610: F, t22612: F, t22614: F, t22616: F, t22618: F, t22950: F, t23833: F, t23835: F, t23837: F, t23860: F, t3652: F, t7412: F) -> F {
    let t24953 = F::new(2.0) * t1393 * t7412 - t2114 * t3652 - t22577 - t22580 - t22583 + t22587 + t22594 + t22599 + t22605 + t22608 - t22610 - t22612 - t22614 - t22616 - t22618 + t22950 - t23833 - t23835 + t23837 + t23860;
    t24953
}
