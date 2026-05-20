//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2989/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2989<F: Float>(t1041: F, t13969: F, t17642: F, t17906: F, t3117: F, t10390: F, t10403: F, t10413: F, t10965: F, t1618: F, t17920: F, t17976: F, t3041: F, t3048: F, t3071: F, t3132: F, t42511: F, t43155: F, t43157: F, t43161: F, t4596: F, t50062: F, t50077: F, t50302: F, t50445: F, t5681: F, t5900: F, t5909: F) -> F {
    let t62515 = t1041 * t13969 * t17642;
    let t62534 = t3117 * t17906;
    let t62544 = F::new(5.0) / F::new(10368.0) * t62515 - F::new(11.0) / F::new(486.0) * t43155 - F::new(5.0) / F::new(243.0) * t43157 - t10403 * t3071 * t5681 * t3132 / F::new(1152.0) + t10413 * t3071 * t5681 * t3041 / F::new(2304.0) + t42511 * t5909 / F::new(2304.0) - t43161 / F::new(13824.0) - t10965 * t5900 / F::new(2304.0) - t50445 * t1618 / F::new(144.0) - t62534 / F::new(1728.0) + t50062 / F::new(576.0) - t50302 * t4596 / F::new(72.0) + t3048 * t17976 / F::new(108.0) + F::new(2.0) / F::new(243.0) * t50077 + F::new(5.0) / F::new(3456.0) * t10390 * t17920;
    t62544
}
