//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2987/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2987<F: Float>(t17906: F, t3048: F, t1041: F, t248: F, t43338: F, t5677: F, t1022: F, t10403: F, t10408: F, t10413: F, t10937: F, t10957: F, t13532: F, t13537: F, t13542: F, t14211: F, t1616: F, t17593: F, t17923: F, t18016: F, t18025: F, t18030: F, t2775: F, t2960: F, t3070: F, t3071: F, t3123: F, t3131: F, t42397: F, t42505: F, t42541: F, t4347: F, t49616: F, t50027: F, t5900: F, t62055: F, t62059: F, t62291: F) -> F {
    let t62441 = t3048 * t17906;
    let t62445 = t1041 * t248 * t43338 * t5677;
    let t62475 = -t62055 * t62291 * t3131 * t2775 * t62059 / F::new(288.0) + t42541 * t18016 / F::new(576.0) + t18030 * t3123 / F::new(3072.0) - F::new(19.0) / F::new(1296.0) * t10957 * t5900 + t62441 / F::new(324.0) - F::new(5.0) / F::new(62208.0) * t62445 + t10937 * t18025 / F::new(108.0) + F::new(5.0) / F::new(6912.0) * t3070 * t10408 * t1616 * t13532 + F::new(5.0) / F::new(2592.0) * t3070 * t42397 * t1616 * t13537 - t3070 * t3071 * t1616 * t13542 / F::new(576.0) - t10413 * t3071 * t49616 * t17923 / F::new(2304.0) + t10403 * t3071 * t14211 * t4347 * t1022 / F::new(576.0) - t42505 * t18016 / F::new(108.0) + t2960 * t17593 / F::new(27.0) - t50027 / F::new(108.0);
    t62475
}
