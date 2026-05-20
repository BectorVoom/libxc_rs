//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2337/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2337<F: Float>(t10481: F, t23508: F, t10469: F, t1603: F, t11058: F, t1625: F, t11045: F, t11064: F, t1058: F, t1060: F, t10857: F, t11028: F, t11034: F, t11040: F, t11046: F, t11048: F, t11049: F, t11061: F, t11067: F, t14608: F, t14622: F, t14654: F, t3200: F, t43480: F, t43536: F, t4669: F, t4674: F, t4677: F, t4685: F) -> (F, F, F, F) {
    let t47819 = t23508 * t10481;
    let t47840 = t1603 * t10469;
    let t47841 = t47840 * t11058;
    let t47844 = t1625 * t10481;
    let t47853 = t47840 * t11045;
    let t47857 = t47840 * t11064;
    let t47867 = t1058 * t1060 * t10857 * t1625 + t11046 * t11048 * t47844 - F::new(3.0) * t14622 * t3200 * t4677 + t11028 * t4669 + F::new(6.0) * t11034 * t14654 - F::new(3.0) * t11040 * t14608 + t11049 * t47853 + F::new(6.0) * t11061 * t47841 - F::new(6.0) * t11067 * t47857 + F::new(6.0) * t43480 * t4674 - F::new(3.0) * t43536 * t4685;
    (t47819, t47840, t47844, t47867)
}
