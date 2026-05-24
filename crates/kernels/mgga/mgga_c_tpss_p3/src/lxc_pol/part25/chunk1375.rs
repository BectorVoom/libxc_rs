//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1375/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1375<F: Float>(t118: F, t13547: F, t13974: F, t1760: F, t1834: F, t18547: F, t18690: F, t18710: F, t19579: F, t20218: F, t20219: F, t20221: F, t20357: F, t20361: F, t2056: F, t21027: F, t21576: F, t21858: F, t25232: F, t3499: F, t3538: F, t4341: F, t4525: F, t51642: F, t51664: F, t5463: F, t5706: F, t5709: F, t5801: F, t5905: F, t5909: F, t6243: F, t6245: F, t626: F, t6323: F, t65533: F, t67782: F, t68823: F, t71884: F, t72425: F, t72576: F) -> F {
    let t72593 = -F::new(6.0) * t18547 * t18690 * t51642 + F::new(3.0) * t1760 * t71884 * t5709 - F::new(2.0) * t1760 * t20218 * t4525 + F::new(3.0) * t1760 * t5909 * t68823 + F::new(6.0) * t5706 * t21858 - F::new(6.0) * t65533 * t20221 - F::new(2.0) * t6243 * t20361 + t1834 * t13974 + F::new(3.0) * t1760 * t18710 * t21027 + F::new(6.0) * t1760 * t67782 * t6245 + F::new(2.0) * t19579 * t20357 * t51664 - t118 * (t72425 + t72576) + F::new(2.0) * t6243 * t20219 - F::new(4.0) * t2056 * t21576 - F::new(4.0) * t3499 * t21576 - F::new(4.0) * t626 * t4341 * t6323 + t5905 * t5463 - F::new(4.0) * t25232 * t3538 - F::new(2.0) * t5801 * t13547;
    t72593
}
