//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 962/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk962<F: Float>(t25758: F, t4664: F, t1052: F, t1066: F, t14529: F, t1635: F, t1956: F, t23327: F, t23346: F, t23359: F, t23372: F, t25447: F, t25450: F, t25453: F, t25732: F, t25736: F, t25739: F, t25743: F, t25751: F, t25755: F, t25757: F, t3026: F, t6687: F, t7557: F, t7600: F) -> (F, F) {
    let t25759 = t25758 * t4664;
    let t25762 = F::new(0.82246703342411321825e-2) * t6687 * t25447 + F::new(0.91385225936012579807e-3) * t25450 + F::new(2.0) * t1052 * t25453 - t1052 * t25732 + F::new(0.21932454224643019153e-1) * t23346 * t7557 - F::new(0.27415567780803773942e-2) * t25736 + F::new(0.16449340668482264365e-1) * t6687 * t25739 + F::new(2.0) * t1052 * t25743 - t23359 - t23372 * t1635 + F::new(2.0) * t3026 * t7600 - F::new(0.27415567780803773942e-2) * t23327 * t25751 - t14529 * t1956 - t25755 * t1066 - F::new(6.0) * t25757 * t25759;
    (t25759, t25762)
}
