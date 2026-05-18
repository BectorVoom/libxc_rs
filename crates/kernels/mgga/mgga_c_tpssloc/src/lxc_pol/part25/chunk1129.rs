//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1129/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1129<F: Float>(t22690: F, t6639: F, t81573: F, t2379: F, t25038: F, t252: F, t6646: F, t829: F, t22986: F, t22997: F, t2647: F, t1887: F, t23069: F) -> (F, F, F, F) {
    let t81575 = t81573 * t22690 * t6639;
    let t81585 = t25038 * t6646 * t252 * t2379 * t829;
    let t81589 = t22986 * t6646 * t22997 * t2647;
    let t81591 = t23069 * t1887;
    (t81575, t81585, t81589, t81591)
}
