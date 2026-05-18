//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 972/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk972<F: Float>(t25374: F, t25927: F, t1081: F, t1530: F, t28: F, t4303: F, t1649: F, t776: F, t868: F, t1877: F, t1915: F, t22959: F, t23290: F, t25013: F, t2522: F, t25354: F, t25358: F, t25372: F, t25397: F, t25892: F, t25898: F, t25901: F, t25905: F, t25921: F, t6666: F, t6670: F, t6841: F, t6848: F, t7541: F, t7649: F, t7656: F) -> (F, F, F, F, F, F) {
    let t25928 = t25927 * t25374;
    let t25930 = t1081 * t1530;
    let t25934 = t28 * t4303;
    let t25938 = t1649 * t776;
    let t25945 = t1649 * t868;
    let t25949 = F::new(3.0) * t25013 * t25892 + F::new(3.0) / F::new(2.0) * t2522 * t6666 * t7649 - F::new(3.0) / F::new(2.0) * t22959 * t25898 + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t25901 + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t25905 + F::new(3.0) / F::new(2.0) * t2522 * t7541 * t6841 + t1877 * t25354 * t28 / F::new(2.0) - t1877 * t25358 * t6848 / F::new(2.0) + t1877 * t7541 * t1081 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t22959 * t25921 - t1877 * t23290 * t7656 / F::new(2.0) + t25372 * t25928 - t1877 * t6670 * t25930 / F::new(2.0) - t1877 * t6670 * t25934 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t25938 + t1877 * t6666 * t1649 / F::new(2.0) - t1877 * t6670 * t25945 / F::new(2.0) - t25397;
    (t25928, t25930, t25934, t25938, t25945, t25949)
}
