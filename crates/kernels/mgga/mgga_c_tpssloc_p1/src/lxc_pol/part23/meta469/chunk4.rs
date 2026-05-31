//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1390/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1390<F: Float>(t10403: F, t10408: F, t1041: F, t10413: F, t10876: F, t1539: F, t1616: F, t17712: F, t21118: F, t21391: F, t3070: F, t3071: F, t42309: F, t42388: F, t42624: F, t4342: F, t4582: F, t4583: F, t5398: F, t5681: F, t5685: F, t5873: F, t5878: F, t5909: F, t61950: F, t62360: F, t70640: F, t70655: F, t70660: F, t70665: F, t70703: F, t75836: F, t75912: F, t77621: F, t973: F, t974: F, t998: F) -> F {
    let t77687 = -t1041 * t4582 * t4583 * t77621 / F::cast_from(576.0_f64) - F::cast_from(7.0_f64) / F::cast_from(54.0_f64) * t973 * t974 * t42624 * t75836 + t973 * t974 * t998 * t75912 / F::cast_from(288.0_f64) + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t973 * t974 * t42309 * t75836 - t62360 / F::cast_from(2304.0_f64) - t10413 * t3071 * t5878 * t5685 / F::cast_from(768.0_f64) - t10403 * t3071 * t5681 * t5873 / F::cast_from(192.0_f64) + t61950 * t5909 / F::cast_from(384.0_f64) + t42388 * t3071 * t21391 * t1539 / F::cast_from(192.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t10876 * t4582 * t17712 * t5873 - t70640 / F::cast_from(288.0_f64) - t70655 / F::cast_from(27.0_f64) + t70660 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(486.0_f64) * t70665 - t3070 * t3071 * t4342 * t1616 * t5398 / F::cast_from(192.0_f64) - F::cast_from(5.0_f64) / F::cast_from(576.0_f64) * t3070 * t10408 * t21118 * t1616 + t70703 / F::cast_from(288.0_f64);
    t77687
}
