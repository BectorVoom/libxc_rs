//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 481/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk481<F: Float>(t1020: F, t1041: F, t1618: F, t1622: F, t3039: F, t3070: F, t3084: F, t3130: F, t3160: F, t378: F, t4572: F, t4604: F, t4625: F, t4631: F, t4641: F, t4644: F, t5857: F, t5861: F, t5869: F, t5875: F, t5880: F, t5885: F, t5890: F, t5894: F, t5900: F, t5905: F, t5909: F, t973: F) -> F {
    let t5914 = t1041 * t5857 / F::new(4608.0) + F::new(5.0) / F::new(13824.0) * t1041 * t5861 + t4644 * t1622 / F::new(2304.0) + t1020 * t5869 / F::new(3072.0) + t3130 * t5875 / F::new(1536.0) - t3039 * t5880 / F::new(3072.0) - t3160 + t4625 / F::new(2304.0) - t973 * t5885 / F::new(144.0) + t4604 / F::new(432.0) + t973 * t5890 / F::new(288.0) + t973 * t5894 / F::new(216.0) + t4572 / F::new(3456.0) + t4631 / F::new(2304.0) - t1041 * t5900 / F::new(2304.0) - t3084 + t5905 * t378 / F::new(3072.0) + t3070 * t5909 / F::new(2304.0) + t4641 * t1618 / F::new(1536.0);
    t5914
}
