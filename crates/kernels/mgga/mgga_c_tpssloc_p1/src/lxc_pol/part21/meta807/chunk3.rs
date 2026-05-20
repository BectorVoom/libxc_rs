//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2813/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2813<F: Float>(t17100: F, t225: F, t10110: F, t13029: F, t13042: F, t13065: F, t13072: F, t13461: F, t1519: F, t1528: F, t16804: F, t17057: F, t17070: F, t17092: F, t259: F, t2591: F, t2713: F, t2720: F, t2742: F, t4142: F, t4147: F, t4265: F, t4273: F, t4301: F, t47568: F, t5631: F, t5636: F, t5637: F, t5658: F, t852: F, t855: F, t866: F, t9590: F, t9593: F) -> F {
    let t59466 = t17100 * t225;
    let t59475 = t2591 * t5631 * t259 + F::new(2.0) * t13029 * t1519 * t259 - F::new(4.0) * t47568 * t1528 + F::new(8.0) * t4147 * t13072 + F::new(8.0) * t13065 * t4273 - F::new(2.0) * t4147 * t13461 + F::new(2.0) * t16804 * t852 * t259 + F::new(4.0) * t17092 * t2720 + F::new(4.0) * t4142 * t4265 * t259 - F::new(4.0) * t13065 * t4301 + F::new(8.0) * t2713 * t17070 - F::new(2.0) * t9593 * t5658 - F::new(4.0) * t13042 * t4301 + F::new(4.0) * t2713 * t17057 - F::new(2.0) * t59466 * t866 - F::new(6.0) * t855 * t10110 * t5636 * t2742 + F::new(2.0) * t9590 * t5637;
    t59475
}
