//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 954/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk954<F: Float>(t1760: F, t6267: F, t3598: F, t6243: F, t11606: F, t11764: F, t20234: F, t974: F, t1743: F, t6169: F, t11487: F, t14766: F, t18494: F, t18505: F, t18512: F, t21747: F, t21751: F, t21789: F, t21792: F, t21795: F, t21802: F) -> (F, F, F, F, F) {
    let t22003 = t1760 * t6267;
    let t22004 = t3598 * t22003;
    let t22007 = t6243 * t1760;
    let t22008 = t11606 * t22007;
    let t22011 = t11764 * t20234;
    let t22012 = t974 * t22011;
    let t22015 = t6169 * t1743;
    let t22032 = t11487 - F::new(5.0) / F::new(9.0) * t14766 - t18494 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t18505 + t18512 / F::new(3.0) - F::new(2.0) / F::new(27.0) * t21802 + t21789 / F::new(3.0) + t21747 / F::new(6.0) - t21792 - t21751 - t21795 / F::new(6.0);
    (t22004, t22008, t22012, t22015, t22032)
}
