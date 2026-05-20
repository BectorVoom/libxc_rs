//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1375/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1375<F: Float>(t122920: F, t1873: F, t33690: F, t6534: F, t120123: F, t120125: F, t120127: F, t120129: F, t120131: F, t120132: F, t120134: F, t120137: F, t120140: F, t120141: F, t120143: F, t120146: F, t120149: F, t120151: F, t120153: F, t120163: F, t120165: F) -> F {
    let t123084 = t122920 * t1873;
    let t123086 = t33690 * t6534;
    let t123088 = t120123 + t120125 + t120127 + t120129 + t120131 + F::new(2.0) * t120132 + F::new(2.0) * t120134 + t120137 + t120140 + F::new(2.0) * t120141 + F::new(2.0) * t120143 + F::new(2.0) * t120146 + F::new(2.0) * t120149 + F::new(2.0) * t120151 + F::new(2.0) * t120153 + F::new(2.0) * t120163 + t120165 + F::new(2.0) * t123084 + F::new(2.0) * t123086;
    t123088
}
