//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2126/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2126<F: Float>(t28827: F, t6876: F, t7684: F, t8944: F, t26164: F, t24995: F, t75203: F, t8643: F, t34999: F, t5308: F, t28813: F, t19577: F, t22574: F, t33136: F) -> (F, F, F, F, F, F) {
    let t96796 = F::new(6.0) * t6876 * t28827;
    let t96797 = t7684 * t8944;
    let t96799 = F::new(4.0) * t96797 * t26164;
    let t96802 = F::new(6.0) * t24995 * t8643 * t75203;
    let t96805 = F::new(12.0) * t24995 * t34999 * t5308;
    let t96807 = F::new(2.0) * t6876 * t28813;
    let t96813 = F::new(6.0) * t22574 * t33136 * t19577;
    (t96796, t96799, t96802, t96805, t96807, t96813)
}
