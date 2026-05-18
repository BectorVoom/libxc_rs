//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1377/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1377<F: Float>(t10336: F, t1920: F, t1949: F, t337: F, t9276: F, t1887: F, t23323: F, t6781: F, t2966: F, t6805: F, t135: F, t23631: F, t6688: F) -> (F, F, F, F, F, F) {
    let t82799 = F::new(0.30461741978670859935e-2) * t1920 * t10336 * t1949;
    let t82802 = t9276 * t337;
    let t82803 = t82802 * t1887;
    let t82806 = t23323 * t6781;
    let t82809 = t1920 * t2966 * t6805;
    let t82822 = t23631 * t135 * t6688;
    (t82799, t82802, t82803, t82806, t82809, t82822)
}
