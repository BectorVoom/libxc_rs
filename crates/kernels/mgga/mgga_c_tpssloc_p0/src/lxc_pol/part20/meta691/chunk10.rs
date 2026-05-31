//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2633/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2633<F: Float>(t11712: F, t11913: F, t491: F, t11887: F, t52834: F, t11616: F, t11640: F, t11890: F, t11907: F, t11914: F, t15022: F, t15023: F, t15240: F, t15241: F, t15248: F, t15429: F, t1758: F, t3604: F, t3624: F, t44691: F, t45323: F, t5064: F, t5072: F, t5075: F, t5079: F, t5080: F) -> (F, F) {
    let t53545 = t11712 * t11913 * t491;
    let t53565 = t52834 * t11887;
    let t53590 = F::cast_from(3.0_f64) * t11914 * t15429 * t5072 + F::cast_from(3.0_f64) * t11914 * t15429 * t5075 - F::cast_from(3.0_f64) * t15022 * t3624 * t5072 - F::cast_from(3.0_f64) * t15240 * t3624 * t5079 + t11616 * t1758 + t11640 * t5064 - F::cast_from(6.0_f64) * t11890 * t53565 - F::cast_from(3.0_f64) * t11907 * t15023 + F::cast_from(3.0_f64) * t15241 * t3604 - F::cast_from(18.0_f64) * t15248 * t44691 - F::cast_from(3.0_f64) * t45323 * t5080;
    (t53545, t53590)
}
