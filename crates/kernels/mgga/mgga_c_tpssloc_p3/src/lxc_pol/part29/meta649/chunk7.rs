//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2165/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2165<F: Float>(t86752: F, t86801: F, t87952: F, t88001: F, t12606: F, t3: F, t12915: F, t13487: F, t13191: F, t13471: F, t1530: F, t16596: F, t1877: F, t1915: F, t193: F, t202: F, t22959: F, t23290: F, t2379: F, t25013: F, t2522: F, t25358: F, t25365: F, t25374: F, t2553: F, t4119: F, t4314: F, t57893: F, t57912: F, t6666: F, t6670: F, t7541: F, t81525: F, t81539: F, t82312: F, t86717: F, t868: F, t86836: F, t870: F, t87944: F) -> (F, F, F) {
    let t88003 = t86752 + t86801 + t87952 + t88001;
    let t88391 = t3 * t12606;
    let t89733 = t12915 * t13487;
    let t89775 = -F::new(2.0) * t1877 * t86836 * t868 + F::new(12.0) * t22959 * t89733 + F::new(4.0) * t1877 * t81539 * t25374 - F::new(6.0) * t2522 * t23290 * t16596 - F::new(12.0) * t25013 * t57912 - F::new(6.0) * t2522 * t6670 * t57893 + F::new(3.0) * t2522 * t7541 * t2553 - F::new(6.0) * t2522 * t25358 * t13487 + F::new(6.0) * t2522 * t6666 * t4119 - F::new(6.0) * t2522 * t23290 * t25365 - F::new(6.0) * t1877 * t82312 * t86717 + F::new(12.0) * t4314 * t1915 * t13191 - t1877 * t6670 * t13471 + F::new(6.0) * t4314 * t7541 * t2379 - t1877 * t81525 * t1530 + t193 * t202 * t87944 * t870;
    (t88003, t88391, t89775)
}
