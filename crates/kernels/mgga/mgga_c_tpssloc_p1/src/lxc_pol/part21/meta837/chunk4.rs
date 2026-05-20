//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2982/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2982<F: Float>(t14080: F, t4571: F, t14202: F, t4644: F, t1043: F, t1615: F, t375: F, t10408: F, t1041: F, t1044: F, t10957: F, t10965: F, t14229: F, t17890: F, t248: F, t2771: F, t2780: F, t3070: F, t3071: F, t3117: F, t42721: F, t49822: F, t49827: F, t49829: F, t49831: F, t49846: F, t5857: F, t5861: F, t5867: F, t59682: F, t59690: F, t62064: F) -> (F, F) {
    let t62282 = t14080 * t4571;
    let t62284 = t4644 * t14202;
    let t62291 = t375 * t1043 * t1615;
    let t62296 = t49822 / F::new(1152.0) - t42721 / F::new(6912.0) + t3070 * t3071 * t5867 * t2780 / F::new(4608.0) + F::new(5.0) / F::new(13824.0) * t3070 * t10408 * t5867 * t2771 + F::new(19.0) / F::new(1944.0) * t49827 - t49829 / F::new(324.0) + t49831 / F::new(648.0) + t3117 * t17890 / F::new(2304.0) + t1041 * t248 * t1044 * t59682 / F::new(4608.0) + F::new(5.0) / F::new(13824.0) * t10965 * t5861 + F::new(19.0) / F::new(2592.0) * t10957 * t5857 - t62282 / F::new(324.0) - t62284 / F::new(10368.0) - t1041 * t248 * t1044 * t59690 / F::new(1152.0) + t62064 * t62291 * t14229 / F::new(576.0) - F::new(5.0) / F::new(1728.0) * t49846;
    (t62291, t62296)
}
