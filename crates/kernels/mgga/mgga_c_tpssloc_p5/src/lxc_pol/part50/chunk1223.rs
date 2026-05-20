//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1223/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1223<F: Float>(t2314: F, t32677: F, t4034: F, t112594: F, t113: F, t119685: F, t119792: F, t119795: F, t119796: F, t119799: F, t119810: F, t119811: F, t119815: F, t119820: F, t1393: F, t1459: F, t1849: F, t1869: F, t24980: F, t25958: F, t31224: F, t31240: F, t33080: F, t33155: F, t4037: F, t650: F, t6517: F, t652: F, t671: F, t672: F) -> F {
    let t119824 = F::new(2.0) * t2314 * t32677;
    let t119826 = F::new(2.0) * t4034 * t32677;
    let t119827 = -t113 * (t119685 + t119792) + t119795 - t119796 + t31240 * t1849 + t33155 * t1393 - F::new(6.0) * t119799 - F::new(2.0) * t652 * t33080 * t671 - F::new(2.0) * t31224 * t4037 - t650 * t33080 - F::new(4.0) * t6517 * t24980 - t119810 - F::new(4.0) * t119811 - F::new(2.0) * t1869 * t25958 - F::new(2.0) * t119815 * t672 - F::new(2.0) * t112594 * t1459 - F::new(2.0) * t119820 * t1459 - t119824 - t119826;
    t119827
}
